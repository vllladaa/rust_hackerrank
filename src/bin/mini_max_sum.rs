use std::io;
fn mini_max_sum(arr: Vec<u64>) {
    let total_sum: u64 = arr.iter().sum();
    let min_element = arr.iter().min().unwrap();
    let max_element = arr.iter().max().unwrap();

    let min_sum = total_sum - max_element;
    let max_sum = total_sum - min_element;

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

    let arr: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Ввод должен быть числом"))
        .collect();

    mini_max_sum(arr);
}
