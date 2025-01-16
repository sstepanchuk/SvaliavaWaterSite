use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse::Parse, Token, LitStr, Type, Path};
use syn::parse::ParseStream;

struct ApiEndpointArgs {
    path: LitStr,
    method: Path,
    request: Type,
    response: Type,
    authorized: bool,
}

impl Parse for ApiEndpointArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        input.parse::<Token![,]>()?;
        
        let method = input.parse()?;
        input.parse::<Token![,]>()?;
        
        let request = input.parse()?;
        input.parse::<Token![,]>()?;
        
        let response = input.parse()?;
        
        let authorized = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let ident: Path = input.parse()?;
            ident.is_ident("authorized")
        } else {
            false
        };

        Ok(ApiEndpointArgs {
            path,
            method,
            request,
            response,
            authorized,
        })
    }
}

#[proc_macro_attribute]
pub fn api_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ApiEndpointArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let path = &args.path;
    let method = &args.method;
    let request = &args.request;
    let response = &args.response;
    
    let impl_endpoint = quote! {
        impl ApiEndpoint for #name {
            type Request = #request;
            type Response = #response;
            type Params = Self;

            const METHOD: &'static str = stringify!(#method);
            const PATH: &'static str = #path;
        }
    };

    let impl_authorized = if args.authorized {
        quote! {
            impl AuthorizedApiEndpoint for #name {}
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #input

        #impl_endpoint
        #impl_authorized
    };

    TokenStream::from(expanded)
}