
// declaring private cause it should be accessible in the main file only.
mod days;

fn main() {
    days::day1::ownership::demo();
    // passing arguments to the days folder day1 folder functions file sum functions
    println!(" the sum of 5 and 10 is {}", days::day1::functions::sum(5,10));
    days::day1::functions::function_demo();
    println!("is 5 positive? {}", days::day1::functions::is_positive(5));
    println!("the concatenated string is {}", days::day1::functions::concatinate("hello", "world"));

   days::day1::ownership::ownership_rules();
   days::day1::ownership::str_string_demo();

}
