use rand::prelude::*;
use const_random::const_random;
use std::mem;
use std::io::{self, Write};
use std::time::*;

mod print;
mod constants;
mod maze_logic;

use print::*;
use constants::*;
use maze_logic::*;



fn main() {

    let mut rand = thread_rng();

    let mut maze_ = [[(false, [false; 4]); WIDTH as usize]; HEIGHT as usize];

    /*
    for row in maze_.iter_mut() {
        for cell in row.iter_mut() {
            cell.0 = rand.gen_bool(0.5);
        }
    }
    */

    print::maze_print_speed_test(&maze_, 10000);



    //println!("DOD: {}, No DOD {}", maze.get_size(), mem::size_of_val(&maze_));

    //let map = [[(false, [false; 4]); WIDTH as usize]; HEIGHT as usize];
}

