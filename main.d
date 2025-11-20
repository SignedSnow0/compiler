fn factorial(n: i32): i32 {
    if n < 2 {
        return 1;
    }
    return n * factorial(n - 1);
}

fn factorial_iter(n: i32): i32 {
    let acc: i32 = 1;
    while n > 1 {
        acc = acc * n;
        n = n - 1;
    }

    return acc;
}

fn main(): i32 {
    let x: i32 = 5;
    return factorial_iter(x + 1) / factorial(x - 2);
}
