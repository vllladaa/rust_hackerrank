use std::io;

fn diagonal_difference(arr: Vec<Vec<i32>>, n: usize) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][n - i - 1];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let n: usize = input.trim().parse().expect("Ввод не является целым числом");

    let mut arr = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Не является целым числом"))
            .collect();
        arr.push(row);
    }

    let result = diagonal_difference(arr, n);
    println!("{}", result);
}
