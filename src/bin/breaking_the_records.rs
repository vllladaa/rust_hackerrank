fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        }
        if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}

fn main() {
    // Пример 1
    let scores1 = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    let result1 = breaking_records(scores1);
    println!("{:?}", result1); // Ожидаемый вывод: [2, 4]

    // Пример 2
    let scores2 = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
    let result2 = breaking_records(scores2);
    println!("{:?}", result2); // Ожидаемый вывод: [4, 0]
}
