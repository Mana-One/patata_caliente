use std::collections::{HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use crate::challenge::Challenge;

#[derive(Deserialize, Serialize, Debug)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MonstruousMazeChallenge {
    input: MonstrousMazeInput
}

type Coordinates = [usize; 2];
// line | column | endurance

type Row = Vec<char>;

#[derive(Debug, Clone, Copy)]
struct Node {
    pub line: usize,
    pub column: usize,
    pub endurance: u8
}

impl Node {
    fn new(line: usize, column: usize, endurance: u8) -> Self {
        Node { line, column, endurance }
    }

    fn go_north(self: &Self) -> Node {
        Node { line: self.line - 1, column: self.column, endurance: self.endurance }
    }

    fn go_east(self: &Self) -> Node {
        Node { line: self.line, column: self.column + 1, endurance: self.endurance }
    }

    fn go_south(self: &Self) -> Node {
        Node { line: self.line + 1, column: self.column, endurance: self.endurance }
    }

    fn go_west(self: &Self) -> Node {
        Node { line: self.line, column: self.column - 1, endurance: self.endurance }
    }

    fn take_damage(self: &Self) -> Node {
        Node { line: self.line, column: self.column, endurance: self.endurance - 1 }
    }
}

#[derive(Debug)]
enum Direction {
    None,
    North,
    East,
    South,
    West
}

impl Challenge for MonstruousMazeChallenge {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        "MonstruousMaze".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MonstruousMazeChallenge { input }
    }

    fn solve(&self) -> Self::Output {
        let (grid, width, height) = get_grid(&self.input.grid);
        let mut queue: VecDeque<Node> = VecDeque::new();
        let start = get_start(&grid, width, self.input.endurance);
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let mut path = get_path_map(&grid);
        let mut end = [0usize; 2];
        let valid = ['Y', ' ', 'M', 'X'];

        visited.insert([start.line, start.column]);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {

            visited.insert([node.line, node.column]);
            if grid[node.line * width + node.column] == 'X' {
                end = [node.line, node.column];
                break;
            }
            
            if node.line > 0 && 
                !visited.contains(&[node.line - 1, node.column]) && 
                valid.contains(&grid[(node.line - 1) * width + node.column]) {

                let next = grid[(node.line - 1) * width + node.column];
                if next != 'M' {
                    let next_node = node.go_north();
                    queue.push_back(next_node);
                    path[next_node.line * width + next_node.column] = Direction::North;
                    
                } else if node.endurance > 1 {
                    let next_node = node.go_north().take_damage();
                    queue.push_back(next_node);
                    path[next_node.line * width + next_node.column] = Direction::North;
                }
            }

            if node.column < width - 1 && 
                !visited.contains(&[node.line, node.column + 1]) && 
                valid.contains(&grid[node.line * width + node.column + 1]) {

                let next = grid[node.line * width + node.column + 1];
                if next != 'M' {
                    let next_node = node.go_east();
                    queue.push_back(next_node);
                    path[(width * next_node.line + next_node.column)] = Direction::East;

                } else if node.endurance > 1 {
                    let next_node = node.go_east().take_damage();
                    queue.push_back(next_node);
                    path[(width * next_node.line + next_node.column)] = Direction::East;
                }
            }

            if node.line < height - 1 && 
                !visited.contains(&[node.line + 1, node.column]) && 
                valid.contains(&grid[(node.line + 1) * width + node.column]) {

                let next = grid[(node.line + 1) * width + node.column];
                if next != 'M' {
                    let next_node = node.go_south();
                    queue.push_back(next_node);
                    path[width * next_node.line + next_node.column] = Direction::South;

                } else if node.endurance > 1 {
                    let next_node = node.go_south().take_damage();
                    queue.push_back(next_node);
                    path[width * next_node.line + next_node.column] = Direction::South;
                }
            }

            if node.column > 0 && 
                !visited.contains(&[node.line, node.column - 1]) && 
                valid.contains(&grid[node.line * width + node.column - 1]) {

                let next = grid[node.line * width + node.column - 1];
                if next != 'M' {
                    let next_node = node.go_west();
                    queue.push_back(next_node);
                    path[width * next_node.line + next_node.column] = Direction::West;

                } else if node.endurance > 1 {
                    let next_node = node.go_west().take_damage();
                    queue.push_back(next_node);
                    path[width * next_node.line + next_node.column] = Direction::West;
                }
            }
        } 

        Self::Output {
            path: compute_path(&path, width, end)
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let (grid, width, _) = get_grid(&self.input.grid);
        let start = get_start(&grid, width, self.input.endurance);

        let end = answer.path
            .chars()
            .fold(start, move |acc, x| {
                if x == '^' {
                    return acc.go_north();
                } else if x == '>' {
                    return acc.go_east();
                } else if x == 'v' {
                    return acc.go_south();
                } else {
                    return acc.go_west();
                }
            });
        grid[end.line * width + end.column] == 'X'
    }
}

fn get_grid(grid: &str) -> (Row, usize, usize){
    let grid = grid.lines()
        .map(|l| l.to_string().chars().collect::<Row>())
        .collect::<Vec<Row>>();
    let height = grid.len();
    let width = grid[0].len();

    (grid.concat(), width, height)
}

fn get_start(grid: &Row, width: usize, endurance: u8) -> Node {
    let (idx, _) = grid
        .iter()
        .enumerate()
        .find(|(_, &c)| c == 'Y').unwrap();
    Node::new(idx / width, idx % width, endurance)
}

fn get_path_map(grid: &Row) -> Vec<Direction> {
    grid.iter().map(|_| Direction::None).collect::<Vec<Direction>>()
}

fn compute_path(path_map: &Vec<Direction>, width: usize, end: Coordinates) -> String {
    let mut current_coordinates = end.clone();
    let mut res = String::from("");

    loop {
        let [line, column] = current_coordinates;
        match path_map[line * width + column] {
            Direction::None => { break; },
            Direction::North => {
                current_coordinates = [line + 1, column];
                res = format!("^{res}");
            },
            Direction::East => {
                current_coordinates = [line, column - 1];
                res = format!(">{res}");
            },
            Direction::South => {
                current_coordinates = [line - 1, column];
                res = format!("v{res}");
            },
            Direction::West => {
                current_coordinates = [line, column + 1];
                res = format!("<{res}");
            }
        }
    }

    res
}
