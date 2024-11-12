use std::collections::HashMap;

fn sock_merchant(n: i32, ar: Vec<i32>) -> i32 {
    let mut sock_count = HashMap::new();

    for sock in ar {
        *sock_count.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for &count in sock_count.values() {
        pairs += count / 2;
    }

    pairs
}

fn main() {
    let n = 9;
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let result = sock_merchant(n, ar);
    println!("{}", result);
}
