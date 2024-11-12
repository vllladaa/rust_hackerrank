fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 { // Renamed to snake_case

    let l = a.iter().cloned().reduce(|acc, x| lcm(acc, x)).unwrap();

    let g = b.iter().cloned().reduce(|acc, x| gcd(acc, x)).unwrap();

    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}
fn main() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let result = get_total_x(a, b);
    println!("{}", result);
}
