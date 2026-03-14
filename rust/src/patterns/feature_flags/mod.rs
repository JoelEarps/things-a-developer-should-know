mod feature_flags_functionality_one;
mod feature_flags_functionality_two;
mod handler_struct;

pub use handler_struct::AppComponentManager;

/*

Learning notes: https://medium.com/better-programming/compile-time-feature-flags-in-rust-why-how-when-129aada7d1b3

Feature flags are compile time configuration that improve codes performance, portability, size and maintainability
Performance - only including functionality specific to use case you stop calling unnecessary functions
Size - overall size of binary is influenced by which flags are on and off
Maintainability - you can turn off broken code to not affect the rest off the functionality for example, in our instance if we are going to have issues with a kafka broker we can then turn off the feature flag to triggers the kafka service thus removing the issue
Security - once again any security issues you can turn off risks that occur overnight. Furthermore the more code you depend on the more likely to have issues you are.
Deployment - you can also deploy code to a subset of users and use various deployment strategies.
Portability - turn off specific OS based software requirements to allow cross compilation.

🧱 Goal
I have a struct that manages components.
One of those is a Kafka/WebSocket streaming component.
I want to exclude the stream setup and usage entirely at compile-time if the feature isn’t enabled.

Software design patterns:
1. Strategy Pattern - dStrategy is a behavioral design pattern that lets you define a family of algorithms, put each of them into a separate class, and make their objects interchangeable. In our case we do this at compile time.
2. Null Object Pattern - Providing a null object with the same interface as the real one, allowing you to create a null implementation so you don't need iof checks everywhere.
3. Abstract Factory Pattern - Using rusts trait system could argue we are using a abstract family pattern, creating a different object based on config or context. We are producing a family of related objects without specifying concrete classes

It uses Rust compile time configuration idiom.

Architectural Pattern - using the plugin or modular architecture.

*/
