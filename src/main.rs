fn main() {
    let day_of_christmas: String = get_day_of_christmas(4);
    let day_of_christmas_suffix: String = get_day_of_christmas_suffix(4);
    println!("Day of Christmas: {day_of_christmas}{day_of_christmas_suffix}");
    println!("Hello, world!");
}

fn get_day_of_christmas(day: i8) -> String {
    let day_number: String = day.to_string();
    return day_number;
}

fn get_day_of_christmas_suffix(day: i8) -> String {
    let day_suffix: String = if day == 1 {
        "st".to_string()
    } else if day == 2 {
        "nd".to_string()
    } else if day == 3 {
        "rd".to_string()
    } else {
        "th".to_string()
    };
    return day_suffix;
}
