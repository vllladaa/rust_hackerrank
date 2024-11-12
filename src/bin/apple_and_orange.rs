fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {

    let apple_count = apples.iter()
        .filter(|&&apple| a + apple >= s && a + apple <= t)
        .count();

    let orange_count = oranges.iter()
        .filter(|&&orange| b + orange >= s && b + orange <= t)
        .count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let s_t = "7 11".to_string();
    let a_b = "5 15".to_string();
    let apples = "-2 2 1".to_string();
    let oranges = "5 -6".to_string();

    let s_t: Vec<i32> = s_t.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let a_b: Vec<i32> = a_b.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let apples: Vec<i32> = apples.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let oranges: Vec<i32> = oranges.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let (s, t) = (s_t[0], s_t[1]);
    let (a, b) = (a_b[0], a_b[1]);

    count_apples_and_oranges(s, t, a, b, apples, oranges);
}
