// =============================================================================
// TOKIO TASKS & ASYNC EXECUTION: How Green Threads Work
// =============================================================================
//
// KEY CONCEPTS:
// 1. Tokio tasks are "green threads" - lightweight, cooperative tasks scheduled by the runtime
// 2. Tasks ONLY yield control at `.await` points - this is cooperative multitasking
// 3. Between await points, a task has exclusive use of its thread
// 4. CPU-bound work WITHOUT await points will BLOCK other tasks on the same thread
// 5. Progress is made by the executor polling tasks when they're ready
//
// THE ASYNC EXECUTION MODEL:
// - When you call `.await`, you're saying "I'm waiting for something, let other tasks run"
// - The executor (Tokio runtime) switches to another ready task
// - When the awaited future is ready, the task gets scheduled again
// - This is fundamentally different from OS threads which are preemptively scheduled

use std::time::Instant;

/// Demonstrates that tasks yield control ONLY at .await points
/// This is a well-behaved async function - it yields frequently
pub async fn yielding_task(task_name: &str, iterations: u32) {
    for i in 0..iterations {
        println!("[{}] Starting iteration {}", task_name, i);
        // .await is a YIELD POINT - other tasks can run while we sleep
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("[{}] Finished iteration {}", task_name, i);
    }
}

/// BAD EXAMPLE: This task does CPU-bound work WITHOUT yielding
/// It will BLOCK the executor thread and prevent other tasks from running
pub async fn blocking_cpu_task(task_name: &str) {
    println!("[{}] Starting CPU-bound work (THIS WILL BLOCK!)", task_name);

    // Simulate CPU-bound work - NO await points means NO yielding
    // Other tasks on this thread CANNOT make progress during this time
    let start = Instant::now();
    let mut sum: u64 = 0;
    for i in 0..50_000_000 {
        sum = sum.wrapping_add(i);
    }
    let elapsed = start.elapsed();

    println!(
        "[{}] Finished CPU work in {:?}, sum={} (blocked entire thread!)",
        task_name, elapsed, sum
    );
}

/// GOOD EXAMPLE: CPU-bound work with manual yield points using tokio::task::yield_now()
/// This allows other tasks to make progress
pub async fn cooperative_cpu_task(task_name: &str) {
    println!("[{}] Starting cooperative CPU work", task_name);

    let start = Instant::now();
    let mut sum: u64 = 0;
    for i in 0..50_000_000 {
        sum = sum.wrapping_add(i);

        // Yield every 1 million iterations to let other tasks run
        if i % 1_000_000 == 0 {
            // yield_now() is a manual yield point - gives other tasks a chance
            tokio::task::yield_now().await;
        }
    }
    let elapsed = start.elapsed();

    println!(
        "[{}] Finished cooperative CPU work in {:?}, sum={}",
        task_name, elapsed, sum
    );
}

/// Shows how multiple async I/O operations can run concurrently
/// Because each .await is a yield point, tasks interleave
pub async fn interleaved_io_task(task_name: &str, steps: u32) {
    for step in 0..steps {
        println!("[{}] Step {} - about to do async I/O", task_name, step);
        // Simulates async I/O - this is where we YIELD to other tasks
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("[{}] Step {} - async I/O complete", task_name, step);
    }
}

/// Demonstrates tokio::task::spawn_blocking for CPU-bound work
/// This moves the blocking work to a dedicated thread pool
pub async fn proper_blocking_task(task_name: &str) -> u64 {
    println!("[{}] Spawning blocking task on thread pool", task_name);

    // spawn_blocking moves this to a separate thread pool
    // The async executor thread is NOT blocked
    let result = tokio::task::spawn_blocking(move || {
        println!("[spawn_blocking] Running on dedicated blocking thread");
        let mut sum: u64 = 0;
        for i in 0..50_000_000 {
            sum = sum.wrapping_add(i);
        }
        sum
    })
    .await
    .expect("Blocking task panicked");

    println!(
        "[{}] Blocking task completed with result: {}",
        task_name, result
    );
    result
}

/// Shows the difference between sequential and concurrent execution
pub async fn sequential_tasks() -> std::time::Duration {
    let start = Instant::now();

    // Sequential: Task B waits for Task A to complete
    // Total time = time(A) + time(B)
    async_work("Sequential-A").await;
    async_work("Sequential-B").await;

    start.elapsed()
}

pub async fn concurrent_tasks() -> std::time::Duration {
    let start = Instant::now();

    // Concurrent: Both tasks run "at the same time"
    // Total time ≈ max(time(A), time(B))
    let (_, _) = tokio::join!(async_work("Concurrent-A"), async_work("Concurrent-B"));

    start.elapsed()
}

async fn async_work(name: &str) {
    println!("[{}] Starting work", name);
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    println!("[{}] Finished work", name);
}

/// Demonstrates how tasks are polled by showing the Future trait mechanism
/// This is what happens under the hood when you write async/await
pub async fn explain_polling() {
    println!("=== How Polling Works ===");
    println!("1. When you call .await, you're calling Future::poll()");
    println!("2. poll() returns Poll::Pending (not ready) or Poll::Ready(value)");
    println!("3. If Pending, the task yields and the executor runs other tasks");
    println!("4. When the resource is ready, the waker notifies the executor");
    println!("5. The executor polls the task again");
    println!();

    // This is conceptually what happens:
    // loop {
    //     match future.poll(context) {
    //         Poll::Ready(result) => return result,
    //         Poll::Pending => {
    //             // Yield control, run other tasks
    //             // Wait for waker to signal readiness
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tokio_task_tests {
    use super::*;

    /// Test 1: Shows interleaved execution of yielding tasks
    /// Expected: Task outputs will be interleaved because they yield at .await
    #[tokio::test]
    async fn test_yielding_tasks_interleave() {
        println!("\n=== TEST: Yielding Tasks Interleave ===\n");

        // Spawn two tasks that will run concurrently
        let task_a = tokio::spawn(yielding_task("Task-A", 3));
        let task_b = tokio::spawn(yielding_task("Task-B", 3));

        // Wait for both - notice how their output interleaves!
        let _ = tokio::join!(task_a, task_b);

        println!("\n=== Notice how Task-A and Task-B outputs are interleaved! ===\n");
    }

    /// Test 2: Shows that CPU-bound work blocks other tasks
    /// Expected: The blocking task runs to completion before other task starts
    #[tokio::test(flavor = "current_thread")]
    async fn test_blocking_prevents_progress() {
        println!("\n=== TEST: Blocking Task Prevents Progress ===\n");

        // On a single-threaded runtime, the blocking task will prevent
        // the other task from making ANY progress
        let blocker = tokio::spawn(blocking_cpu_task("Blocker"));
        let worker = tokio::spawn(yielding_task("Worker", 3));

        let _ = tokio::join!(blocker, worker);

        println!("\n=== The Blocker ran entirely before Worker could make progress! ===\n");
    }

    /// Test 3: Shows cooperative CPU task allowing progress
    /// Expected: Tasks interleave even with CPU work due to yield_now()
    #[tokio::test(flavor = "current_thread")]
    async fn test_cooperative_cpu_work() {
        println!("\n=== TEST: Cooperative CPU Work ===\n");

        let cpu_task = tokio::spawn(cooperative_cpu_task("CPU-Worker"));
        let io_task = tokio::spawn(yielding_task("IO-Worker", 5));

        let _ = tokio::join!(cpu_task, io_task);

        println!("\n=== Tasks were able to interleave due to yield_now()! ===\n");
    }

    /// Test 4: Shows spawn_blocking for proper handling of blocking work
    #[tokio::test]
    async fn test_spawn_blocking() {
        println!("\n=== TEST: spawn_blocking for CPU Work ===\n");

        // spawn_blocking moves work to a thread pool, freeing the async executor
        let blocking_task = tokio::spawn(proper_blocking_task("Heavy-Compute"));
        let io_task = tokio::spawn(interleaved_io_task("IO-Task", 3));

        let (blocking_result, _) = tokio::join!(blocking_task, io_task);
        println!("Blocking task result: {:?}", blocking_result);

        println!("\n=== IO-Task made progress while CPU work ran on separate thread! ===\n");
    }

    /// Test 5: Sequential vs Concurrent execution timing
    #[tokio::test]
    async fn test_sequential_vs_concurrent() {
        println!("\n=== TEST: Sequential vs Concurrent Execution ===\n");

        let sequential_time = sequential_tasks().await;
        println!("Sequential execution took: {:?}", sequential_time);

        let concurrent_time = concurrent_tasks().await;
        println!("Concurrent execution took: {:?}", concurrent_time);

        // Concurrent should be roughly half the time
        println!(
            "\nConcurrent was ~{:.1}x faster!",
            sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64
        );
    }

    /// Test 6: Demonstrates task spawning and join handles
    #[tokio::test]
    async fn test_task_spawning() {
        println!("\n=== TEST: Task Spawning ===\n");

        // spawn() creates a new task that runs independently
        // The task runs on the runtime's thread pool
        let handle = tokio::spawn(async {
            println!("Spawned task starting");
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            println!("Spawned task finishing");
            42 // Return value
        });

        println!("Main task continues immediately after spawn");

        // .await on JoinHandle waits for task completion and gets result
        let result = handle.await.expect("Task panicked");
        println!("Spawned task returned: {}", result);
    }

    /// Test 7: Shows that without .await, async functions don't run
    #[tokio::test]
    async fn test_lazy_futures() {
        println!("\n=== TEST: Futures are Lazy ===\n");

        // Creating a future does NOT start execution
        let future = async {
            println!("This only prints when awaited!");
        };

        println!("Future created but not yet running...");

        // Execution only starts when we await
        future.await;

        println!("Future completed!");
    }

    /// Test 8: tokio::select! for racing tasks
    #[tokio::test]
    async fn test_select_racing() {
        println!("\n=== TEST: Racing Tasks with select! ===\n");

        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                println!("Timer won!");
            }
            _ = async {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            } => {
                println!("Long task won!");
            }
        }

        println!("select! completes when FIRST branch finishes");
    }

    /// Overview test that explains the concepts
    #[tokio::test]
    async fn test_explain_async_model() {
        explain_polling().await;

        println!("=== Summary ===");
        println!("✓ Tasks yield ONLY at .await points (cooperative scheduling)");
        println!("✓ CPU-bound work without yields BLOCKS the executor thread");
        println!("✓ Use yield_now() for manual yield points in CPU loops");
        println!("✓ Use spawn_blocking() for heavy CPU work");
        println!("✓ Futures are lazy - they don't run until awaited");
        println!("✓ tokio::join! runs futures concurrently");
        println!("✓ tokio::select! races futures, completing on first finish");
    }
}
