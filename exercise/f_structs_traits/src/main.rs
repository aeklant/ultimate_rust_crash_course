use exercise_f::{ 
    Bite,
    Carrot,
    Grapes,
};

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot::new(100.0);
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes::new(100);
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    
    fn bunny_nibbles<T: Bite>(food: &mut T, n_nibbles: u8) {
        for _ in 0..n_nibbles {
            food.bite();
        }
    }

    bunny_nibbles(&mut carrot, 3);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    bunny_nibbles(&mut grapes, 3);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}
