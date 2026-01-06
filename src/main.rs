use rand::{rngs::ThreadRng, seq::SliceRandom};

struct MazeGen {
    maze: Vec<Vec<bool>>,
    size: usize,
    rng: ThreadRng
}

impl MazeGen {
    fn new(size: usize) -> Self {
        Self {
            maze: vec![vec![true; size as usize];size as usize],
            size,
            rng: rand::rng()
        }
    }

    fn carve(&mut self, x: usize, y: usize) {
        self.maze[x][y] = false;

        let mut directions : [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        directions.shuffle(&mut self.rng);

        for dir in directions {
            let nx = x as isize + dir.1 * 2;
            let ny = y as isize + dir.0 * 2;


            if nx > 0 && ny > 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < self.size && ny < self.size && self.maze[nx][ny] {
                    self.maze[(x as isize + dir.1) as usize][(y as isize + dir.0) as usize] = false;
                    self.carve(nx, ny);
                }
            }

        }
    }

    fn display_maze(&self) {
        for x in &self.maze {
            for &y in x {
                if y  {
                    print!("#")
                }
                else {
                    print!("*")
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut maze = MazeGen::new(99);
    maze.carve(1, 1);
    maze.display_maze();
}
