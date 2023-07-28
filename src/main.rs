// Asynchronous Approach: Using async and await
use async_recursion::async_recursion;
use std::time::Instant;

#[async_recursion]
async fn factorial_async(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_async(n - 1).await
    }
}

// Thread-based Approach: Using threads
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use tokio::runtime::Runtime;

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    // Asynchronous Approach
    let start_time_async = Instant::now();
    let n = 10;

    // Create the runtime
    let rt = Runtime::new().unwrap();

    // Get a handle from this runtime
    let handle = rt.handle();

    handle.block_on(async {
        let result = factorial_async(n).await;
        println!("Factorial of {:?} (async) is: {:?}", n, result);
    });

    let elapsed_time_async = start_time_async.elapsed();
    println!("Time taken (async): {:?}", elapsed_time_async);

    // Thread-based Approach
    let start_time_thread = Instant::now();
    let n = 10;

    let (tx, rx): (mpsc::Sender<u64>, Receiver<u64>) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = factorial(n);
        tx.send(result).unwrap();
    });

    let result = rx.recv().unwrap();
    println!("Factorial of {} (thread) is: {}", n, result);

    handle.join().unwrap();

    let elapsed_time_thread = start_time_thread.elapsed();
    println!("Time taken (thread): {:?}", elapsed_time_thread);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_factorial_async() {
        let n = 10;
        let start_time_async = Instant::now();
        let result = factorial_async(n);
        let elapsed_time_async = start_time_async.elapsed();

        // Assert the correct result and that it took a reasonable amount of time
        assert_eq!(
            tokio::runtime::Runtime::new().unwrap().block_on(result),
            3_628_800
        );
        assert!(elapsed_time_async < std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_factorial_thread() {
        let n = 10;
        let start_time_thread = Instant::now();
        let result = factorial(n);
        let elapsed_time_thread = start_time_thread.elapsed();

        // Assert the correct result and that it took a reasonable amount of time
        assert_eq!(result, 3_628_800);
        assert!(elapsed_time_thread < std::time::Duration::from_secs(1));
    }
}
