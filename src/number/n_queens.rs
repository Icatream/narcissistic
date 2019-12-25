use itertools::Itertools;
use lazyonce::LazyOnce;
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

    fn score(&self, len: usize) -> usize {
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
    fn reverse_vertical(&self) -> Self {
        let max = self.0.len() - 1;
        let vec = self
            .0
            .iter()
            .map(|p| Point::new(p.row, max - p.column))
            .collect();

        Queen(vec)
    }

    //reverse at level axis
    fn reverse_level(&self) -> Self {
        let max = self.0.len() - 1;
        let vec = self
            .0
            .iter()
            .map(|p| Point::new(max - p.row, p.column))
            .collect();

        Queen(vec)
    }

    //rotate 90° left
    fn rotate_left(&self) -> Self {
        let middle = ((self.0.len() as f64) - 1f64) / 2f64;
        let vec = self
            .0
            .iter()
            .map(|p| {
                let (r, c) = point_to_float(p, middle);
                let (r, c) = (-c, r);
                float_to_point(r, c, middle)
            })
            .collect();

        Queen(vec)
    }

    //rotate 90° right
    fn rotate_right(&self) -> Self {
        let middle = ((self.0.len() as f64) - 1f64) / 2f64;
        let vec = self
            .0
            .iter()
            .map(|p| {
                let (r, c) = point_to_float(p, middle);
                let (r, c) = (c, -r);
                float_to_point(r, c, middle)
            })
            .collect();

        Queen(vec)
    }

    //rotate 180°
    fn rotate_half(&self) -> Self {
        let middle = ((self.0.len() as f64) - 1f64) / 2f64;
        let vec = self
            .0
            .iter()
            .map(|p| {
                let (r, c) = point_to_float(p, middle);
                let (r, c) = (-r, -c);
                float_to_point(r, c, middle)
            })
            .collect();

        Queen(vec)
    }

    //rotate oblique
    fn rotate_oblique_right(&self) -> Self {
        let middle = ((self.0.len() as f64) - 1f64) / 2f64;
        let vec = self
            .0
            .iter()
            .map(|p| {
                let (r, c) = point_to_float(p, middle);
                let (r, c) = (c, r);
                float_to_point(r, c, middle)
            })
            .collect();

        Queen(vec)
    }

    //rotate oblique
    fn rotate_oblique_left(&self) -> Self {
        let middle = ((self.0.len() as f64) - 1f64) / 2f64;
        let vec = self
            .0
            .iter()
            .map(|p| {
                let (r, c) = point_to_float(p, middle);
                let (r, c) = (-c, -r);
                float_to_point(r, c, middle)
            })
            .collect();

        Queen(vec)
    }
}

#[inline]
fn point_to_float(p: &Point, m: f64) -> (f64, f64) {
    ((p.row as f64) - m, (p.column as f64) - m)
}

#[inline]
fn float_to_point(r: f64, c: f64, m: f64) -> Point {
    let r = (r + m) as usize;
    let c = (c + m) as usize;
    Point::new(r, c)
}

impl PartialEq for Queen {
    fn eq(&self, other: &Self) -> bool {
        let len = self.0.len();
        if len != other.0.len() {
            return false;
        }
        let mut iter1 = self.0.iter().map(|p| p.score(len)).sorted();
        let mut iter2 = other.0.iter().map(|p| p.score(len)).sorted();
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

impl Eq for Queen {}

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
            let mut iter = self.0.iter().map(|p| p.score(len)).sorted();
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

struct VariantQueens {
    queen: Queen,
    reverse_vertical: LazyOnce<Queen>,
    reverse_level: LazyOnce<Queen>,
    rotate_left: LazyOnce<Queen>,
    rotate_right: LazyOnce<Queen>,
    rotate_half: LazyOnce<Queen>,
    rotate_oblique_left: LazyOnce<Queen>,
    rotate_oblique_right: LazyOnce<Queen>,
}

impl VariantQueens {
    fn new(queen: Queen) -> Self {
        VariantQueens {
            queen,
            reverse_vertical: LazyOnce::new(),
            reverse_level: LazyOnce::new(),
            rotate_left: LazyOnce::new(),
            rotate_right: LazyOnce::new(),
            rotate_half: LazyOnce::new(),
            rotate_oblique_left: LazyOnce::new(),
            rotate_oblique_right: LazyOnce::new(),
        }
    }

    // fn queen(&self) -> &Queen {
    //     &self.queen
    // }

    fn reverse_vertical(&self) -> &Queen {
        self.reverse_vertical.get(|| self.queen.reverse_vertical())
    }

    fn reverse_level(&self) -> &Queen {
        self.reverse_level.get(|| self.queen.reverse_level())
    }

    fn rotate_left(&self) -> &Queen {
        self.rotate_left.get(|| self.queen.rotate_left())
    }

    fn rotate_right(&self) -> &Queen {
        self.rotate_right.get(|| self.queen.rotate_right())
    }

    fn rotate_half(&self) -> &Queen {
        self.rotate_half.get(|| self.queen.rotate_half())
    }

    fn rotate_oblique_left(&self) -> &Queen {
        self.rotate_oblique_left
            .get(|| self.queen.rotate_oblique_left())
    }

    fn rotate_oblique_right(&self) -> &Queen {
        self.rotate_oblique_right
            .get(|| self.queen.rotate_oblique_right())
    }
}

impl PartialEq for VariantQueens {
    fn eq(&self, other: &Self) -> bool {
        self.queen.eq(&other.queen)
            || self.queen.eq(other.reverse_vertical())
            || self.queen.eq(other.reverse_level())
            || self.queen.eq(other.rotate_left())
            || self.queen.eq(other.rotate_right())
            || self.queen.eq(other.rotate_half())
            || self.queen.eq(other.rotate_oblique_left())
            || self.queen.eq(other.rotate_oblique_right())
    }
}

impl Eq for VariantQueens {}

pub fn unique(vec: &Vec<Queen>) -> Vec<Queen> {
    let mut iter = vec.iter().map(|queen| {
        let queen = (*queen).clone();
        VariantQueens::new(queen)
    });

    let mut list: Vec<VariantQueens> = Vec::with_capacity(vec.len());
    while let Some(q) = iter.next() {
        if !list.contains(&q) {
            list.push(q);
        }
    }

    list.into_iter().map(|q| q.queen).collect()
}
