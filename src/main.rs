mod modules {
    pub mod fibonacci;
    pub mod fizzbuzz;
}

fn main() {
    println!("fibonacci is {}", modules::fibonacci::fib(4));
    modules::fizzbuzz::fizzbuzz();
}
