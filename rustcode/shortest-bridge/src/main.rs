use std::collections::{HashSet, VecDeque};

fn main() {
    let grid = [
        [0,1,0,0,0],
        [0,1,0,0,0],
        [1,1,0,0,1],
        [0,0,0,0,1],
        [0,0,0,0,1]
    ];

    //println!("{grid:?}");

    let mut visit = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid.len() {
            if grid[r][c] == 1 {
                //dbg!(r,c);
                dfs(r,c, &grid, &mut visit);
                break;
            }
            break;
        }
    }

    //println!("{visit:?}");

    let shortest_path = bfs(visit, &grid);
    println!("{shortest_path:?}");

}

fn bfs(visit: HashSet<(usize,usize)>, grid: &[[i32;5];5]) -> usize {
    let mut dist = 0usize;
    let mut queue: VecDeque<(usize, usize)> = visit.clone().into_iter().collect();

    while queue.len() != 0 {

        let size = queue.len();
        for _ in 0..size {

            //dbg!(size);
            if let Some((row,col)) = queue.pop_front() {

                if dist > 0 && grid[row][col] == 1 {return dist;}

                if row + 1 < grid.len() && !visit.contains(&(row+1, col)) {
                    queue.push_back((row+1, col));
                }
    
                if row  >= 1 && !visit.contains(&(row-1, col)) {
                    queue.push_back((row-1, col));
                }
    
                if col + 1 < grid.len() && !visit.contains(&(row, col+1)) {
                    queue.push_back((row, col+1));
                }
    
                if col  >= 1  && !visit.contains(&(row, col-1)) {
                    queue.push_back((row, col-1));
                }
    
            }
        }

        dist += 1;
    }

    dist
}

fn dfs(r: usize, c: usize, grid: &[[i32;5];5], visit: &mut HashSet<(usize, usize)>) -> () {

    let mut stack = vec![(r,c)];
    while stack.len() != 0 {
        //dbg!(&stack);
        if let Some ((row, col)) = stack.pop() {
            
            if row + 1 < grid.len() && !visit.contains(&(row+1, col)) && grid[row+1][col] == 1 {
                stack.push((row+1, col));
            }

            if row  >= 1 && !visit.contains(&(row-1, col)) && grid[row-1][col] == 1 {
                stack.push((row-1, col));
            }

            if col + 1 < grid.len() && !visit.contains(&(row, col+1)) && grid[row][col+1] == 1 {
                stack.push((row, col+1));
            }

            if col  >= 1 && !visit.contains(&(row, col-1)) && grid[row][col-1] == 1 {
                stack.push((row, col-1));
            }

            visit.insert((row, col));

        }
    }

}


