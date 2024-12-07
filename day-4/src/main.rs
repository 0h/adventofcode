use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let sample = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let sampvec = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn count_xmas(grid: &[Vec<char>]) -> usize {
        let directions = [
            (0, 1),   // Horizontal right
            (1, 0),   // Vertical down
            (1, 1),   // Diagonal down-right
            (1, -1),  // Diagonal down-left
            (0, -1),  // Horizontal left (backward)
            (-1, 0),  // Vertical up (backward)
            (-1, -1), // Diagonal up-left (backward)
            (-1, 1),  // Diagonal up-right (backward)
        ];

        let rows = grid.len();
        let cols = grid[0].len();
        let target = "XMAS";
        let target_len = target.len();

        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                for (dx, dy) in directions.iter() {
                    let mut word = String::new();

                    for step in 0..target_len {
                        let ni = i as isize + step as isize * dx;
                        let nj = j as isize + step as isize * dy;

                        if ni < 0 || nj < 0 || ni >= rows as isize || nj >= cols as isize {
                            break; // Out of bounds
                        }

                        word.push(grid[ni as usize][nj as usize]);
                    }

                    if word == target {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn count_x_mas(grid: &[Vec<char>]) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                // Check if grid[i][j] is a valid 'A' center for an X-MAS
                if grid[i][j] == 'A' {
                    let mut unique = false;

                    // [i-1][j-1]   [i-1][j]   [i-1][j+1]
                    // [i][j-1]     [i][j]     [i][j+1]
                    // [i+1][j-1]   [i+1][j]   [i+1][j+1]

                    // Check top-left M and bottom-right S
                    if grid[i - 1][j - 1] == 'M'
                        && grid[i + 1][j + 1] == 'S'
                        && grid[i - 1][j + 1] == 'S'
                        && grid[i + 1][j - 1] == 'M'
                    {
                        unique = true;
                    }

                    // Check top-right S and bottom-left M
                    if grid[i + 1][j - 1] == 'M'
                        && grid[i - 1][j + 1] == 'S'
                        && grid[i + 1][j + 1] == 'M'
                        && grid[i - 1][j - 1] == 'S'
                    {
                        unique = true;
                    }
                    if grid[i + 1][j - 1] == 'S'
                        && grid[i - 1][j + 1] == 'M'
                        && grid[i + 1][j + 1] == 'S'
                        && grid[i - 1][j - 1] == 'M'
                    {
                        unique = true;
                    }
                    if grid[i + 1][j - 1] == 'S'
                        && grid[i - 1][j + 1] == 'M'
                        && grid[i + 1][j + 1] == 'M'
                        && grid[i - 1][j - 1] == 'S'
                    {
                        unique = true;
                    }
                    // Count only if either X-MAS is formed
                    if unique {
                        count += 1;
                    }
                }
            }
        }

        count
    }
    println!("{:?}", count_x_mas(&sampvec));
}
