/*
Unique<T> is a pointer that declaires the following:
We are a covariant over T
We may own a value of type T
We are send and sync if T is send and sync
Our pointer is never null 


You will typically use this
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unique_demo() {
        let unique = Unique::new(1);
    }
}