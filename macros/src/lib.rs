use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    parse_macro_input, Expr, FnArg, Ident, ItemFn, Lit, Pat, Type
};

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

    // ensure first argument is a PrefixContext, changing the first argument if necessary
    let _context_ident: Ident = if let Some(arg) = function.sig.inputs.first()
        && let FnArg::Typed(pat_type) = arg
        && let Type::Path(type_path) = pat_type.ty.as_ref()
        && type_path.path.is_ident("PrefixContext")
    {
        match pat_type.pat.as_ref() {
            Pat::Ident(pat_ident) => {pat_ident.ident.clone()}
            _ => { panic!("No identifier found") }
        } 
    } else {
        let arg = syn::parse(quote! {__context: framework::structs::prefix::PrefixContext<'_>}.into())
            .unwrap();
        function.sig.inputs.insert(0, arg);
        syn::parse::<Ident>(quote! {__context}.into()).unwrap()
    };

    let arguments = &function.sig.inputs.iter().collect::<Vec<_>>();

    println!("command name : {}", function.sig.ident);
    println!("command description : {docstring:?}");
    println!("function args : {arguments:#?}");

    function.to_token_stream().into()
}
