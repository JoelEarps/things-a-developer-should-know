/* Chapter 3: Mockall Crate
To remove even more boilerplate, we can use the mockall crate.
This crate allows us to create mocks with less code and more flexibility.
You use the mockall attribute to create a mock.
So lets add soem complexity for where this becomes more useful.
We are making a request now which is made in the TraitsToBeMocked trait.
In the mock trait we can override the get_response method to return a mock response.
This allows us to test the code without having to rely on the production struct.
Now for this very simple example this is not really needed and the implementation is very basic, but imagine we have a lot of calls,
or code that is triggered based on the response of the request. Now we can test the code without having to rely on those responses occuring.
*/

use mockall::automock;

struct ProdExchangeClient {
    test: String,
}

impl ProdExchangeClient {
    fn new(input: &str) -> Self {
        ProdExchangeClient {
            test: input.to_string(),
        }
    }
}

#[automock]
trait TraitsToBeMocked {
    fn get_test(&self) -> String;

    fn get_response(&self) -> Result<String, reqwest::Error> {
        Ok(reqwest::blocking::get("https://www.rust-lang.org")?.text()?)
    }
}

struct ServiceWrapper<T: TraitsToBeMocked> {
    client: T,
}

impl<T: TraitsToBeMocked> ServiceWrapper<T> {
    fn new(client: T) -> ServiceWrapper<T> {
        Self { client }
    }

    fn get_test(&self) -> String {
        self.client.get_test()
    }

    fn get_response(&self) -> Result<String, reqwest::Error> {
        self.client.get_response()
    }
}

#[cfg(test)]
mod mockall_unit_tests {
    use super::*;

    #[test]
    fn mockall_test() {
        let mut mock = MockTraitsToBeMocked::new();
        mock.expect_get_response()
            .returning(|| Ok("mocked_response".to_string()));
        mock.expect_get_test()
            .returning(|| "mocked_test".to_string());

        let service_under_test = ServiceWrapper::new(mock);
        assert_eq!("mocked_test".to_string(), service_under_test.get_test());
        assert_eq!(
            "mocked_response".to_string(),
            service_under_test.get_response().unwrap()
        );
    }
}
