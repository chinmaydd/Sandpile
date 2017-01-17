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

                    let mut ret = Sandpile {
                        rows: rows,
                        cols: cols,
                        data: temp_data
                    };

                    ret = ret.stabilize();

                    ret
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

    fn topple_pass(mut self, job_queue: Vec<(usize, usize)>) -> Self {
        let y: usize = (self.cols - 1) as usize;
        let x: usize = (self.rows - 1) as usize;

        for (i, j) in job_queue {
            self.data[i][j] = self.data[i][j] - 4;
            if i == 0 && j == 0 {
                // Left corner
                self.data[0][1] += 1;
                self.data[1][0] += 1;
            } else if i == 0 && j == y {
                // Right corner
                self.data[0][j-1] += 1;
                self.data[1][j] += 1;
            } else if i == x && j == 0 {
                // Bottom left
                self.data[i-1][0] += 1;
                self.data[i][1] += 1;
            } else if i == x && j == y {
                // Bottom right
                self.data[i-1][j] += 1;
                self.data[i][j-1] += 1;
            } else if i == 0 {
                // First row
                self.data[0][j-1] += 1;
                self.data[0][j+1] += 1;
                self.data[1][j] += 1;
            } else if j == 0 {
                // First collumn
                self.data[i-1][0] += 1;
                self.data[i+1][0] += 1;
                self.data[i][1] += 1;
            } else if i == x {
                // Last row
                self.data[i][j-1] += 1;
                self.data[i][j+1] += 1;
                self.data[i-1][j] += 1;
            } else if j == y {
                // Last collumn
                self.data[i-1][j] += 1;
                self.data[i+1][j] += 1;
                self.data[i][j-1] += 1;
            } else {
                self.data[i-1][j] += 1;
                self.data[i+1][j] += 1;
                self.data[i][j-1] += 1;
                self.data[i][j+1] += 1;
            }
        }
        
        self
    }

    fn get_job_queue(&self) -> Vec<(usize, usize)> {
        let mut job_queue: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
                if self.data[i][j] > 3 {
                    job_queue.push((i,j));
                }
            }
        }

        job_queue
    }

    fn stabilize(mut self) -> Self {
        let mut job_queue = self.get_job_queue();
        
        while !job_queue.is_empty() {
            self = self.topple_pass(job_queue);
            job_queue = self.get_job_queue();
        }
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

            a.stabilize()
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

    #[test]
    fn stabilization_test_1() {
        let a = Sandpile {
            rows: 3,
            cols: 3,
            data: vec![vec![4,4,4], vec![0,0,0], vec![1,1,1]]
        };

        let b = Sandpile {
            rows: 3,
            cols: 3,
            data: vec![vec![1,2,1], vec![1,1,1], vec![1,1,1]]
        };

        assert_eq!(a.stabilize(), b);
    }

    #[test]
    fn stabilization_test_2() {
         let a = Sandpile {
            rows: 3,
            cols: 3,
            data: vec![vec![4,4,4], vec![4,4,4], vec![2,2,2]]
        };

        let b = Sandpile {
            rows: 3,
            cols: 3,
            data: vec![vec![2,3,2], vec![2,3,2], vec![3,3,3]]
        };

        assert_eq!(a.stabilize(), b);
    }

}
