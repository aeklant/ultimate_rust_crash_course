const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut total_missiles: i32 = STARTING_MISSILES;
    let ready_missiles: i32 = READY_AMOUNT;

    println!(
        "Firing {} of my {} missiles...", ready_missiles, total_missiles);
    total_missiles -= ready_missiles;

    println!("{} missiles left", total_missiles)
}
