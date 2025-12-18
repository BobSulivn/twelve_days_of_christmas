fn main() {
    let day_of_christmas: String = get_day_of_christmas(4);
    let day_of_christmas_suffix: String = get_day_of_christmas_suffix(4);
    let christmas_present: String = get_christmas_present(4);
    println!(
        "On the {day_of_christmas}{day_of_christmas_suffix} my true love gave to me {christmas_present}"
    );
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

fn get_christmas_present(day: i8) -> String {
    // subtract 1 from the day to account for the 0 index in the array
    let present_index = (day - 1) as usize;
    let christmas_presents = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let christmas_present = christmas_presents[present_index];

    return christmas_present.to_string();
}
