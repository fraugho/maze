use std::{io::{self, Write}, mem};
use std::collections::{HashSet, VecDeque};
use rand::*;
use rand::rngs::ThreadRng;
use crate::constants::*;
use serde::{Serialize, Deserialize};
use rand::prelude::SliceRandom;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn to_offset(&self) -> (i16, i16) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub r_walls: Vec<bool>,
    pub b_walls: Vec<bool>,
    pub ideal_path: Vec<(i8,i8)>,
    pub start_pos: (u8, u8),
    pub end_pos: (u8, u8),
}
impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let size: usize = width * height;
        let mut maze = Maze {
            width,
            height,
            cells: vec![false; size],
            r_walls: vec![true; size],
            b_walls: vec![true; size],
            ideal_path: vec![],
            start_pos: (0, 0),
            end_pos: (0, 0),
        };
        maze.set_pos();
        maze.gen_maze();
        return maze;
    }

    pub fn set_pos(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            self.start_pos = (
                rng.gen_range(0..self.width) as u8,
                rng.gen_range(0..self.height) as u8
            );
            self.end_pos = (
                rng.gen_range(0..self.width) as u8,
                rng.gen_range(0..self.height) as u8
            );
            if self.start_pos != self.end_pos {
                break;
            }
        }
    }

    pub fn print_pos(&self){
        println!("start_pos:{:?} end_pos:{:?}", self.start_pos, self.end_pos);
    }

    pub fn bfs_solve(&mut self) -> Option<Vec<Direction>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = std::collections::HashMap::new();

        queue.push_back((self.start_pos, Vec::new()));
        visited.insert(self.start_pos);

        while let Some((current_pos, path)) = queue.pop_front() {
            if current_pos == self.end_pos {
                self.ideal_path = path
                    .iter()
                    .map(|direction| match direction {
                        Direction::Left => (-1, 0),
                        Direction::Right => (1, 0),
                        Direction::Up => (0, -1),
                        Direction::Down => (0, 1),
                    })
                    .collect();
                return Some(path);
            }

            for &direction in &[Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let (dx, dy) = direction.to_offset();
                let new_pos = (
                    (current_pos.0 as i16 + dx) as u8,
                    (current_pos.1 as i16 + dy) as u8,
                );

                if !self.is_edge(&current_pos, &(dx, dy)) 
                && self.can_move(current_pos, direction) 
                && !visited.contains(&new_pos) {
                    let mut new_path = path.clone();
                    new_path.push(direction);
                    queue.push_back((new_pos, new_path));
                    visited.insert(new_pos);
                    came_from.insert(new_pos, (current_pos, direction));
                }
            }
        }

        None
    }

    pub fn is_edge(&self, cur_pos: &(u8, u8), direction: &(i16, i16)) -> bool {
        let new_pos = (cur_pos.0 as i16 + direction.0, cur_pos.1 as i16 + direction.1);
        new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= self.width as i16 || new_pos.1 >= self.height as i16
    }

    fn can_move(&self, pos: (u8, u8), direction: Direction) -> bool {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        match direction {
            Direction::Left => x > 0 && !self.r_walls[y * self.width + x - 1],
            Direction::Right => x < self.width - 1 && !self.r_walls[y * self.width + x],
            Direction::Up => y > 0 && self.b_walls[(y - 1) * self.width + x],
            Direction::Down => y < self.height - 1 && !self.b_walls[y * self.width + x],
        }
    }

    fn can_move_path(&self, pos: (u8, u8), direction: (i8, i8)) -> bool {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        match direction {
            (-1, 0) => x > 0 && !self.r_walls[y * self.width + x - 1],
            (1, 0) => x < self.width - 1 && !self.r_walls[y * self.width + x],
            (0, -1) => y > 0 && !self.b_walls[(y - 1) * self.width + x],
            (0, 1) => y < self.height - 1 && !self.b_walls[y * self.width + x],
            _ => false,
        }
    }

    pub fn can_follow_path(&self) -> bool {
        let mut cur_pos = self.start_pos;
        for &direction in &self.ideal_path {
            if self.can_move_path(cur_pos, direction) {
                cur_pos.0 = cur_pos.0.wrapping_add(direction.0 as u8);
                cur_pos.1 = cur_pos.1.wrapping_add(direction.1 as u8);
            } else {
                return false;
            }
        }
        cur_pos == self.end_pos
    }

    pub fn print(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let mut buffer: Vec<u8> = Vec::with_capacity((self.width * 4 * self.height + self.height) as usize);
        let wall_option = [b' ', b'|'];
        let cell_option = [b' ', b'*'];
        let floor_option = [b' ', b'-'];
        let floor = vec![b'-'; self.width];
        let space = vec![b' '; self.width];
        let ceiling = space.iter().zip(floor).flat_map(|(&floor, space)| [floor, space]).collect::<Vec<u8>>();

        buffer.extend(ceiling.iter());
        buffer.push(b'\n');

        for row in 0..self.height {
            let cell_slice = &self.cells[(self.width*row)..(self.width*row + self.width)];
            let r_wall_slice = &self.r_walls[(self.width*row)..(self.width*row + self.width)];
            let b_wall_slice = &self.b_walls[(self.width*row)..(self.width*row + self.width)];

            buffer.push(b'|');
            for col in 0..self.width {
                let cell = if (col as u8, row as u8) == self.start_pos {
                    b'S'
                } else if (col as u8, row as u8) == self.end_pos {
                    b'E'
                } else {
                    cell_option[cell_slice[col] as usize]
                };
                buffer.push(cell);
                buffer.push(wall_option[r_wall_slice[col] as usize]);
            }
            buffer.push(b'\n');

            buffer.push(b' ');
            buffer.extend(b_wall_slice.iter().flat_map(|&floor| [floor_option[floor as usize], b' ']));
            buffer.push(b'\n');
        }

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }
    //doesnt correctly display start and end
    pub fn z_print(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        //WIDTH * 3 accounts for cell and r_wall and b_wall
        // + HEIGHT is for \n
        let mut buffer: Vec<u8> = Vec::with_capacity((self.width * 4 * self.height + self.height) as usize);
        let wall_option = [b'|', b' '];
        let cell_option = [b'*', b' '];
        let floor_option = [b'-', b' '];
        let floor = vec![b'-'; self.width];
        let space = vec![b' '; self.width];
        let ceiling = space.iter().zip(floor).flat_map(|(&floor, space)| [floor, space]).collect::<Vec<u8>>();
        //let floor: Vec<u8> = (0..WIDTH*2).map(|i| if i % 2 == 0 { b'_' } else { b' ' }).collect();
        //let floor: [u8; WIDTH] = core::array::from_fn(|i| b"_"[i & 1]);

        buffer.extend(ceiling.iter());
        buffer.push(b'\n');
        for row in 0..self.height{
            let cell_slice = &self.cells[(self.width*row)..(self.width*row + self.width)];
            let r_wall_slice = &self.r_walls[(self.width*row)..(self.width*row + self.width)];
            let b_wall_slice = &self.b_walls[(self.width*row)..(self.width*row + self.width)];
            buffer.push(b'|');
            buffer.extend(cell_slice
                .iter()
                .zip(r_wall_slice.iter())
                .flat_map(|(&cell, &wall)| [cell_option[cell as usize], wall_option[wall as usize]]));
            buffer.push(b'\n');
            buffer.push(b' ');
            buffer.extend(b_wall_slice.iter().flat_map(|&floor| [floor_option[floor as usize], b' ']));
            //buffer.extend_from_slice(&floor);
            buffer.push(b'\n');
        }

        // Calculate correct indices for start and end positions
        let start_index = ceiling.len() + 1 + // Account for ceiling and first newline
        (self.width * 2 + 2) * (self.start_pos.1) as usize + // Account for full rows
        1 + self.start_pos.0 as usize * 2; // Account for first '|' and position within row

        let end_index = ceiling.len() + 1 +
        (self.width * 2 + 2) * (self.end_pos.1 ) as usize + 
        1 + self.end_pos.0 as usize * 2;

        // Place start and end markers
        buffer[start_index] = b'S';
        buffer[end_index] = b'E';

        handle.write_all(&buffer).unwrap();
        handle.flush().unwrap();
    }

    pub fn gen_maze(&mut self) {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut rng = thread_rng();
        let start_pos: [usize;2]  = [rng.gen_range(0..self.width), rng.gen_range(0..self.height)];
        self.recursive_backtrack(start_pos[0], start_pos[1], &mut visited, &mut rng);
    }

    fn recursive_backtrack(&mut self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>, rng: &mut ThreadRng) {
        visited.insert((x, y));
        self.cells[y * self.width + x] = true;  // Mark as path

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut shuffled_directions = directions.to_vec();
        shuffled_directions.shuffle(rng);

        for (dx, dy) in shuffled_directions {
            let next_x = x as i32 + dx;
            let next_y = y as i32 + dy;

            //checks if the position it is trying to go it is 
            //past top, bottom, left, and right walls

            if next_x >= 0 && next_x < self.width as i32 && next_y >= 0 && next_y < self.height as i32 
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
            (0, -1) => self.b_walls[(y - 1) * self.width + x] = false,
            //RIGHT
            (1, 0) => self.r_walls[y * self.width + x] = false,
            //DOWN
            (0, 1) => self.b_walls[y * self.width + x] = false,
            //LEFT
            (-1, 0) => self.r_walls[y * self.width + x - 1] = false,
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
