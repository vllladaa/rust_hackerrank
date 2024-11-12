fn page_count(n: i32, p: i32) -> i32 {

    let from_start = p / 2;

    let from_end = (n / 2) - (p / 2);

    std::cmp::min(from_start, from_end)
}

fn main() {
    let n = 6;
    let p = 2;

    let result = page_count(n, p);
    println!("{}", result);
}
