pub struct PascalsTriangle {
    height: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { height: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.height == 0 {
            Vec::new()
        }
        else {
            let mut triangle: Vec<Vec<u32>> = vec![vec![1]];

            for i in 1..self.height {
                let mut row = vec![1];
                let k = (i - 1) as usize;

                for j in 1..i {
                    row.push(triangle[k][(j - 1) as usize] + triangle[k][j as usize]);
                }

                row.push(1);
                triangle.push(row);
            }

            triangle
        }
    }
}
