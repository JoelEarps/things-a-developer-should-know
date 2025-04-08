/* Chapter 1: Manual Mocks 
In this instance we have two structs, one for production and one for testing.
The production struct is used in the main code, while the testing struct is used in the tests.
The testing struct has the same interface as the production struct, but it returns different data.
This allows us to test the code without having to rely on the production struct.
However this requires us to write a lot of boilerplate code.
*/

struct ProdExchangeClient {
    test: String,
}

struct MockExchangeClient {
    test: String,
}

trait DefaultTrait {
    fn new() -> Self;
}

trait TraitsToBeMocked {
    fn get_test(&self) -> String;
}

impl DefaultTrait for ProdExchangeClient {
    fn new() -> Self {
        ProdExchangeClient {
            test: "test".to_string(),
        }
    }
}

impl DefaultTrait for MockExchangeClient {
    fn new() -> Self {
        MockExchangeClient {
            test: "test".to_string(),
        }
    }
}

impl TraitsToBeMocked for ProdExchangeClient {
    fn get_test(&self) -> String {
        self.test.clone()
    }
}



impl TraitsToBeMocked for MockExchangeClient {
    fn get_test(&self) -> String {
        "mocked_data".to_string()
    }
}

/* Chapter 2: Trait Bounds and Mocks 
Therefore a solution to this is trait bounds
 */

#[cfg(test)]
mod mocking_tests {
    use super::*;
    #[test]
    fn manual_trait_mocks(){
        let struct_under_test = ProdExchangeClient::new();
        let mock_struct = MockExchangeClient::new();
        assert_eq!("test".to_string(), struct_under_test.get_test());
        assert_eq!("mocked_data".to_string(), mock_struct.get_test());
    }
}