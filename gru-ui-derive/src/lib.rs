extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream
{
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let mut lenses = quote::quote!();
    if let syn::Data::Struct(data) = input.data
    {
        if let syn::Fields::Named(fields) = data.fields
        {
            for field in fields.named
            {
                if let Some(attribute) = field.ident
                {
                    let lens = quote::format_ident!("{}_{}_{}", "Lens", name, attribute);
                    let ty = field.ty;
                    lenses.extend(quote::quote!
                    (
                        #[allow(non_camel_case_types)]
                        #[derive(Clone, Copy)]
                        pub struct #lens;

                        impl#generics Lens<#name#generics, #ty> for #lens
                        {
                            #[inline]
                            fn with<A, F: FnOnce(&#ty) -> A>(&self, data: &#name#generics, f: F) -> A
                            {
                                f(&data.#attribute)
                            }

                            #[inline]
                            fn with_mut<A, F: FnOnce(&mut #ty) -> A>(&mut self, data: &mut #name#generics, f: F) -> A
                            {
                                f(&mut data.#attribute)
                            }
                        }

                        impl#generics #name#generics
                        {
                            #[allow(non_upper_case_globals)]
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
