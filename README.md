# Mini Exercises in Rust

This repository contains a set of mini exercises in Rust. All tasks are consolidated in a single file, and each is implemented as a separate function.

## Project Structure

The code is contained in a single Rust file (e.g., `main.rs`) that includes:
- Functions implementing individual tasks (e.g., `fn task1()`, `fn task2()`, etc.).
- The `main()` function, which serves as the entry point of the program and allows you to call specific tasks.

## How to Run

1. Make sure you have Rust tools installed (you can download them from [rust-lang.org](https://www.rust-lang.org/)).

2. Clone this repository:
   ```bash
   git clone https://github.com/Szafter12/Rust-exercises
   cd Rust-exercises
   ```

3. Run the program to select and execute a specific task:
   ```bash
   cargo run
   ```

4. If needed, modify the `main()` function in the `main.rs` file to call a different task function.

## Example Content of `main.rs`

```rust
fn fibbonacci(n: i32) {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    for _ in 0..n {
        println!("{}", first);
        let temp: i32 = first;
        first = second + temp;
        second = temp;
    }
}

fn main() {
    fibbonacci(10);
}
```

## Planned Extensions

- Adding more tasks.
- Splitting tasks into modules when the number of functions becomes too large.
- Adding unit tests for each function.

## How to Contribute

1. Fork this repository.
2. Create a new branch for your changes:
   ```bash
   git checkout -b branch-name
   ```
3. Make your changes and create a pull request.
