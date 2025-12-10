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
    pub push_item: Option<syn::Expr>,
    pub no_accessors: Option<bool>,
    pub phantom: Option<syn::LitStr>,
}

impl Field {
    pub fn ident(&self) -> &syn::Ident {
        self.ident.as_ref().expect("ident should exist")
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
            quote! { #ident }
        }
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

    pub fn builder_accessors(&self) -> TokenStream {
        let ident = self.ident();
        let ty = self.inner_ty();

        let constructor_name = self.field_constructor_name();
        let constructor_fn = match self.setter.as_ref() {
            Some(path) => quote! { #path },
            None => quote! { crate::value::Value::new },
        };

        let getter = format_ident!("get_{ident}");
        let (getter_result_ty, after_get) = match InnerType::new(self.inner_ty()) {
            InnerType::CanCopy => (quote! { #ty }, quote! { .copied() }),
            InnerType::Vec(inner_ty) => (quote! { &[#inner_ty] }, quote! { .map(|v| v.as_ref()) }),
            InnerType::Other => (quote! { &#ty }, quote! {}),
        };

        let setter = format_ident!("set_{ident}");
        let setter_visibility = if self.private_setter.is_some_and(|v| v) {
            quote! {}
        } else {
            quote! { pub }
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
                        ..self
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
                #constructor_fn(value)
            }

            #accessors
        }
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

#[derive(Debug)]
enum InnerType {
    CanCopy,
    Vec(syn::Type),
    Other,
}

impl InnerType {
    fn new(ty: &syn::Type) -> Self {
        if is_bool(ty) || is_static_str_ref(ty) {
            return Self::CanCopy;
        }

        if let Some(inner_ty) = get_vec_inner_type(ty) {
            return Self::Vec(inner_ty);
        }

        Self::Other
    }
}

fn is_bool(ty: &syn::Type) -> bool {
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

fn get_vec_inner_type(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(last_segment) = type_path.path.segments.last() {
            if last_segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(generics) = &last_segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = generics.args.first() {
                        return Some(inner_ty.clone());
                    }
                }
            }
        }
    }
    None
}
