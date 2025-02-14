fn main() {
    println!("fibonacci is {}", fib(4))
}

//0 1 2 3 4 5 6 7  8  9  10
//0 1 1 2 3 5 8 13 21 34 55
fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for a in 1..num {
        println!("{}", a);
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}
