use std::io;
fn birthday_cake_candles(candles: Vec<u32>) -> u32 {
    let max_height = candles.iter().max().unwrap();
    let count = candles.iter().filter(|&&height| height == *max_height).count();
    count as u32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

    input.clear();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

    let candles: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Ввод должен быть числом"))
        .collect();

    let result = birthday_cake_candles(candles);
    println!("{}", result);
}
