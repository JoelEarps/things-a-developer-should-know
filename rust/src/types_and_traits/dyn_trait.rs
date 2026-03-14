// Keywords: dyn trait, dynamic dispatch, vtable, polymorphism, fat pointer, monomorphization
/// A `dyn<trait>` rusts way of doing dynamic dispatch, it allows you to work with values of different concrete types without knowing the exact type at compile time
/// It is essentially a fat pointer that contains a pointer to the actual value (data) and a function to the vtable (virtual method table).
/// A vtable is a look uo of function pointers generated at compile time. For each concrete type that implements a trait.
/// The vtable contains a drop in place - function pointer for the destructor.
/// size - size of the concrete type
/// align - alignment of the concrete type
/// method pointers - function pointers for each method in the trait
/// A vtable is essentially a fixed point array of function pointers - at compile time for each type it knows which trait you are calling methods on, the layout of the vtable and therefore the offset to the function pointer. This makes it O(1) look up time for an indirect call of a function.
/// At instruction level for generics it is a direct call for generics, will becomes a CALL instruction in assembly.
/// At instruction level for dyn dispatch, it is an indirect call, will becomes a CALLQ instruction in assembly. This is because the vtable is a pointer to a function pointer, and the function pointer is a pointer to the actual function.
/// Load the vtable pointer from the trait object
/// Load the data pointer to pass as self
/// Index into the vtable at the correct offset for the method
/// Indirect call through that function pointer
/// Why are generics preferred over `dyn<trait>`?
/// Generics are implemented at compile time using monomorphization, this means that the compiler generates a specific version of the code for each concrete type that is used.
/// There is therefore no runtime overhead for generic code, it is often zero cost and therefore determintsic at run time at the cost of a slower compile time an dlarger binary
/// For dyn dispatch, the application performs an indirect call at runtime, which is slower than a direct call. It is smaller in terms of binary size and faster for compilation.
///
/// So when should we use `dyn<trait>`?
/// Binary size matters but performance is not a concern.
/// When we only know types/ modes of operation at run time.
/// Recursive types - when you have a type that contains itself, you cannot use generics.
/// API boundaries - hide implementation details and provide a stable interface.
///
/// You cannot use them
/// An example of when i used dyn dispatch in the real world: when I wanted to create run time configurable components in a modular architecture and wanted to be able to changed these at run time without recompiling the code and having to start the application in a different mode.
trait DynTraitExample {
    fn dyn_method(&self) -> String;
}

pub struct DynTraitImpl {}

pub struct DynTraitImpl2 {}

impl DynTraitExample for DynTraitImpl {
    fn dyn_method(&self) -> String {
        "DynTraitImpl".to_string()
    }
}

impl DynTraitExample for DynTraitImpl2 {
    fn dyn_method(&self) -> String {
        "DynTraitImpl2".to_string()
    }
}

/// Why do we need to put this in a Box?
///
/// `dyn Trait` is an unsized type (DST - Dynamically Sized Type). The compiler doesn't know
/// the size at compile time because the concrete type is only known at runtime.
///
/// Rust requires all function parameters to have a known size at compile time so it knows
/// how much stack space to allocate. Since `dyn DynTraitExample` could be any type that
/// implements the trait (each with different sizes), we need indirection.
///
/// Box<dyn Trait> solves this by:
/// 1. Allocating the actual data on the heap (variable size)
/// 2. Storing a fat pointer on the stack (fixed 16 bytes: data_ptr + vtable_ptr)
///
/// Alternatives to Box:
/// - `&dyn Trait` - borrowed reference (no heap allocation, doesn't take ownership)
/// - `&mut dyn Trait` - mutable borrowed reference
/// - `Rc<dyn Trait>` - reference counted (multiple owners, single-threaded)
/// - `Arc<dyn Trait>` - atomic reference counted (multiple owners, multi-threaded)
fn dyn_trait_demo(print_object: Box<dyn DynTraitExample>) {
    println!("{}", print_object.dyn_method());
}

fn dyn_any_demo() {}

fn dyn_fn_demo() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyn_trait_impl1_returns_correct_string() {
        let impl1: Box<dyn DynTraitExample> = Box::new(DynTraitImpl {});
        assert_eq!(impl1.dyn_method(), "DynTraitImpl");
    }

    #[test]
    fn test_dyn_trait_impl2_returns_correct_string() {
        let impl2: Box<dyn DynTraitExample> = Box::new(DynTraitImpl2 {});
        assert_eq!(impl2.dyn_method(), "DynTraitImpl2");
    }

    #[test]
    fn test_heterogeneous_collection_with_dyn_trait() {
        // This demonstrates the power of dyn traits - storing different concrete types in the same collection
        let implementations: Vec<Box<dyn DynTraitExample>> = vec![
            Box::new(DynTraitImpl {}),
            Box::new(DynTraitImpl2 {}),
            Box::new(DynTraitImpl {}),
        ];

        let results: Vec<String> = implementations
            .iter()
            .map(|item| item.dyn_method())
            .collect();

        assert_eq!(
            results,
            vec!["DynTraitImpl", "DynTraitImpl2", "DynTraitImpl"]
        );
    }

    #[test]
    fn test_dyn_trait_demo_function() {
        // Test that we can pass different implementations to the same function
        // The vtable lookup happens at runtime to find the correct dyn_method implementation
        dyn_trait_demo(Box::new(DynTraitImpl {}));
        dyn_trait_demo(Box::new(DynTraitImpl2 {}));
    }
}
