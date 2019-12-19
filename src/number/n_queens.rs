use itertools::Itertools;
use lazyonce::LazyOnce;
use std::cmp::Ordering;
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

    fn count(&self, len: usize) -> usize {
        self.row * len + self.column
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

    ///reverse at vertical axis
    fn reverse(&self, len: usize) -> Self {
        let vec = self
            .0
            .iter()
            .map(|ref p| Point::new(p.row, len - p.column))
            .collect();

        Queen(vec)
    }

    ///rotate 90° left
    fn rotate(&self, len: usize) -> Self {
        let middle = len as isize;
        let vec = self
            .0
            .iter()
            .map(|ref p| {
                let r = middle - ((p.row * 2) as isize);
                let c = middle - ((p.column * 2) as isize);
                let (r, c) = (-1 * c, r);
                let r = (middle + r) as usize;
                let c = (middle + c) as usize;
                Point::new(r, c)
            })
            .collect();

        Queen(vec)
    }

    fn eq(&self, other: &Self, len: usize) -> bool {
        let mut iter1 = self.0.iter().map(|p| p.count(len)).sorted();
        let mut iter2 = other.0.iter().map(|p| p.count(len)).sorted();
        while let Some(c1) = iter1.next() {
            let c2 = iter2.next();
            match c2 {
                Some(c2) if c1 == c2 => (),
                _ => return false,
            }
        }
        match iter2.next() {
            Some(_) => return false,
            None => return true,
        }
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        let mut s = String::new();
        let len = self.0.len();
        if len != 0 {
            // let mut map = HashMap::with_capacity(len);
            // let mut iter = self.0.iter();
            // while let Some(ref point) = iter.next() {
            //     let k = point.row * len + point.column;
            //     map.insert(k, ());
            // }
            // let mut i = 0;
            // for _ in 0..len {
            //     for _ in 0..len {
            //         match map.get(&i) {
            //             Some(_) => s.push_str("[Q]"),
            //             None => s.push_str("[ ]"),
            //         }
            //         i += 1;
            //     }
            //     s.push_str("\n");
            // }
            let mut iter = self.0.iter().map(|p| p.count(len)).sorted();
            let mut p_index = iter.next();
            let mut i = 0;
            for _ in 0..len {
                for _ in 0..len {
                    match p_index {
                        Some(p) if p == i => {
                            s.push_str("[Q]");
                            p_index = iter.next();
                        }
                        _ => s.push_str("[ ]"),
                    }
                    i += 1;
                }
                s.push_str("\n");
            }
        }
        write!(f, "{}-Queens:\n{}", len, s)
    }
}

impl Clone for Queen {
    fn clone(&self) -> Self {
        Queen(self.0.clone())
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

pub fn fundamental(vec: &Vec<Queen>, len: usize) -> Vec<Queen> {
    let mut vec: Vec<TransformedQueen> = vec
        .iter()
        .map(|queen| {
            let queen = (*queen).clone();
            TransformedQueen::new(queen, len)
        })
        .sorted()
        .collect();
    vec.dedup();

    unimplemented!()
}

struct TransformedQueen {
    len: usize,
    queen: Queen,
    reverse: LazyOnce<Queen>,
    rotate: LazyOnce<Queen>,
}

impl TransformedQueen {
    fn new(queen: Queen, len: usize) -> Self {
        TransformedQueen {
            len,
            queen,
            reverse: LazyOnce::new(),
            rotate: LazyOnce::new(),
        }
    }

    fn queen(&self) -> &Queen {
        &self.queen
    }

    fn reversed(&self) -> &Queen {
        self.reverse.get(|| self.queen.reverse(self.len))
    }

    fn rotated(&self) -> &Queen {
        self.rotate.get(|| self.queen.rotate(self.len))
    }
}

impl PartialOrd for TransformedQueen {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len < other.len {
            return Some(Ordering::Less);
        } else if self.len > other.len {
            return Some(Ordering::Greater);
        }

        unimplemented!()
    }
}

impl PartialEq for TransformedQueen {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
            && (self.queen.eq(&other.queen, self.len)
                || self.queen.eq(other.reversed(), self.len)
                || self.queen.eq(other.rotated(), self.len))
    }
}

impl Eq for TransformedQueen {}

impl Ord for TransformedQueen {
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}
