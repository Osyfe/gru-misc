extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream
{
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let mut lenses = quote::quote!();
    if let syn::Data::Struct(data) = input.data
    {
        if let syn::Fields::Named(fields) = data.fields
        {
            for field in fields.named
            {
                if let Some(attribute) = field.ident
                {
                    let lens = quote::format_ident!("{}_{}_{}", "Lens", attribute, name);
                    let ty = field.ty;
                    lenses.extend(quote::quote!
                    (
                        pub struct #lens;

                        impl Lens<#name, #ty> for #lens
                        {
                            #[inline]
                            fn with<A, F: FnOnce(&#ty) -> A>(&self, data: &#name, f: F) -> A
                            {
                                f(&data.#attribute)
                            }

                            #[inline]
                            fn with_mut<A, F: FnOnce(&mut #ty) -> A>(&self, data: &mut #name, f: F) -> A
                            {
                                f(&mut data.#attribute)
                            }
                        }

                        impl #name
                        {
                            pub const #attribute: #lens = #lens;
                        }
                    ));
                } else { panic!("Only named fields allowed."); }
            }
        } else { panic!("Only named fields allowed."); }
    } else { panic!("Only structs allowed."); }
    //println!("{}", lenses);
    TokenStream::from(lenses)
}
