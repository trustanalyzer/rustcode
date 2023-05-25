use std::collections::{VecDeque, HashSet};
use std::cmp::{min, max};

fn main() {
    let grid = [
        [1,0,1],
        [0,0,0],
        [1,0,1]];

    let N = grid.len();
    let mut q = VecDeque::new();
    for r in 0..N {
        for c in 0..N {
            if grid[r][c]==1 {
                q.push_back((r,c,0usize));
            }
        }
    }

    dbg!(&q);

    if q.len() == 0 || q.len() == N*N {
        println!("-1");
    }

    let mut visited = HashSet::new();
    while q.len() > 0 {
        
        dbg!(&q, &visited);
        if let Some((r,c, dist)) = q.pop_front() {
                visited.insert((r,c));
                dbg!(dist);

                if c + 1 < N && !visited.contains(&(r, c+1)) && grid[r][c+1]==0 {
                    q.push_back((r, c+1, dist+1));
                }

                if c > 1 && !visited.contains(&(r, c-1)) && grid[r][c-1]==0 {
                    q.push_back((r, c-1, dist+1));
                }

                if r + 1 < N && !visited.contains(&(r+1, c)) && grid[r+1][c]==0 {
                    q.push_back((r+1, c, dist+1));
                }

                if r > 1 && !visited.contains(&(r-1, c)) && grid[r-1][c]==0 {
                    q.push_back((r-1, c, dist+1));
                }
            }
        }
    }
