use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn new(row: usize, column: usize) -> Self {
        Point { row, column }
    }

    fn conflict_with_different_column(&self, other: &Point) -> bool {
        if self.row == other.row {
            return true;
        }

        let x = (other.column as isize) - (self.column as isize);
        let y = (other.row as isize) - (self.row as isize);

        if x.abs() == y.abs() {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Queen(Vec<Point>);

impl Queen {
    // fn new() -> Self {
    //     Queen(Vec::new())
    // }

    fn with_capacity(size: usize) -> Self {
        Queen(Vec::with_capacity(size))
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        let mut s = String::new();
        let len = self.0.len();
        if len != 0 {
            let mut map = HashMap::with_capacity(len);
            let mut iter = self.0.iter();
            while let Some(ref point) = iter.next() {
                let k = point.row * len + point.column;
                map.insert(k, ());
            }
            let mut i = 0;
            for _ in 0..len {
                for _ in 0..len {
                    match map.get(&i) {
                        Some(_) => s.push_str("[Q]"),
                        None => s.push_str("[ ]"),
                    }
                    i += 1;
                }
                s.push_str("\n");
            }
            // } else {
            //     for _ in 0..len {
            //         for _ in 0..len {
            //             s.push_str("[ ]");
            //         }
            //         s.push_str("\n");
            //     }
        }
        write!(f, "{}-Queens:\n{}", len, s)
    }
}

fn no_conflicts(board: &Vec<Rc<Point>>, current: &Point) -> bool {
    for p in board {
        if current.conflict_with_different_column(p) {
            return false;
        }
    }
    true
}

fn queens(board: Vec<Rc<Point>>, current_column: usize, max: usize) -> Vec<Queen> {
    let mut vec = Vec::new();
    for row in 0..=max {
        let current = Point::new(row, current_column);
        if no_conflicts(&board, &current) {
            let mut board = board.clone();
            board.push(Rc::new(current));
            if current_column < max {
                vec.append(&mut queens(board, current_column + 1, max));
            } else {
                let mut queen = Queen::with_capacity(max);
                for rc_point in board {
                    queen.0.push((*rc_point).clone());
                }
                vec.push(queen);
            }
        }
    }
    vec
}

pub fn calc_queens(size: usize) -> Vec<Queen> {
    queens(Vec::with_capacity(size), 0, size - 1)
    // let half = (((size as f64) / 2f64).ceil()) as usize;

    // let board = Vec::with_capacity(size);
    // let current_column = 0;
    // let max = size - 1;

    // let mut vec = Vec::new();
    // for row in 0..half {
    //     let current = Point::new(row, current_column);
    //     if no_conflicts(&board, &current) {
    //         let mut board = board.clone();
    //         board.push(Rc::new(current));
    //         if current_column < max {
    //             vec.append(&mut queens(board, current_column + 1, max));
    //         } else {
    //             let mut queen = Queen::with_capacity(max);
    //             for rc_point in board {
    //                 queen.0.push((*rc_point).clone());
    //             }
    //             vec.push(queen);
    //         }
    //     }
    // }
    // vec
}
