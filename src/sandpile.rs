use std::ops::Add;

#[derive(PartialEq, Debug)]
pub struct Sandpile {
    cols: i32,
    rows: i32,
    data: Vec<Vec<i32>>,
}
            
impl Default for Sandpile {
    fn default() -> Sandpile {
        Sandpile {
            cols: 3,
            rows: 3,
            data: vec![vec![0; 3]; 3]
        }
    } 
}

impl Sandpile {
    fn new(cols: i32, rows: i32, data: Option<Vec<i32>>) -> Sandpile {
        match data {
            None => {
                let temp_data: Vec<Vec<i32>> = vec![vec![0; cols as usize]; rows as usize];
                Sandpile {
                    rows: rows,
                    cols: cols,
                    data: temp_data
                }
            },
            Some(x) => {
                if x.len() != (cols*rows) as usize {
                    panic!("Data does not match Sandpile dimensions.");
                } else {
                    let mut temp_data: Vec<Vec<i32>> = vec![vec![0; cols as usize]; rows as usize];
                    let mut counter: usize = 0;
                    for i in 0..rows as usize {
                        for j in 0..cols as usize {
                            temp_data[i][j] = x[counter];
                            counter += 1;
                        }
                    }

                    // #NOYOLO
                    temp_data.stabilize();

                    Sandpile {
                        rows: rows,
                        cols: cols,
                        data: temp_data
                    }
                }
            }
        }
    }

    fn is_add_compatible(&self, other: &Self) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            true
        } else {
            false
        }
    }

    fn stabilize(&mut self) -> &Self {
        self
    }
}

impl Add for Sandpile {
    type Output = Sandpile;

    fn add(self, other: Sandpile) -> Sandpile {
        if self.is_add_compatible(&other) {
            let mut a = Sandpile::new(self.rows, self.cols, None);
            
            for i in 0..self.rows as usize {
                for j in 0..self.cols as usize {
                    a.data[i][j] = self.data[i][j] + other.data[i][j];
                }
            }

            a.stabilize();
            
            a
        } else {
            panic!("The two sandpiles are not compatible for addition!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sandpile;

    #[test]
    fn test_equality() {
        let a = Sandpile::new(3, 3, None);
        let b = Sandpile::new(3, 3, None);

        assert_eq!(a, b)
    }

    #[test]
    fn test_data_initialization() {
        let a = Sandpile {
            rows: 3,
            cols: 3,
            data: vec![vec![1,1,1], vec![1,2,1], vec![2,2,2]]
        };

       let b = Sandpile::new(3, 3, Some(vec![1,1,1,1,2,1,2,2,2]));

       assert_eq!(a,b)
    }

}
