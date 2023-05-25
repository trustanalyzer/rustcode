const N: usize = 50;

fn main() {
    let mut matrix = [[0u32; N]; N];

    let (mut left, mut right) = (0usize, N-1);
    let (mut top, mut bottom) = (0usize, N-1);

    let mut val = 1u32;
    while left <= right {

        for c in left..=right {
            matrix[top][c] = val;
            val += 1;
        }
        top += 1;

        for r in top..=bottom {
            matrix[r][right] = val;
            val += 1;
        }
        right -= 1;

        for c in (left..=right).rev() {
            matrix[bottom][c] = val;
            val += 1;
        }
        bottom -= 1;

        for r in (top..=bottom).rev() {
            matrix[r][left] = val;
            val += 1;
        }
        left += 1;
    }

    println!("{:#?}", matrix);
}

