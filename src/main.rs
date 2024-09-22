use rand::prelude::*;
use const_random::const_random;
use std::mem;
use std::io::{self, Write, BufWriter};
use std::time::*;
use serde_json;
use std::fs::File;

mod print;
mod constants;
mod maze_logic;

use print::*;
use constants::*;
use maze_logic::*;



fn main() {

    let mut rng = thread_rng();
    let x = rng.gen::<u8>();

    let mut maze_ = [[(false, [false; 4]); WIDTH as usize]; HEIGHT as usize];

    /*
    for row in maze_.iter_mut() {
        for cell in row.iter_mut() {
            cell.0 = rand.gen_bool(0.5);
        }
    }
    */

    //print::maze_print_speed_test(&maze_, 10000);
    //let mut maze = Maze::new(WIDTH, HEIGHT);
    //
    /*
    let mut maze = Maze::new(rng.gen::<u8>() as usize, rng.gen::<u8>() as usize);

    maze.print();
    maze.gen_maze();
    maze.print();
    */
    make_dataset("demo.txt", 10);
}

fn make_dataset(file_name: &str, size: u32){
    let mut rng = thread_rng();
    let file = File::create(file_name).expect("failed to create file");
    let mut writer = BufWriter::new(file);

    for _ in 0..size {
        let mut maze = Maze::new(rng.gen::<u8>() as usize, rng.gen::<u8>() as usize);
        maze.gen_maze();
        maze.set_pos();
        //maze.solve();
        maze.bfs_solve();
        /*
        maze.print();
        if (!maze.ideal_path.is_empty()){
            if (maze.can_follow_path()){
                println!("Solved");
            }
        }
        else {
            println!("Can Not Solve");
        }
        maze.print_pos();
        */

        let line = serde_json::to_string(&maze).unwrap();
        writeln!(writer, "{}", line).expect("was not able to write to file");
    }
}
