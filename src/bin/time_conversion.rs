fn time_conversion(s: &str) -> String {

    let period = &s[8..];
    let hour = &s[0..2].parse::<u32>().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];

    let military_hour = match period {
        "AM" => {
            if *hour == 12 {
                0
            } else {
                *hour
            }
        }
        "PM" => {
            if *hour == 12 {
                12
            } else {
                *hour + 12
            }
        }
        _ => *hour,
    };

    format!("{:02}:{:}:{:}", military_hour, minutes, seconds)
}

fn main() {
    let input = "07:05:45PM";
    let result = time_conversion(input);
    println!("{}", result); // Вывод: 19:05:45
}
