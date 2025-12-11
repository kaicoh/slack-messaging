pub fn get_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
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
pub enum InnerType {
    CanCopy,
    Vec(syn::Type),
    Other,
}

impl InnerType {
    pub fn new(ty: &syn::Type) -> Self {
        if is_bool(ty) || is_static_str_ref(ty) {
            return Self::CanCopy;
        }

        if let Some(inner_ty) = get_vec_inner_type(ty) {
            return Self::Vec(inner_ty);
        }

        Self::Other
    }
}

pub fn is_bool(ty: &syn::Type) -> bool {
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

pub fn is_static_str_ref(ty: &syn::Type) -> bool {
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

pub fn get_vec_inner_type(ty: &syn::Type) -> Option<syn::Type> {
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
