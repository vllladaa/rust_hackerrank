fn divisible_sum_pairs(n: usize, k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in i+1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}
fn main() {
    let first_line = "6 3".to_string(); // Строка для примера
    let second_line = "1 3 2 6 1 2".to_string(); // Строка для примера

    let first_line_parts: Vec<&str> = first_line.split_whitespace().collect();
    let n: usize = first_line_parts[0].parse().unwrap();
    let k: i32 = first_line_parts[1].parse().unwrap();

    let ar: Vec<i32> = second_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = divisible_sum_pairs(n, k, ar);

    println!("{}", result);  // 5
}
