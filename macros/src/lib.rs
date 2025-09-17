use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse_macro_input, Expr, FnArg, Ident, ItemFn, Lit, Local, Pat, Stmt, Type};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let docstring = function.attrs.iter().find_map(|attribute| {
        if let syn::Meta::NameValue(nv) = &attribute.meta
            && nv.path.is_ident("doc")
            && let Expr::Lit(value) = &nv.value
            && let Lit::Str(s) = &value.lit
        {
            Some(s.token().to_string())
        } else {
            None
        }
    });

    let mut args = function.sig.inputs.clone().into_iter();

    // ensure the only argument is a PrefixContext, changing the first argument if necessary
    let context_ident: Ident = if let Some(arg) = function.sig.inputs.first()
        && let FnArg::Typed(pat_type) = arg
        && let Type::Path(type_path) = pat_type.ty.as_ref()
        && let Some(last_segment) = type_path.path.segments.last()
        && last_segment.ident == "PrefixContext"
        && let Pat::Ident(pat_ident) = pat_type.pat.as_ref()
        && pat_ident.ident != "_"
    {
        args.next();
        pat_ident.ident.clone()
    } else {
        let arg =
            syn::parse(quote! {__context: framework::structs::prefix::PrefixContext<'_>}.into())
                .unwrap();

        function.sig.inputs.clear();
        function.sig.inputs.insert(0, arg);
        syn::parse::<Ident>(quote! {__context}.into()).unwrap()
    };

    for (i, arg) in args.enumerate() {
        match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                    todo!()
                };
                let arg_ident = pat_ident.ident.clone();

                let arg_ident_quoted = arg_ident.to_string();

                let arg_type = pat_type.ty;
                let statement: Stmt = syn::parse(quote! {
                    let #arg_ident: #arg_type = framework::handlers::message_binder::bind_message(&#context_ident.msg.content, #i, #arg_ident_quoted);
                }.into()).unwrap();
                
                println!("statement : {}", statement.to_token_stream());

                function
                    .block
                    .stmts
                    .insert(0, statement);
            }
            FnArg::Receiver(_) => {
                panic!("i have no clue how you got a receiver here")
            }
        }
    }

    println!("command name : {}", function.sig.ident);
    println!("command description : {docstring:?}");
    println!("context identifier : {context_ident:?}");

    function.to_token_stream().into()
}
