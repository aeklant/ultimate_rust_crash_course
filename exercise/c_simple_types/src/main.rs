// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::print_difference;
use ding_machine::print_array;
use ding_machine::ding;
use ding_machine::on_off;
use ding_machine::print_distance;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_array: [f32; 2] = [coords.0, coords.1]; 
    print_array(coords_array);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[series.len()-1]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords);
}
