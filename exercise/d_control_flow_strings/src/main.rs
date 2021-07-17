fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "sum" {
            sum()
        } else if arg == "double" {
            double()
        } else {
            count(arg)
        }
    }
}

fn sum() {
    let mut sum = 0;

    for num in 7..=23 {
        sum += num; 
    };

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    
    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut i = 0;
    loop {
        if i >= 8 {break}    

        print!("{} ", arg);
        i += 1;
    }

    println!();
}
