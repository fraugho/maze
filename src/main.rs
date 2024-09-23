use rand::prelude::*;
use std::io::{self, Write, BufWriter};
use std::sync::{Arc};
use std::time::Instant;
use serde_json;
use std::fs::File;
use std::thread;

mod print;
mod constants;
mod maze_logic;
mod model_tester;

use constants::*;
use maze_logic::*;

const MAX_MAZE_SIZE: usize = 10;

fn main() {
    /*
    if let Err(e) = model_tester::main() {
        eprintln!("Error: {:?}", e);
    }
    */
    let mut now = Instant::now();
    mt_make_dataset("demo3.txt", 100);
    println!("mt took {} ms", now.elapsed().as_micros());
    now = Instant::now();
    make_dataset("demo4.txt", 100);
    println!("st took {} ms", now.elapsed().as_micros());
    //print_speed_test();
}

fn print_speed_test(){
    let mut rng = thread_rng();
    let mut maze_ = [[(false, [false; 4]); WIDTH as usize]; HEIGHT as usize];

    for row in maze_.iter_mut() {
        for cell in row.iter_mut() {
            cell.0 = rng.gen_bool(0.5);
        }
    }

    print::maze_print_speed_test(&maze_, 10000);
}
fn mt_make_dataset(file_name: &str, size: usize) {
    let num_cpus = num_cpus::get();
    let times = size / num_cpus;
    let mut handles = vec![];

    // Create a vector to store the mazes, each thread will append to it
    let mut thread_times = vec![times; num_cpus];
    thread_times[num_cpus - 1] += size - (times * num_cpus);

    // Create a shared vector to collect the mazes from all threads
    let mut mazes = vec![Vec::new(); num_cpus]; // Vec of Vecs to hold results for each thread

    for i in 0..num_cpus {
        let times = thread_times[i];
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            let mut local_maze = Vec::new(); // Local vector for this thread
            
            // Generate mazes
            for _ in 0..times {
                let mut maze = Maze::new(rng.gen_range(1..=MAX_MAZE_SIZE) as usize, rng.gen_range(1..=MAX_MAZE_SIZE) as usize);
                maze.gen_maze();
                maze.set_pos();
                maze.bfs_solve();
                local_maze.push(maze);
            }
            local_maze // Return local vector of mazes
        });
        handles.push(handle);
    }

    // Collect all the mazes from threads
    for (i, handle) in handles.into_iter().enumerate() {
        let local_mazes = handle.join().unwrap(); // Get the result from the thread
        mazes[i] = local_mazes; // Store it in the main mazes vector
    }

    // Flatten all thread results into a single vector
    let all_mazes: Vec<Maze> = mazes.into_iter().flatten().collect();

    // Write the mazes to a file
    let file = File::create(file_name).expect("was not able to create file");
    let mut writer = BufWriter::new(file);
    for maze in all_mazes {
        writeln!(writer, "{}", serde_json::to_string(&maze).unwrap()).expect("failed to write");
    }
}

fn make_dataset(file_name: &str, size: usize){


    let mut rng = thread_rng();
    let file = File::create(file_name).expect("failed to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..size {
        let mut maze = Maze::new(rng.gen_range(1..=MAX_MAZE_SIZE) as usize, rng.gen_range(1..=255) as usize);
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
        if i % 1000 == 0 {
            println!("{}", i);
        }
    }
}
