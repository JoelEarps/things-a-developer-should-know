// Join Handles are owned permission to join on a task
// The join handle can return value which you need to await, when you call await you will wait for the background task to finish before continuing
// If no value you don't need to await the join handle
use tokio::task::JoinHandle;

fn join_handles() -> JoinHandle<i32> {
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        1 + 1
    })
}

#[cfg(test)]
mod tokio_test {

    use super::*;

    #[tokio::test]
    async fn join_handles_and_join_sets() {
        println!("Starting task");
        let task_one_handle: JoinHandle<i32> = join_handles();
        let result = task_one_handle.await.expect("Task panicked");
        println!("We do not get here till task finished with :{}", result);

        println!("This task will spin forever and not complete");
        // 3. You can also abort a task before it finishes
        let handle2 = tokio::spawn(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                println!("This will never complete and spin in the background");
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        // Stop the task early
        handle2.abort();

        // Awaiting after aborting returns an error
        match handle2.await {
            Err(join_error) if join_error.is_cancelled() => {
                println!("Task was aborted successfully");
            }
            _ => panic!("Task was not aborted properly"),
        }
    }
}
