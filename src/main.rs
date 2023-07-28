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
