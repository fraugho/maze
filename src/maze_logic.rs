use std::{io::{self, Write}, mem};
use std::collections::HashSet;
use rand::*;
use rand::rngs::ThreadRng;
use crate::constants::*;
use rand::prelude::SliceRandom;

pub struct Maze {
    cells: [bool; WIDTH * HEIGHT],
    //l_walls: [bool; WIDTH * HEIGHT],
    r_walls: [bool; WIDTH * HEIGHT],
    //t_walls: [bool; WIDTH * HEIGHT],
    b_walls: [bool; WIDTH * HEIGHT],
}
impl Maze {
    pub fn new() -> Self {
        return Maze { 
            cells: [false; (WIDTH * HEIGHT) as usize],
            //l_walls: [false; (WIDTH * HEIGHT) as usize],
            r_walls: [false; (WIDTH * HEIGHT) as usize],
            //t_walls: [false; (WIDTH * HEIGHT) as usize],
            b_walls: [false; (WIDTH * HEIGHT) as usize],
        }
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
        let floor_option = [b'-', b' '];
        let floor = [b'-'; WIDTH];
        let space = [b' '; WIDTH];
        let ceiling = floor.iter().zip(space).flat_map(|(&floor, space)| [floor, space]).collect::<Vec<u8>>();
        //let floor: Vec<u8> = (0..WIDTH*2).map(|i| if i % 2 == 0 { b'_' } else { b' ' }).collect();
        //let floor: [u8; WIDTH] = core::array::from_fn(|i| b"_"[i & 1]);

        buffer.extend(ceiling.iter());
        buffer.push(b'\n');
        for row in 0..HEIGHT{
            let cell_slice = &self.cells[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            let r_wall_slice = &self.r_walls[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            let b_wall_slice = &self.b_walls[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            buffer.extend(cell_slice
                .iter()
                .zip(r_wall_slice.iter())
                .flat_map(|(&cell, &wall)| [cell_option[cell as usize], wall_option[wall as usize]]));
            buffer.push(b'\n');
            buffer.extend(b_wall_slice.iter().flat_map(|&floor| [floor_option[floor as usize], b' ']));
            //buffer.extend_from_slice(&floor);
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }


    fn is_not_outer_wall(x: usize, y: usize) -> bool {
        x > 0 && x < WIDTH - 1 && y > 0 && y < HEIGHT - 1
    }

    pub fn gen_maze(&mut self) {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut rng = thread_rng();
        let start_pos: [usize;2]  = [rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT)];
        self.recursive_backtrack(start_pos[0], start_pos[1], &mut visited, &mut rng);
    }

    fn recursive_backtrack(&mut self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>, rng: &mut ThreadRng) {
        visited.insert((x, y));
        self.cells[y * WIDTH + x] = true;  // Mark as path

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut shuffled_directions = directions.to_vec();
        shuffled_directions.shuffle(rng);

        for (dx, dy) in shuffled_directions {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;

            //checks if the position it is trying to go it is 
            //past top, bottom, left, and right walls

            if next_x >= 0 && next_x < WIDTH as i32 && next_y >= 0 && next_y < HEIGHT as i32 
            && !visited.contains(&(next_x as usize, next_y as usize)) {
                // Remove wall between current cell and next cell
                let wall_x = x as i32;
                let wall_y = y as i32;
                self.remove_wall(wall_x as usize, wall_y as usize, (dx, dy));

                self.recursive_backtrack(next_x as usize, next_y as usize, visited, rng);
            }
        }
    }

    fn remove_wall(&mut self, x: usize, y: usize, direction: (i32,i32)) {
        match direction {
            //UP
            (0, -1) => self.b_walls[(y - 1) * WIDTH + x] = true,
            //RIGHT
            (1, 0) => self.r_walls[y * WIDTH + x] = true,
            //DOWN
            (0, 1) => self.b_walls[y * WIDTH + x] = true,
            //LEFT
            (-1, 0) => self.r_walls[y * WIDTH + x - 1] = true,
            _ => panic!("invalid direction (fn remove_wall)"),
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
        let mut buffer: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT + HEIGHT);
        //let cell_option = [b'*', b' '];

        for row in 0..HEIGHT{
            let cell_slice = &self.cells[(HEIGHT*row)..(HEIGHT*row + WIDTH)];
            buffer.extend(cell_slice
                .iter()
                .map(|&cell| if !cell { b'*'} else { b' '}));
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }
}


pub struct Maze_ro {
    cells: [bool; WIDTH * HEIGHT],
    l_walls: [bool; WIDTH * HEIGHT],
    r_walls: [bool; WIDTH * HEIGHT],
    t_walls: [bool; WIDTH * HEIGHT],
    b_walls: [bool; WIDTH * HEIGHT]
}
impl Maze_ro {
    pub fn new() -> Self {
        return Maze_ro { 
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
        let mut buffer: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT + HEIGHT);
        //let cell_option = [b'*', b' '];
        for chunk in self.cells.chunks(WIDTH){
            buffer.extend(chunk.iter().map(|cell| if !cell {b'*'} else {b' '}));
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }
}
