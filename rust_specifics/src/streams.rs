/*
https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust/
Streams are an iterator when seen from the async world
Iterator<MyItem> - is a sequence of 0..N objects of my items.
Usually comes up when you want to iterate over a collection
In Rust an iterator has two requirements:
1. An assignable type 
2. The next method which returns an Option<T> where T is the type of the item
The reason for an option is in case you have no elements left and the iterator is now exhausted

Stream async iterator
Stream has a type but now th async version of next, it technically returns a future

Simplified definition of async rust
1. Futures - promises to yield a value in the future, a task that is going to perform an action to fufil this promise and return a value in the future
2. Poll - builds a bridge between async and sync world

IF future is a task that returns a value, a Poll is the return type of the future indicating if the value si ready or not.
Context usually a struct that contains a waker, it stops the async runtime from polling your future again and again even though it is not rwady yet.
The waker allows the future to notify the runtime when it is ready to progress and the value is ready to be used.
This stops CPU and resources form being used, Reactors are used to notify the runtime when the waker has woken up.
Pin - type that prevents the object from moving into memory

Creating a stream

Stream iter - create a stream from an iterator, useful when testing and you only care about the data sent
streams repeat with - create a stream and use a lambda to repeatedly pass values lazily

Future utils is different to stream utils in tokio and therefore this can create problems with imports

Stream unfold - takes states/ seed as a parameter
Make sure you pin a stream before using it
*/

use futures::stream::{self, StreamExt};

async fn streams_iter_demo(){
    let stream = stream::iter(vec![17, 19]);
    assert_eq!(vec![17, 19], stream.collect::<Vec<i32>>().await);   
}

async fn stream_repeat_with(){
    let mut curr = 1;
    let mut pow2 = stream::repeat_with(|| { let tmp = curr; curr *= 2; tmp });

    assert_eq!(Some(1), pow2.next().await);
    assert_eq!(Some(2), pow2.next().await);
    assert_eq!(Some(4), pow2.next().await);
    assert_eq!(Some(8), pow2.next().await);
}

fn custom_iterator_demo(){

}

#[cfg(test)]
mod stream_tests {
    
}
