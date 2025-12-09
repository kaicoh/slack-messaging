use darling::FromField;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(builder), forward_attrs)]
pub struct Field {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub setter: Option<syn::Expr>,
    pub private_setter: Option<bool>,
}

impl Field {
    pub fn ident(&self) -> &syn::Ident {
        self.ident.as_ref().expect("ident should exist")
    }

    pub fn inner_ty(&self) -> &syn::Type {
        get_option_inner_type(&self.ty).expect("field should be Option type")
    }

    pub fn field_name(&self) -> TokenStream {
        let ident = self.ident();
        quote! { #ident }
    }

    pub fn field_constructor_name(&self) -> TokenStream {
        let ident = self.ident();
        let constructor_name = format_ident!("new_{ident}");
        quote! { #constructor_name }
    }

    pub fn default_field(&self) -> TokenStream {
        let ident = self.ident();
        let constructor = self.field_constructor_name();
        quote! { #ident: Self::#constructor(::std::option::Option::None) }
    }

    pub fn builder_field(&self) -> TokenStream {
        let ident = self.ident();
        let ty = self.inner_ty();
        quote! { #ident: crate::value::Value<#ty> }
    }

    pub fn builder_accessors(&self) -> TokenStream {
        let ident = self.ident();
        let ty = self.inner_ty();

        let constructor_name = self.field_constructor_name();
        let constructor_fn = match self.setter.as_ref() {
            Some(path) => quote! { #path },
            None => quote! { crate::value::Value::new },
        };

        let getter = format_ident!("get_{ident}");
        let (getter_result_ty, after_get) = if self.inner_ty_can_copy() {
            (quote! { #ty }, quote! { .copied() })
        } else {
            (quote! { &#ty }, quote! {})
        };

        let setter = format_ident!("set_{ident}");
        let setter_visibility = if self.private_setter.is_some_and(|v| v) {
            quote! {}
        } else {
            quote! { pub }
        };

        let doc_getter = format!("get {ident} field value.");
        let doc_setter = format!("set {ident} field value.");

        quote! {
            fn #constructor_name(value: ::std::option::Option<#ty>) -> crate::value::Value<#ty> {
                #constructor_fn(value)
            }

            #[doc = #doc_getter]
            pub fn #getter(&self) -> ::std::option::Option<#getter_result_ty> {
                self.#ident.inner_ref()#after_get
            }

            #[doc = #doc_setter]
            #setter_visibility fn #setter(self, value: ::std::option::Option<impl Into<#ty>>) -> Self {
                Self {
                    #ident: Self::#constructor_name(value.map(|v| v.into())),
                    ..self
                }
            }

            #[doc = #doc_setter]
            #setter_visibility fn #ident(self, value: impl Into<#ty>) -> Self {
                self.#setter(Some(value))
            }
        }
    }

    fn inner_ty_can_copy(&self) -> bool {
        let inner_ty = self.inner_ty();
        is_bool_type(inner_ty) || is_static_str_ref(inner_ty)
    }
}

fn get_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(type_path) = ty else {
        return None; // Not a path type
    };

    // Check for qualified self (e.g., <T as Trait>::Assoc)
    if type_path.qself.is_some() {
        return None;
    }

    let path = &type_path.path;

    // Ensure the path is not empty and ends with "Option"
    let last_segment = path.segments.last()?;
    if last_segment.ident != "Option" {
        return None; // Not an Option type
    }

    // Check for correct path structure (e.g., Option<T> or std::option::Option<T>)
    // This part ensures we're not matching a type named "Option" in a different module
    if !(path.segments.len() == 1
        || (path.segments.len() == 3
            && (path.segments[0].ident == "core" || path.segments[0].ident == "std")
            && path.segments[1].ident == "option"))
    {
        return None;
    }

    // Get the generic arguments
    let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None; // No generic arguments
    };

    // Ensure there's exactly one generic argument (the inner type)
    if generics.args.len() != 1 {
        return None;
    }

    // Extract the inner type
    let syn::GenericArgument::Type(inner_type) = &generics.args[0] else {
        return None; // Not a type argument
    };

    Some(inner_type)
}

fn is_bool_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        // Check if the path has a single segment
        if let Some(segment) = path.segments.first() {
            // Check if the segment's identifier is "bool"
            if segment.ident == "bool" {
                // Ensure there are no generic arguments or other path components
                return path.segments.len() == 1 && segment.arguments.is_empty();
            }
        }
    }
    false
}

fn is_static_str_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(syn::TypeReference {
        elem,
        lifetime,
        mutability: None, // &'static str is immutable
        ..
    }) = ty
    {
        // Check for the 'static lifetime
        if let Some(lt) = lifetime {
            if lt.ident.to_string() != "static" {
                return false;
            }
        } else {
            // If no explicit lifetime is given, it might be inferred as 'static in some contexts,
            // but for explicit &'static str, the lifetime will typically be present.
            // For robustness, you might need to consider inferred static lifetimes depending on your use case.
            return false;
        }

        // Check if the inner type is `str`
        if let syn::Type::Path(syn::TypePath { path, .. }) = &**elem {
            if path.segments.len() == 1 && path.segments[0].ident == "str" {
                // Ensure there are no generic arguments on `str`
                if let syn::PathArguments::None = path.segments[0].arguments {
                    return true;
                }
            }
        }
    }
    false
}
