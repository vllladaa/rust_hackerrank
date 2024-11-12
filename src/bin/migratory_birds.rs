fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut bird_count = vec![0; 6];

    for &bird in arr.iter() {
        bird_count[bird as usize] += 1;
    }

    let max_count = bird_count.iter().max().unwrap();

    for i in 1..=5 {
        if bird_count[i] == *max_count {
            return i as i32;
        }
    }

    0
}

fn main() {
    // Пример 0
    let arr = vec![1, 4, 4, 4, 5, 3];
    println!("{}", migratory_birds(arr));

    // Пример 1
    let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
    println!("{}", migratory_birds(arr));
}
