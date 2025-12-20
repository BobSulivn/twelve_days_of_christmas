fn main() {
    let mut counter: i32 = 0;
    while counter < 12 {
        let day_of_christmas: String = get_day_of_christmas(counter);
        let christmas_present: String = get_christmas_present(counter);
        let mut inner_counter = counter - 1;
        println!(
            "On the {day_of_christmas} day of Christmas, my true love gave to me {christmas_present}"
        );
        if inner_counter != -1 {
            while inner_counter >= 0 {
                let other_present: String = get_christmas_present(inner_counter);
                if inner_counter != 0 {
                    println!(", {other_present}");
                } else {
                    println!("and {other_present}");
                }
                inner_counter -= 1;
            }
        }
        counter += 1;
    }
}

fn get_day_of_christmas(index: i32) -> String {
    let day_index: usize = index as usize;
    let days_of_christmas = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelth",
    ];
    return days_of_christmas[day_index].to_string();
}

fn get_christmas_present(index: i32) -> String {
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
