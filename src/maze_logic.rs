use std::{io::{self, Write}, mem};
use crate::constants::*;

pub struct Maze {
    cells: [bool; WIDTH * HEIGHT],
    l_walls: [bool; WIDTH * HEIGHT],
    r_walls: [bool; WIDTH * HEIGHT],
    t_walls: [bool; WIDTH * HEIGHT],
    b_walls: [bool; WIDTH * HEIGHT]
}
impl Maze {
    pub fn new() -> Self {
        return Maze { 
            cells: [false; (WIDTH * HEIGHT) as usize],
            l_walls: [false; (WIDTH * HEIGHT) as usize],
            r_walls: [false; (WIDTH * HEIGHT) as usize],
            t_walls: [false; (WIDTH * HEIGHT) as usize],
            b_walls: [false; (WIDTH * HEIGHT) as usize]}
    }
    pub fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }
    pub fn print(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        //WIDTH * 3 accounts for cell and r_wall and b_wall
        // + HEIGHT is for \n
        let mut buffer: Vec<u8> = Vec::with_capacity((WIDTH * 4 * HEIGHT + HEIGHT) as usize);
        let wall_option = [b'|', b' '];
        let cell_option = [b'*', b' '];
        let floor = [b'_'; WIDTH * 2];
        //let floor: Vec<u8> = (0..WIDTH*2).map(|i| if i % 2 == 0 { b'_' } else { b' ' }).collect();
        //let floor: [u8; WIDTH] = core::array::from_fn(|i| b"_"[i & 1]);

        //buffer.extend_from_slice(&floor);
        //buffer.push(b'\n');
        for row in 0..HEIGHT{
            let cell_slice = &self.cells[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            let r_wall_slice = &self.r_walls[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            buffer.extend(cell_slice
                .iter()
                .zip(r_wall_slice.iter())
                .flat_map(|(&cell, &wall)| [cell_option[cell as usize], wall_option[wall as usize]]));
            buffer.push(b'\n');
            buffer.extend_from_slice(&floor);
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }
}

pub struct Maze_ {
    cells: [bool; WIDTH * HEIGHT],
    walls: [bool; WIDTH * HEIGHT * 4]
}
impl Maze_ {
    pub fn new() -> Self {
        return Maze_ { cells: [false; (WIDTH * HEIGHT) as usize], walls: [false; (WIDTH * HEIGHT * 4) as usize] }
    }
    pub fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }
    pub fn print(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let buffer: Vec<u8> = Vec::with_capacity((WIDTH * 3 + 1) as usize);

        for col in 0..WIDTH {
            //let slice = self.cells
            //buffer.extend()
        }
    }
}


pub struct Maze_r {
    cells: [bool; WIDTH * HEIGHT],
    l_walls: [bool; WIDTH * HEIGHT],
    r_walls: [bool; WIDTH * HEIGHT],
    t_walls: [bool; WIDTH * HEIGHT],
    b_walls: [bool; WIDTH * HEIGHT]
}
impl Maze_r {
    pub fn new() -> Self {
        return Maze_r { 
            cells: [false; (WIDTH * HEIGHT) as usize],
            l_walls: [false; (WIDTH * HEIGHT) as usize],
            r_walls: [false; (WIDTH * HEIGHT) as usize],
            t_walls: [false; (WIDTH * HEIGHT) as usize],
            b_walls: [false; (WIDTH * HEIGHT) as usize]}
    }
    pub fn get_size(&self) -> usize {
        mem::size_of::<Self>()
    }
    pub fn print(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        //WIDTH * 3 accounts for cell and r_wall and b_wall
        // + HEIGHT is for \n
        let mut buffer: Vec<u8> = Vec::with_capacity((WIDTH * HEIGHT + HEIGHT) as usize);
        //let cell_option = [b'*', b' '];

        for row in 0..HEIGHT{
            let cell_slice = &self.cells[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            let r_wall_slice = &self.r_walls[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            buffer.extend(cell_slice
                .iter()
                .map(|&cell| if !cell { b'*'} else { b' '}));
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }
}
