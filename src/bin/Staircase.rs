use std::io;
fn staircase(n: usize) {
    for i in 1..=n {
        let spaces = n - i;
        let hashes = i;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let n: usize = input.trim().parse().expect("Ввод не является целым числом");

    staircase(n);
}
