use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

mod types;
mod utils;

// The main struct we get from parsing the attributes
// Ref: https://github.com/TedDriggs/darling?tab=readme-ov-file#shape-validation
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct Target {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), types::Field>,
    validate: Option<darling::Result<syn::Path>>,
}

impl Target {
    fn validate(&self) -> Option<syn::Path> {
        match self.validate.clone().transpose() {
            Ok(v) => v,
            Err(err) => {
                abort! {
                    err.span(), "Invalid attribute #[builder(validate = ...)]";
                    note = "Invalid argument for `validate` attribute. Only paths are allowed.";
                    help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`";
                }
            }
        }
    }

    fn fields(&self) -> Vec<types::Field> {
        self.data.clone().take_struct().unwrap().fields
    }

    fn has_multiple_fields(&self) -> bool {
        self.fields().len() > 1
    }

    fn into_token_strem(self) -> TokenStream {
        if self.data.is_enum() {
            panic!("enum is not supported")
        }

        let ident = &self.ident;
        let (imp, ty, whr) = self.generics.split_for_impl();

        let builder = format_ident!("{ident}Builder");
        let doc_builder_method = format!("constract [`{builder}`] object.");
        let doc_builder = format!("Builder for [`{ident}`] object.");
        let doc_build_method = format!("build [`{ident}`] object.");

        let validate_across_fields = match self.validate() {
            Some(path) => quote! { #path(self) },
            None => quote! { vec![] },
        };
        // fields of builder object. they doesn't includes phantom type.
        let builder_fields = self
            .fields()
            .iter()
            .filter_map(|f| {
                if f.is_phantom() {
                    None
                } else {
                    Some(f.clone())
                }
            })
            .collect::<Vec<types::Field>>();

        let merge_fn = format_ident!("merge_{}", builder_fields.len() + 1);

        let builder_field_names = builder_fields.iter().map(types::Field::field_name);
        let builder_field_names_0 = builder_field_names.clone();
        let builder_field_names_1 = builder_field_names.clone();
        let builder_field_names_2 = builder_field_names.clone();
        let builder_field_names_3 = builder_field_names.clone();
        let builder_field_names_4 = builder_field_names.clone();

        let has_multi_fields = self.has_multiple_fields();
        let accessors = builder_fields
            .iter()
            .map(|f| f.builder_accessors(has_multi_fields));

        let fields = self.fields();
        let build_target_fields = fields.iter().map(types::Field::build_target_field);
        let init_builder_fields = fields.iter().map(types::Field::init_builder_field);
        let expand_builder_fields = if fields.iter().any(types::Field::is_phantom) {
            quote! { #(#builder_field_names_0),* , .. }
        } else {
            quote! { #(#builder_field_names_0),* }
        };
        let default_fields = fields.iter().map(types::Field::default_field);

        quote! {
            impl #imp #ident #ty #whr {
                #[doc = #doc_builder_method]
                pub fn builder() -> #builder #ty {
                    #builder::default()
                }
            }

            #[doc = #doc_builder]
            #[derive(Debug)]
            pub struct #builder #ty #whr {
                #(#init_builder_fields,)*
            }

            impl #imp #builder #ty #whr {
                #(#accessors)*

                fn validate_across_fields(&self) -> Vec<crate::errors::ValidationErrorKind> {
                    #validate_across_fields
                }

                #[doc = #doc_build_method]
                pub fn build(self) -> ::std::result::Result<#ident #ty, crate::errors::ValidationErrors> {
                    let v0 = crate::value::Value::<()> {
                        inner: None,
                        errors: self.validate_across_fields(),
                    };

                    let Self { #expand_builder_fields } = self;
                    crate::value::#merge_fn(v0, #(#builder_field_names_1),*)
                        .map(|(_, #(#builder_field_names_2),*)| #ident {
                            #(#build_target_fields),*
                        })
                        .map_err(|(e_0, #(#builder_field_names_3),*)| {
                            let errors = [
                                #(crate::errors::ValidationError::new_single_field(stringify!(#builder_field_names_4), #builder_field_names_4)),*
                            ]
                            .into_iter()
                            .filter_map(|v| v)
                            .chain([crate::errors::ValidationError::new_across_fields(e_0)]
                                .into_iter()
                                .filter_map(|v| v)
                            )
                            .collect();

                            crate::errors::ValidationErrors {
                                object: ::std::borrow::Cow::Borrowed(stringify!(#ident)),
                                errors,
                            }
                        })
                }
            }

            impl #imp Default for #builder #ty #whr {
                fn default() -> Self {
                    Self {
                        #(#default_fields,)*
                    }
                }
            }
        }
        .into()
    }
}

#[proc_macro_error]
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Target::from_derive_input(&input)
        .map(Target::into_token_strem)
        .unwrap_or_else(|e| e.write_errors().into())
}
