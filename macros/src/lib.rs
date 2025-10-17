use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, FnArg, Ident, ItemFn, Lit, Pat, Stmt, Type, parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let docstring = function
        .attrs
        .iter()
        .find_map(|attribute| {
            if let syn::Meta::NameValue(nv) = &attribute.meta
                && nv.path.is_ident("doc")
                && let Expr::Lit(value) = &nv.value
                && let Lit::Str(s) = &value.lit
            {
                Some(s.token().to_string())
            } else {
                None
            }
        })
        .unwrap_or(String::new());

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
            syn::parse(quote! {__context: ::framework::structs::context::PrefixContext<'_>}.into())
                .unwrap();

        function.sig.inputs.clear();
        function.sig.inputs.insert(0, arg);
        syn::parse::<Ident>(quote! {__context}.into()).unwrap()
    };

    let arg_strings = args.clone().map(|arg: FnArg| match arg {
        FnArg::Receiver(_) => todo!(),
        FnArg::Typed(pat_type) => {
            let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {
                todo!()
            };
            pat_ident.ident.to_string()
        }
    });
    let args_string = quote! {[ #(#arg_strings),* ]};

    let mut statements: Vec<Stmt> = Vec::new();

    let arg_state = syn::parse(quote! {
        let mut __arg_state = ::framework::structs::arg_state::ArgState::from_message(&#context_ident.msg.content, &#args_string);
    }.into()).unwrap();
    
    statements.push(arg_state);

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
                    let #arg_ident = match __arg_state.bind::<#arg_type, _>(#arg_ident_quoted, #i) {
                        Ok(__arg_ok) => __arg_ok,
                        Err(__arg_err) => return ::framework::structs::command_result::CommandResult::Err(*__arg_err),
                    };
                }.into()).unwrap();

                statements.push(statement);
            }
            FnArg::Receiver(receiver) => {
                return syn::Error::new(receiver.span(), "self is not supported")
                    .into_compile_error()
                    .into();
            }
        }
    }

    statements.push(syn::parse(quote! {
        if let Err(e) = __arg_state.finish() {
            return ::framework::structs::command_result::CommandResult::Err(*e);
        }
    }.into()).unwrap());

    statements.extend(function.block.stmts);
    function.block.stmts = statements;

    let mut res_stream = function.to_token_stream();

    let function_ident = function.sig.ident;
    let function_name = function_ident.to_string();

    res_stream.extend(quote! {
        ::framework::inventory::submit! {
            ::framework::structs::command::PrefixCommand {
                name: #function_name,
                description: #docstring,
                callback: |x| {
                    Box::pin(async move {
                        let res = #function_ident(x).await;
                        res.into()
                    })
                },
            }
        }
    });

    res_stream.into()
}
