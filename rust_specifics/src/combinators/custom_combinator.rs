/// Custom trait to allow us to handle all request responses via alloy
/// Declare a trait which is parameterised to reflect the result type with generic T for OK type and E for error type
pub trait AlloyCallRequestValidation<T, E> {
    /// Declare function called on error where self is taken to reference the type and then a callback function of generic type F
    /// Where the function/ closure is callable once and will accept reference to the error, we do a reference so we don't transfer ownership, most of the time we want to log and alert
    fn on_error<F>(self, f: F) -> Self
    where
        F: FnOnce(&E);
}

impl<T, E> AlloyCallRequestValidation<T, E> for Result<T, E> {
    fn on_error<F>(self, f: F) -> Self
    where
        F: FnOnce(&E),
    {
        if let Err(ref e) = self {
            println!("In custom combinator");
            f(e);
        }
        self
    }
}

#[cfg(test)]
mod error_generate {

    pub(super) fn generate_result_for_test(to_error: bool) -> Result<i32, String> {
        if to_error {
            Err("This is an Error".to_string())
        } else {
            Ok(100)
        }
    }
}

#[cfg(test)]
mod custom_request_combinator {
    use crate::combinators::custom_combinator::{
        error_generate::generate_result_for_test, AlloyCallRequestValidation,
    };

    #[test]
    fn test_on_error_combinator() {
        let example_usage = generate_result_for_test(true).on_error(|err| {
            assert_eq!(err, "This is an Error");
        });

        println!("{:?}", example_usage);
    }
}
