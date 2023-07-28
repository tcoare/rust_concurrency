# Asynchronous `async` and `await` vs. Threads in Rust

After reading [this blog post](https://shane.ai/posts/threads-and-goroutines/) by Shane Hansen I wanted to create some code to better understand asynchronous async and await vs. threads in Rust

This repo contains a Rust code example that demonstrates the differences between using asynchronous `async` and `await` and threads for concurrent programming in Rust. The code calculates the factorial of a number to showcase both approaches.

## Asynchronous `async` and `await`

The **asynchronous approach** utilizes the `async` and `await` syntax in Rust, making it suitable for handling I/O-bound tasks. In this example, we use the `tokio` runtime to execute asynchronous code.

1. **Concurrency Model:** Asynchronous programming is based on cooperative multitasking, and it allows tasks to efficiently wait for I/O without blocking the thread.

2. **Resource Efficiency:** Asynchronous tasks are lightweight and do not require separate memory stacks or registers, making efficient use of a single thread.

3. **Communication:** Asynchronous tasks can communicate using message passing or channels, focusing on avoiding shared mutable state.

4. **Costs and Trade-offs:**
   - Asynchronous code may involve more complex reasoning about the flow of control compared to synchronous code.
   - Implementing asynchronous functions and handling error scenarios can be more challenging.
   - If the tasks are mostly CPU-bound, using asynchronous programming may not be the most efficient approach.

5. **Use Cases:** Asynchronous programming is well-suited for handling I/O-bound tasks, such as network requests or file operations, where tasks spend significant time waiting for external operations to complete.

## Threads

The **thread-based approach** uses threads, allowing us to achieve true parallelism for CPU-bound tasks. Each thread represents an independent sequence of instructions running on a CPU core.

1. **Concurrency Model:** Threads enable concurrent execution of multiple tasks, allowing for parallel execution if multiple CPU cores are available.

2. **Resource Intensive:** Threads are heavier constructs compared to asynchronous tasks, requiring separate memory stacks, register sets, and other resources.

3. **Communication:** Threads can communicate with each other by sharing memory, but this introduces complexities related to data races and synchronization.

4. **Costs and Trade-offs:**
   - Thread-based concurrency can be resource-intensive when dealing with a large number of threads.
   - Managing shared mutable state and ensuring data safety with threads can be challenging and may require synchronization primitives.
   - Threads may not scale well when the number of concurrent tasks is too high, leading to resource contention.

5. **Use Cases:** Threads are suitable for handling CPU-bound tasks that can benefit from true parallelism, such as complex computations or simulations.

## How to Run the Example

1. Ensure you have Rust and cargo installed on your system.

Run the example using the following command:

```shell
cargo run
```
The program will output the factorial of the number 10 using both the asynchronous approach and the thread-based approach, along with the time taken for each execution.
