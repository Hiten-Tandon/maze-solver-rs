use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    io::Read,
};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidAlgorithm,
    InvalidCharacter,
    FileNotFound,
    MangledRows,
    StartNotFound,
    EndNotFound,
    EmptyMaze,
}

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    DFS,
    BFS,
    GreedyBestFirst,
    AStar,
}

const VALID_CHARS: &'static str = "AB█ ";
const START: char = 'A';
const END: char = 'B';
const DIRECTIONS: [(usize, usize); 4] = [
    (usize::max_value(), 0),
    (0, usize::max_value()),
    (1, 0),
    (0, 1),
];

fn is_maze_valid(maze: &[Vec<char>]) -> Result<(), Error> {
    if maze.len() == 0 {
        Err(Error::EmptyMaze)
    } else if !maze.iter().all(|row| row.len() == maze[0].len()) {
        Err(Error::MangledRows)
    } else if get_start(maze) == None {
        Err(Error::StartNotFound)
    } else if get_end(maze) == None {
        Err(Error::EndNotFound)
    } else if !maze
        .iter()
        .all(|row| row.iter().all(|&c| VALID_CHARS.contains(c)))
    {
        Err(Error::InvalidCharacter)
    } else {
        Ok(())
    }
}

fn get_start(maze: &[Vec<char>]) -> Option<(usize, usize)> {
    for (rowi, row) in maze.iter().enumerate() {
        for (coli, ele) in row.iter().copied().enumerate() {
            if ele == START {
                return Some((rowi, coli));
            }
        }
    }
    None
}

fn get_end(maze: &[Vec<char>]) -> Option<(usize, usize)> {
    for (rowi, row) in maze.iter().enumerate() {
        for (coli, ele) in row.iter().copied().enumerate() {
            if ele == END {
                return Some((rowi, coli));
            }
        }
    }
    None
}

fn dfs(
    maze: &mut [Vec<char>],
    (row, col): (usize, usize),
    display_visited: bool,
    vis: &mut HashSet<(usize, usize)>,
) -> bool {
    if row >= maze.len()
        || col >= maze[row].len()
        || maze[row][col] == '@'
        || maze[row][col] == '█'
        || vis.contains(&(row, col))
    {
        return false;
    }

    if maze[row][col] == END {
        return true;
    }

    vis.insert((row, col));

    if maze[row][col] != START {
        maze[row][col] = '@';
    }

    let res = DIRECTIONS.iter().any(|&(dx, dy)| {
        dfs(
            maze,
            (row.overflowing_add(dx).0, col.overflowing_add(dy).0),
            display_visited,
            vis,
        )
    });

    if !display_visited && maze[row][col] != START {
        maze[row][col] = ' ';
    }

    if res && maze[row][col] != START {
        maze[row][col] = '*';
    }

    res
}

fn bfs(maze: &mut [Vec<char>], start: (usize, usize), display_visited: bool) -> bool {
    let mut frontier: VecDeque<(usize, usize, Vec<(usize, usize)>)> =
        VecDeque::from([(start.0, start.1, vec![])]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some((row, col, mut path)) = frontier.pop_front() {
        if maze[row][col] == END {
            for (ri, ci) in path.into_iter().skip(1) {
                maze[ri][ci] = '*';
            }
            return true;
        }

        if display_visited && maze[row][col] != START {
            maze[row][col] = '@';
        }

        path.push((row, col));

        DIRECTIONS
            .iter()
            .copied()
            .map(|(dx, dy)| (row.overflowing_add(dx).0, col.overflowing_add(dy).0))
            .filter(|&(row, col)| {
                row < maze.len() && col < maze[row].len() && maze[row][col] != '█'
            })
            .for_each(|(row, col)| {
                if !visited.contains(&(row, col)) {
                    visited.insert((row, col));
                    frontier.push_back((row, col, path.clone()))
                }
            });
    }
    false
}
fn manhattan_dist(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    }) + (if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    })
}
fn greedy_best_first_search(
    maze: &mut [Vec<char>],
    (start_row, start_col): (usize, usize),
    (end_row, end_col): (usize, usize),
    display_visited: bool,
) -> bool {
    let mut frontier: BinaryHeap<(Reverse<usize>, usize, usize, Vec<(usize, usize)>)> =
        BinaryHeap::from([(
            Reverse(manhattan_dist((start_row, start_col), (end_row, end_col))),
            start_row,
            start_col,
            vec![],
        )]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((_, row, col, mut path)) = frontier.pop() {
        if maze[row][col] == END {
            for (ri, ci) in path.into_iter().skip(1) {
                maze[ri][ci] = '*';
            }
            return true;
        }

        if display_visited && maze[row][col] != START {
            maze[row][col] = '@';
        }

        path.push((row, col));
        DIRECTIONS
            .iter()
            .copied()
            .map(|(dx, dy)| (row.overflowing_add(dx).0, col.overflowing_add(dy).0))
            .filter(|&(row, col)| {
                row < maze.len() && col < maze[row].len() && maze[row][col] != '█'
            })
            .for_each(|(row, col)| {
                if !visited.contains(&(row, col)) {
                    visited.insert((row, col));
                    frontier.push((
                        Reverse(manhattan_dist((row, col), (end_row, end_col))),
                        row,
                        col,
                        path.clone(),
                    ))
                }
            });
    }

    false
}
fn a_star(
    maze: &mut [Vec<char>],
    (start_row, start_col): (usize, usize),
    (end_row, end_col): (usize, usize),
    display_visited: bool,
) -> bool {
    let mut frontier: BinaryHeap<(Reverse<usize>, usize, usize, Vec<(usize, usize)>)> =
        BinaryHeap::from([(
            Reverse(manhattan_dist((start_row, start_col), (end_row, end_col))),
            start_row,
            start_col,
            vec![],
        )]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((_, row, col, mut path)) = frontier.pop() {
        if maze[row][col] == END {
            for (ri, ci) in path.into_iter().skip(1) {
                maze[ri][ci] = '*';
            }
            return true;
        }

        if display_visited && maze[row][col] != START {
            maze[row][col] = '@';
        }

        path.push((row, col));
        DIRECTIONS
            .iter()
            .copied()
            .map(|(dx, dy)| (row.overflowing_add(dx).0, col.overflowing_add(dy).0))
            .filter(|&(row, col)| {
                row < maze.len() && col < maze[row].len() && maze[row][col] != '█'
            })
            .for_each(|(row, col)| {
                if !visited.contains(&(row, col)) {
                    visited.insert((row, col));
                    frontier.push((
                        Reverse(manhattan_dist((row, col), (end_row, end_col)) + path.len()),
                        row,
                        col,
                        path.clone(),
                    ))
                }
            });
    }

    false
}
fn maze_solver(maze: &mut [Vec<char>], algorithm: Algorithm, display_visited: bool) {
    let start = get_start(maze).unwrap();
    let end = get_end(maze).unwrap();

    match algorithm {
        Algorithm::DFS => _ = dfs(maze, start, display_visited, &mut HashSet::new()),
        Algorithm::BFS => _ = bfs(maze, start, display_visited),
        Algorithm::GreedyBestFirst => {
            _ = greedy_best_first_search(maze, start, end, display_visited)
        }
        Algorithm::AStar => _ = a_star(maze, start, end, display_visited),
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
        println!("Usage: maze-solver-rs <filename> <algoithm name: A*, DFS, BFS, GBFS> <display visited?>")
    }
    let mut file = std::fs::File::open(&args[0]).map_err(|_| Error::FileNotFound)?;
    let mut contents = String::new();
    let display_visited: bool = args[2].parse().unwrap();
    let _ = file.read_to_string(&mut contents);

    let mut grid = contents
        .lines()
        .map(str::chars)
        .map(|x| x.map(|c| if c == '#' { '█' } else { c }))
        .map(Iterator::collect::<Vec<char>>)
        .collect::<Vec<Vec<char>>>();

    is_maze_valid(&grid)?;
    match args[1].as_str() {
        "A*" => maze_solver(&mut grid, Algorithm::AStar, display_visited),
        "BFS" => maze_solver(&mut grid, Algorithm::BFS, display_visited),
        "DFS" => maze_solver(&mut grid, Algorithm::DFS, display_visited),
        "GBFS" => maze_solver(&mut grid, Algorithm::GreedyBestFirst, display_visited),
        _ => return Err(Error::InvalidAlgorithm),
    }
    println!(
        "{}",
        grid.into_iter()
            .map(|row| row.into_iter().collect::<String>() + "\n")
            .collect::<String>()
    );
    Ok(())
}
