fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Если скорости одинаковые
    if v1 == v2 {
        if x1 == x2 {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    if (x2 - x1) % (v1 - v2) == 0 && (x2 - x1) / (v1 - v2) >= 0 {
        return "YES".to_string();
    }

    "NO".to_string()
}

fn main() {
    // Пример 1
    let result = kangaroo(0, 3, 4, 2);
    println!("{}", result);

    // Пример 2
    let result = kangaroo(0, 2, 5, 3);
    println!("{}", result);
}
