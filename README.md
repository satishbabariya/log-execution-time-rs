# log_execution_time

## Overview

`log_execution_time` is a Rust procedural macro that logs the execution time of functions. It leverages the [`log`](https://docs.rs/log/) crate to provide detailed timing information, making it a useful tool for performance monitoring and debugging.

## Features

- Measures and logs the execution time of functions.
- Compatible with synchronous and asynchronous functions.
- Lightweight and easy to integrate into existing projects.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
log_execution_time = "0.1.0"
log = "0.4" # Required for logging
```

Ensure you have a logger initialized in your project, such as `env_logger` or any other compatible logger.

## Usage

Annotate your functions with `#[log_execution_time]`:

```rust
use log_execution_time::log_execution_time;

#[log_execution_time]
fn compute() {
    // Perform some computation
    let sum: u32 = (1..=1000).sum();
    println!("Sum is: {}", sum);
}

fn main() {
    env_logger::init();
    compute();
}
```

### Async Functions

The macro also works with asynchronous functions:

```rust
use log_execution_time::log_execution_time;

#[log_execution_time]
async fn fetch_data() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Data fetched!");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    fetch_data().await;
}
```

## Example

Create an example file in your project to test the macro:

```bash
mkdir examples
```

Add the following to `examples/main.rs`:

```rust
use log_execution_time::log_execution_time;

#[log_execution_time]
fn example_function() {
    let product: u32 = (1..=10).product();
    println!("Product is: {}", product);
}

fn main() {
    env_logger::init();
    example_function();
}
```

Run the example:

```bash
cargo run --example main
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
