use rand::prelude::*;
use std::time::*;
use std::io::{self, Write};

use crate::constants::*;
use crate::maze_logic;

pub fn print_maze(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let options = ['*', ' '];
    for row in arr {
        for col in row{
            print!("{}", options[col.0 as usize]);
        }
        println!();
    }
}

pub fn print_maze_o(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let options = ['*', ' '];
    for row in arr {
        let line: String = row.iter().map(|x| options[x.0 as usize]).collect();
        println!("{}", line);
    }
}

pub fn print_maze_o2(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let options = ['*', ' '];
    for row in arr {
        println!("{}", row.iter().map(|x| options[x.0 as usize]).collect::<String>());
    }
}

pub fn print_maze_oo(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buffer = Vec::with_capacity(WIDTH + 1);
    const OPTION: [u8; 2] = [b'*', b' '];

    for row in arr {
        buffer.clear();
        buffer.extend(row.iter().map(|&(cell, _)| OPTION[cell as usize]));
        buffer.push(b'\n');
        handle.write_all(&buffer).unwrap();
    }

    handle.flush().unwrap();
}

pub fn print_maze_ooo(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let options = [b'*', b' '];

    let maze: Vec<u8> = arr.iter()
        .flat_map(|row| {
            row.iter()
                .map(|&(cell, _)| options[usize::from(cell)])
                .chain(std::iter::once(b'\n'))
        })
        .collect();

    handle.write_all(&maze).unwrap();
    handle.flush().unwrap();
}

pub fn branched_print_maze(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    for row in arr {
        for col in row{
            if col.0 {
                print!(" ");
            }
            else {
                print!("*");
            }
        }
        println!();
    }
}

pub fn branched_print_maze_o(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buffer = Vec::with_capacity(WIDTH + 1);

    for row in arr {
        buffer.clear();
        buffer.extend(row.iter().map(|&(cell, _)| if cell { b' ' } else { b'*' }));
        buffer.push(b'\n');
        handle.write_all(&buffer).unwrap();
    }
    handle.flush().unwrap();
}

pub fn branched_print_maze_oo(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    // Pre-allocate a buffer for the entire maze
    let mut buffer = Vec::with_capacity(WIDTH * HEIGHT + HEIGHT);
    
    for row in arr {
        buffer.extend(row.iter().map(|&(cell, _)| if cell { b' ' } else { b'*' }));
        buffer.push(b'\n');
    }

    // Write the entire maze at once
    handle.write_all(&buffer).unwrap();
    handle.flush().unwrap();
}


pub fn best_print_maze(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    const OPTION: [u8; 2] = [b'*', b' '];
    const WALL: [u8; 2] = [b'|', b' '];
    const TOP: [u8; 2] = [b'_', b' '];
    
    // Pre-allocate a buffer for the entire maze
    let mut buffer = Vec::with_capacity(WIDTH * 3 * HEIGHT * 3 + (HEIGHT * 3));
    
    for row in arr {
        //prints top 
        //buffer.extend(row.iter().map(|&(_, neighbor)| [TOP[neighbor[2]], TOP[neighbor[2]]]).flatten());
        //buffer.push(b'\n');
        //prints left space right
        buffer.extend(row.iter().map(|&(cell, neighbors)| [OPTION[cell as usize],WALL[neighbors[1] as usize]]).flatten());
        buffer.push(b'\n');
        //prints bottom
        buffer.extend(row.iter().map(|&(_, neighbor)| [TOP[neighbor[3] as usize], TOP[neighbor[3] as usize]]).flatten());
        buffer.push(b'\n');
    }

    // Write the entire maze at once
    handle.write_all(&buffer).unwrap();
    handle.flush().unwrap();
}


pub fn print_real_maze_uo(arr: &[[(bool, [bool; 4]); WIDTH]; HEIGHT]) {
    for row in arr {
        for col in row{
            if !col.0 {
                print!("*");
                if !col.1[1] {
                    print!("|");
                }
                else {
                    print!(" ");
                }
            }
            else {
                print!(" ");
            }
        }
        println!("");
        for col in row{
            if !col.1[3] {
                print!("__");
            }
            else {
                print!("  ");
            }
        }
        println!("");
    }
}


pub fn maze_print_speed_test(maze: &[[(bool, [bool; 4]); WIDTH]; HEIGHT], times: u128){

    let mut elasped_time = [0u128; 11];
    let print_funcs = [
        print_maze, print_maze_o, print_maze_o2, print_maze_oo, 
        print_maze_ooo, branched_print_maze, branched_print_maze_o, branched_print_maze_oo, best_print_maze, print_real_maze_uo
    ];

    for (order,func) in print_funcs.iter().enumerate(){
        for _ in 0..times{
            let now = Instant::now();
            func(&maze);
            elasped_time[order] += now.elapsed().as_micros();
        }
    }

    let mut maze = maze_logic::Maze_r::new();


    for _ in 0..times{
        let now = Instant::now();
        maze.print();
        elasped_time[10] += now.elapsed().as_micros();
    }

    println!("branchless print time in ms {}", elasped_time[0] / times);
    println!("branchless w/ lines print time in ms {}", elasped_time[1] / times);
    println!("branchless w/ one liner lines print time in ms {}", elasped_time[2] / times);
    println!("branchless direct buffer time in ms {}", elasped_time[3] / times);
    println!("branchless direct buffer time with collect in ms {}", elasped_time[4] / times);
    println!("branched print time in ms {}", elasped_time[5] / times);
    println!("branched direct buffer time in ms {}", elasped_time[6] / times);
    println!("branched direct buffer optimized time in ms {}", elasped_time[7] / times);
    println!("real print no struct time in ms {}", elasped_time[8] / times);
    println!("real print no struct uo time in ms {}", elasped_time[9] / times);
    println!("real print w/ struct time in ms {}", elasped_time[10] / times);
}
