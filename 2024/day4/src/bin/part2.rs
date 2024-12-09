use day4::reading;

type Kernel = Vec<Vec<Option<u8>>>;

fn main() {
    let kernel: Kernel = vec![
        vec![Some('M' as u8), None, Some('S' as u8)],
        vec![None, Some('A' as u8), None],
        vec![Some('M' as u8), None, Some('S' as u8)],
    ];

    let image = reading::read_as_matrix();

    let count: u64 = kernel
        .all_rotation()
        .iter()
        .map(|x| convolute_match(&x, &image))
        .sum();
    println!("Result: {}", count);
}

trait Rotate
where
    Self: Sized,
{
    fn rotate(&self) -> Self;

    fn all_rotation(self) -> Vec<Self> {
        vec![
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self,
        ]
    }
}

impl Rotate for Kernel {
    fn rotate(&self) -> Self {
        let mut rotated = vec![vec![None; self.len()]; self[0].len()];
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                rotated[j][self.len() - i - 1] = self[i][j];
            }
        }
        rotated
    }
}

fn kernel_match(kernel: &Kernel, image: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    for (k, row) in kernel.iter().enumerate() {
        for (l, &cell) in row.iter().enumerate() {
            if let Some(cell) = cell {
                if image.get(i + k).and_then(|row| row.get(j + l)) != Some(&cell) {
                    return false;
                }
            }
        }
    }
    true
}

fn convolute_match(kernel: &Kernel, image: &Vec<Vec<u8>>) -> u64 {
    let mut count = 0;
    for i in 0..image.len() - kernel.len() + 1 {
        for j in 0..image[0].len() - kernel[0].len() + 1 {
            if kernel_match(kernel, image, i, j) {
                count += 1;
            }
        }
    }
    count
}
