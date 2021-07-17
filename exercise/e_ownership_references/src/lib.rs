pub fn inspect(arg: &String) {
    if arg.ends_with("s") { 
        println!("{} is plural", *arg);
    } else {
        println!("{} is singular", *arg);
    };
}

pub fn pluralize(arg: &mut String) {
    if !arg.ends_with("s") { arg.push_str("s") }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn add(op1: &i32, op2: &i32) -> i32 {
    *op1 + *op2
}
