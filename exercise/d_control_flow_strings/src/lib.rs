pub fn sum() {
    let mut sum = 0;

    for num in 7..=23 {
        sum += num; 
    };

    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 1;
    
    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

pub fn count(arg: String) {
    let mut i = 0;
    loop {
        if i >= 8 {break}    

        print!("{} ", arg);
        i += 1;
    }

    println!();
}
