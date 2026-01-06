use std::{collections::VecDeque, env, process};

use rand::{rngs::ThreadRng, seq::SliceRandom};

struct MazeGen {
    maze: Vec<Vec<u8>>,
    size: usize,
    rng: ThreadRng
}

impl MazeGen {
    fn new(size: usize) -> Self {
        Self {
            maze: vec![vec![1; size as usize];size as usize],
            size,
            rng: rand::rng()
        }
    }

    fn carve(&mut self, x: usize, y: usize) {
        self.maze[x][y] = 0;

        let mut directions : [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        directions.shuffle(&mut self.rng);

        for dir in directions {
            let nx = x as isize + dir.1 * 2;
            let ny = y as isize + dir.0 * 2;


            if nx > 0 && ny > 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < self.size && ny < self.size && self.maze[nx][ny] == 1 {
                    self.maze[(x as isize + dir.1) as usize][(y as isize + dir.0) as usize] = 0;
                    self.carve(nx, ny);
                }
            }

        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let directions : [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut neighbors = Vec::new();

        for dir in directions {
            let nx = x as isize + dir.1;
            let ny = y as isize + dir.0;

            if nx > 0 && ny > 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if self.maze[nx][ny] == 0 {
                    neighbors.push((nx, ny));
                }
            }
        }
        neighbors
    }

    fn trace_path(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let mut search : VecDeque<(usize, usize)> = VecDeque::new();        
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut prev : Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; self.size]; self.size];
        search.push_back((x1, y1));

        while let Some(node) = search.pop_front() {
            if node.0 == x2 && node.1 == y2 {
                println!("found");
                break
            }

            visited[node.0][node.1] = true;
            let neighbors = self.get_neighbors(node.0, node.1);

            for &nei in &neighbors {
                if !visited[nei.0][nei.1] {
                    prev[nei.0][nei.1] = Some((node.0, node.1));
                    search.push_back(nei);
                }
            }

        }

        let mut at = Some((x2, y2));

        while let Some(node) = at {
            self.maze[node.0][node.1] = 2;
            at = prev[node.0][node.1];
        }

    }

    fn display_maze(&self) {
        for x in &self.maze {
            for &y in x {
                if y == 1  {
                    print!("#")
                }
                else if y == 2 {
                    print!("\x1b[92m*\x1b[0m")
                }
                else {
                    print!(" ")
                }
            }
            println!();
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("mgen <maze_size>");
        process::exit(1);
    }

    let size : usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter valid number");
            process::exit(1);
        }
    };

    let mut maze = MazeGen::new(size);
    maze.carve(1, 1);
    maze.trace_path(1, 1, size-2, size-2);
    maze.display_maze();
}
