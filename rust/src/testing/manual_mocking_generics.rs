struct ProdExchangeClient {
    test: String,
}

#[cfg(test)]
struct MockExchangeClient {
    test: String,
}

trait DefaultTrait {
    fn new(input: &str) -> Self;
}

trait TraitsToBeMocked {
    fn get_test(&self) -> String;
}

impl DefaultTrait for ProdExchangeClient {
    fn new(input: &str) -> Self {
        ProdExchangeClient {
            test: input.to_string(),
        }
    }
}

#[cfg(test)]
impl DefaultTrait for MockExchangeClient {
    fn new(input: &str) -> Self {
        println!(
            "MockExchangeClient new called with input: {}, this will be overrided to create a mock",
            input
        );
        MockExchangeClient {
            test: "mocked_client".to_string(),
        }
    }
}

impl TraitsToBeMocked for ProdExchangeClient {
    fn get_test(&self) -> String {
        self.test.clone()
    }
}

#[cfg(test)]
impl TraitsToBeMocked for MockExchangeClient {
    fn get_test(&self) -> String {
        "mocked_data".to_string()
    }
}

struct ServiceWrapper<T: TraitsToBeMocked + DefaultTrait> {
    client: T,
}

impl<T: TraitsToBeMocked + DefaultTrait> ServiceWrapper<T> {
    fn generic_new(value: &str) -> ServiceWrapper<T> {
        Self {
            client: T::new(value),
        }
    }

    fn get_test(&self) -> String {
        self.client.get_test()
    }
}

// Mockall crate
// Mock the exchange client send order to see how we can handle different returns, can we pass this into the overall system? And then test the response in
// Ah maybe here we can do it in the dex api trait?

#[cfg(test)]
mod mocking_tests {
    use super::*;

    #[test]
    fn manual_mocks_with_generics() {
        let service_wrapper_mock =
            ServiceWrapper::<MockExchangeClient>::generic_new("None Mock input");
        let service_wrapper_prod = ServiceWrapper::<ProdExchangeClient>::generic_new("Prod input");
        let mocked = service_wrapper_mock.get_test();
        let prod = service_wrapper_prod.get_test();
        assert_eq!(prod, "Prod input");
        assert_eq!(mocked, "mocked_data");
    }
}
