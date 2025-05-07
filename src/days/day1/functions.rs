pub fn function_demo(){
    let x = 5;
    let y = 10;
    println!("x + y = {}", x + y);
}

pub fn sum(a: i32 , b: i32)->i32{
    a+b
}

pub fn is_positive(num: i32)->bool{
    num>0
}

pub fn concatinate(_s: &str , _t: &str)->String{
    // here we are using the + operator to concatenate the two strings
    let result = String::from(_s) + _t;
    // here we are returning the result as a String
    result
}
