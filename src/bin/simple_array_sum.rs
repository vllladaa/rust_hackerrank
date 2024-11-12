use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Читаємо розмір масиву (і ігноруємо його)
    input.clear();
    io::stdin().read_line(&mut input).unwrap(); // Читаємо елементи масиву

    let sum: i32 = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("{}", sum);
}