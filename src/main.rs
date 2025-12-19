fn main() {
    let day_of_christmas: String = get_day_of_christmas(4);
    let christmas_present: String = get_christmas_present(4);
    println!(
        "On the {day_of_christmas} day of Christmas, my true love gave to me {christmas_present}"
    );
}

fn get_day_of_christmas(index: i8) -> String {
    let days_of_christmas = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelth",
    ];
    let day_index: usize = index as usize;
    return days_of_christmas[day_index].to_string();
}

fn get_christmas_present(index: i8) -> String {
    let present_index = index as usize;
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
