use std::io;

fn a_very_big_sum(ar: Vec<i64>) -> i64 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let _n: usize = input.trim().parse().expect("Ввод не является целым числом");

    input.clear();

    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let ar: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Не является целым числом"))
        .collect();

    let result = a_very_big_sum(ar);
    println!("{}", result);
}