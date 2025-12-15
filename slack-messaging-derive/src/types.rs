use super::utils::*;

use darling::FromField;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(builder), forward_attrs)]
pub struct Field {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub private_setter: Option<bool>,
    pub push_item: Option<syn::Expr>,
    pub no_accessors: Option<bool>,
    pub phantom: Option<syn::LitStr>,
    #[darling(default)]
    pub validate: Vec<syn::LitStr>,
}

impl Field {
    pub fn ident(&self) -> &syn::Ident {
        self.ident.as_ref().expect("ident should exist")
    }

    pub fn ident_str(&self) -> String {
        self.ident().to_string()
    }

    pub fn inner_ty(&self) -> &syn::Type {
        get_option_inner_type(&self.ty).expect("field should be Option type")
    }

    pub fn is_phantom(&self) -> bool {
        self.phantom.is_some()
    }

    pub fn build_target_field(&self) -> TokenStream {
        let ident = self.ident();

        if self.is_phantom() {
            quote! { #ident: ::std::marker::PhantomData }
        } else {
            quote! { #ident: #ident.inner }
        }
    }

    pub fn field_name(&self) -> TokenStream {
        let ident = self.ident();
        quote! { #ident }
    }

    pub fn field_constructor_name(&self) -> TokenStream {
        let constructor_name = format_ident!("new_{}", strip_raw_ident(&self.ident_str()));
        quote! { #constructor_name }
    }

    pub fn default_field(&self) -> TokenStream {
        let ident = self.ident();

        if self.is_phantom() {
            quote! { #ident: ::std::marker::PhantomData }
        } else {
            let constructor = self.field_constructor_name();
            quote! { #ident: Self::#constructor(::std::option::Option::None) }
        }
    }

    pub fn init_builder_field(&self) -> TokenStream {
        let ident = self.ident();

        if let Some(lit) = self.phantom.as_ref() {
            let param: TokenStream = lit.value().parse().unwrap();
            quote! { #ident: ::std::marker::PhantomData<#param> }
        } else {
            let ty = self.inner_ty();
            quote! { #ident: crate::value::Value<#ty> }
        }
    }

    pub fn builder_accessors(&self, has_multi_fields: bool) -> TokenStream {
        let ident = self.ident();
        let ty = self.inner_ty();

        let constructor_name = self.field_constructor_name();
        let constructor_fn = if !self.validate.is_empty() {
            let exprs: Vec<TokenStream> = self
                .validate
                .clone()
                .iter()
                .map(|lit| lit.value().parse().unwrap())
                .collect();
            quote! {
                pipe! { crate::value::Value::new(value) => #(#exprs)|* }
            }
        } else {
            quote! { crate::value::Value::new(value) }
        };

        let getter = format_ident!("get_{}", strip_raw_ident(&self.ident_str()));
        let (getter_result_ty, after_get) = match InnerType::new(self.inner_ty()) {
            InnerType::CanCopy => (quote! { #ty }, quote! { .copied() }),
            InnerType::Vec(inner_ty) => (quote! { &[#inner_ty] }, quote! { .map(|v| v.as_ref()) }),
            InnerType::Other => (quote! { &#ty }, quote! {}),
        };

        let setter = format_ident!("set_{}", strip_raw_ident(&self.ident_str()));
        let setter_visibility = if self.private_setter.is_some_and(|v| v) {
            quote! {}
        } else {
            quote! { pub }
        };
        let after_set = if has_multi_fields {
            quote! { ..self }
        } else {
            quote! {}
        };

        let push_item = match (self.push_item.as_ref(), InnerType::new(self.inner_ty())) {
            (Some(expr), InnerType::Vec(inner_ty)) => {
                let doc = format!("push list element to {ident} field.");

                quote! {
                    #[doc = #doc]
                    pub fn #expr(mut self, value: impl Into<#inner_ty>) -> Self {
                        let mut list = self.#ident.take_inner().unwrap_or_default();
                        list.push(value.into());
                        self.#ident(list)
                    }
                }
            }
            _ => quote! {},
        };

        let doc_getter = format!("get {ident} field value.");
        let doc_setter = format!("set {ident} field value.");

        let accessors = if self.no_accessors.is_some_and(|v| v) {
            quote! {}
        } else {
            quote! {
                #[doc = #doc_getter]
                pub fn #getter(&self) -> ::std::option::Option<#getter_result_ty> {
                    self.#ident.inner_ref()#after_get
                }

                #[doc = #doc_setter]
                #setter_visibility fn #setter(self, value: ::std::option::Option<impl Into<#ty>>) -> Self {
                    Self {
                        #ident: Self::#constructor_name(value.map(|v| v.into())),
                        #after_set
                    }
                }

                #[doc = #doc_setter]
                #setter_visibility fn #ident(self, value: impl Into<#ty>) -> Self {
                    self.#setter(Some(value))
                }

                #push_item
            }
        };

        quote! {
            fn #constructor_name(value: ::std::option::Option<#ty>) -> crate::value::Value<#ty> {
                #constructor_fn
            }

            #accessors
        }
    }
}
