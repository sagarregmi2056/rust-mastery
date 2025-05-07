// days/day1/ownership.rs


pub fn demo() {
    let s = String::from("ownership");
    println!("{}", s);
}
pub fn ownership_rules(){
    // here the s1 allocated in the heap memory and the s2 is a copy of s1
    let s1 = String::from("hello world from the ownership rules");
// here the ownership of String hello is transferred to s2
    let s2 = s1;
    // here the ownership of String hello is transferred to s2
    // here we can't print s1 because it is not in the scope
    println!("{}", s2);
}

pub fn str_string_demo(){
    let static_string = "hello world from the static string";
    let string_object = String::from("hello world from the string object");
    println!("{}", static_string);
    println!("{}", string_object);


    let s : &str = "hello world from the static string &str";
    let  dynamic_string = s.to_string();
    println!("Before modification: {}", dynamic_string);

   let mut  final_string = dynamic_string;
//    here we are pushing welcome to the dynamic string which is mutable

   final_string.push_str("welcome");

    println!("final string :{}", final_string);
}



