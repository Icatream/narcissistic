#[derive(Debug)]
pub struct ListedNumber {
    pub vec: Vec<usize>,
}

impl ListedNumber {
    pub fn new(i: usize) -> ListedNumber {
        let mut vec: Vec<usize> = Vec::new();
        if i == 0 {
            vec.push(0);
        } else {
            let mut i = i;
            while i != 0 {
                let j = i / 10;
                if j != 0 {
                    vec.push(i - j * 10);
                    i = j;
                } else {
                    vec.push(i);
                    break;
                }
            }
        }
        ListedNumber { vec }
    }

    pub fn plus_one(&mut self) {
        self.plus_one_0(0);
    }

    fn plus_one_0(&mut self, index: usize) {
        match self.vec.get(index) {
            Some(v) => {
                let v = *v + 1;
                if v < 10 {
                    self.vec[index] = v;
                } else {
                    let x = v - 10;
                    self.vec[index] = x;
                    self.plus_one_0(index + 1);
                }
            }
            None => {
                self.vec.push(1);
            }
        }
    }

    pub fn value(&self) -> usize {
        self.vec.iter()
            .enumerate()
            .map(|(i, v)| 10_usize.pow(i as u32) * v)
            .sum()
    }

    /*pub fn number_of_digit(&self, digit: usize) -> Option<usize> {
        self.vec.get(digit - 1)
            .map(|x| *x)
    }*/
}

/*
impl Display for ListedNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.vec)
    }
}*/
