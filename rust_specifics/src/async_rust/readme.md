https://www.youtube.com/watch?v=4FdQCIJXjgo

## [Into to Async Await Rust](https://www.youtube.com/watch?v=K8LNPYNvT-U)

Async functions are just functions that return something that returns a future where there is an associated type which is wjat the funciton will return.
Future is a simple state machine that can be polled, if returned pending then it will continue to be polled. If poll returns ready the future will return the wake function to notify its executor that it is ready to work.

Futures are similar to promises, however they are lazy - they are driven to completetion by something else. Await will attempt to run the code to completetion.

To cancel a future all you have to do is stop polling the future.

Task is a lightweight green thread similar to a go routine.

https://www.youtube.com/watch?v=wLQnfJSGJPY

Async programming doesn't necessarily use threads.

