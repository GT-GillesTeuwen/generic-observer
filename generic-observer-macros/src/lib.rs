extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Token};

struct NotifyArgs {
    fields: Vec<Ident>,
}

impl Parse for NotifyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();

        while !input.is_empty() {
            fields.push(input.parse::<Ident>()?);
            let _ = input.parse::<Option<Token![,]>>();
        }

        Ok(NotifyArgs { fields })
    }
}

#[proc_macro_attribute]
pub fn selective_notify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as NotifyArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let observer_fields: Vec<_> = args.fields.iter().map(|id| id).collect();

    let mut struct_fields = vec![];
    let mut set_methods = vec![];
    let mut new_params = vec![];
    let mut new_values = vec![];
    let mut register_observers = vec![];

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in fields.named.iter() {
                let field_name = &field.ident;
                let field_type = &field.ty;

                if field_name.is_some() && observer_fields.contains(&field_name.as_ref().unwrap()) {
                    let observer_map_field = Ident::new(&format!("{}_observers", field_name.as_ref().unwrap()), field.span());

                    struct_fields.push(quote! {
                        #field_name: #field_type,
                        #observer_map_field: Vec<Box<dyn Fn(&Self) + Send + Sync>>,
                    });

                    let set_method_name = Ident::new(&format!("set_{}", field_name.as_ref().unwrap()), field.span());
                    set_methods.push(quote! {
                        pub fn #set_method_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                            for observer in &self.#observer_map_field {
                                observer(&self);
                            }
                        }
                    });

                    new_values.push(quote! { #field_name, #observer_map_field: Vec::new() });

                    let register_observer_name = Ident::new(&format!("register_{}_observer", field_name.as_ref().unwrap()), field.span());
                    register_observers.push(quote! {
                        pub fn #register_observer_name(&mut self, observer: Box<dyn Fn(&Self) + Send + Sync>) {
                            self.#observer_map_field.push(observer);
                        }
                    });
                } else {
                    struct_fields.push(quote! {
                        #field_name: #field_type,
                    });
                    new_values.push(quote! { #field_name });
                }

                new_params.push(quote! { #field_name: #field_type });
            }
        }
    }

    let new_method = quote! {
        pub fn new(#(#new_params),*) -> Self {
            Self {
                #(#new_values),*
            }
        }
    };

    let expanded = quote! {
        struct #name #ty_generics #where_clause {
            #(#struct_fields)*
        }

        impl #impl_generics #name #ty_generics #where_clause {
            #new_method

            #(#set_methods)*

            #(#register_observers)*
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn notify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut struct_fields = vec![];
    let mut set_methods = vec![];
    let mut new_params = vec![];
    let mut new_values = vec![];
    let mut register_observers = vec![];
    
   
    let observer_map = Ident::new(&format!("observers"), input.span());

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in fields.named.iter() {
                let field_name = &field.ident;
                let field_type = &field.ty;

                if field_name.is_some() {

                    struct_fields.push(quote! {
                        #field_name: #field_type,
                    });

                    let set_method_name = Ident::new(&format!("set_{}", field_name.as_ref().unwrap()), field.span());
                    set_methods.push(quote! {
                        pub fn #set_method_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                            for observer in &self.#observer_map {
                                observer(&self);
                            }
                        }
                    });

                    new_values.push(quote! { #field_name });
 
                }
                new_params.push(quote! { #field_name: #field_type });
            }
        }    
        struct_fields.push(quote! {
            #observer_map: Vec<Box<dyn Fn(&Self) + Send + Sync>>,
        });
        register_observers.push(quote! {
            pub fn register_observer(&mut self, observer: Box<dyn Fn(&Self) + Send + Sync>) {
                self.#observer_map.push(observer);
            }
        });
        new_values.push(quote! { #observer_map:Vec::new() });
    }

    
    
    let new_method = quote! {
        pub fn new(#(#new_params),*) -> Self {
            Self {
                #(#new_values),*
            }
        }
    };

    let expanded = quote! {
        struct #name #ty_generics #where_clause {
            #(#struct_fields)*
        }

        impl #impl_generics #name #ty_generics #where_clause {
            #new_method

            #(#set_methods)*

            #(#register_observers)*
        }
    };

    TokenStream::from(expanded)
}