use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr, LitInt, Attribute};
use quote::quote;

#[proc_macro_derive(ParseRe, attributes(re))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_parsere(input) {
        Ok(ts) => ts,
        Err(e) =>  e.to_compile_error().into(),
    }
}

fn derive_parsere(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let call_site = Span::call_site();

    let struct_name = &input.ident;

    let type_err_msg = format!(
        "cannot derive ParseRe for {}: named structs only",
        struct_name.to_string(),
    );
    let fields = if let Data::Struct(ref data) = input.data {
        if let Fields::Named(ref fields) = data.fields {
            fields
        } else {
            return Err(syn::Error::new(call_site, type_err_msg))
        }
    } else {
        return Err(syn::Error::new(call_site, type_err_msg));
    };

    let re_string = get_re(&input.attrs)?;

    let parses = fields.named
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            let name = &f.ident;
            let i = LitInt::new(&(i + 1).to_string(), call_site);
            Some(quote! {
                #name: captures
                    .get(#i)
                    .ok_or_else(|| {
                        Box::new(parsere::Error::NoGroup {
                            re: #re_string,
                            txt: txt.to_string(),
                            index: #i,
                        })
                    })?
                    .as_str()
                    .parse()?,
            })
        });

    let parsre = quote! {
        impl #struct_name {
            fn parse_re(txt: &str) ->
                std::result::Result<Self, std::boxed::Box<dyn std::error::Error>>
            {
                use regex::Regex;
                use once_cell::unsync::Lazy;
                let re = Lazy::new(|| Regex::new(#re_string));
                let captures = match Lazy::force(&re) {
                    Ok(re) => re.captures(txt).ok_or_else(|| {
                        Box::new(parsere::Error::Mismatch {
                            re: #re_string,
                            txt: txt.to_string(),
                        })
                    })?,
                    Err(e) => return Err(Box::new(e.clone())),
                };
                Ok(Self {
                    #( #parses )*
                })
            }
        }
    };

    Ok(TokenStream::from(parsre))
}

fn get_re(attrs: &[Attribute]) -> Result<LitStr, syn::Error> {
    use syn::{AttrStyle, Meta, NestedMeta, Lit};
    let call_site = Span::call_site();

    let attr_err_msg = 
        "expected `re(\"...\")`"
        .to_string();
    let attr_err = Err(syn::Error::new(call_site, attr_err_msg));

    if attrs.len() != 1 {
        return attr_err;
    }

    let a = &attrs[0];

    if let AttrStyle::Outer = a.style {
        ()
    } else {
        return attr_err;
    }

    if let Meta::List(ref m) = a.parse_meta()? {
        if m.path.segments.len() != 1 {
            return attr_err;
        }
        if m.path.segments[0].ident.to_string() != "re" {
            panic!("internal error: misparse");
        }
        if m.nested.len() != 1 {
            return attr_err;
        }
        if let Some(ref m) = m.nested.first() {
            if let NestedMeta::Lit(Lit::Str(lit)) = m {
                return Ok(lit.clone());
            }
        }
    }
    attr_err
}
