use std::collections::HashSet;

fn main() {
    let mut grid = Vec::new();
    grid.push(vec![1,1,1,1,1,1,1,0]);
    grid.push(vec![1,0,0,0,0,1,1,0]);
    grid.push(vec![1,0,1,0,1,1,1,0]);
    grid.push(vec![1,0,0,0,0,1,0,1]);
    grid.push(vec![1,1,1,1,1,1,1,0]);

    //dbg!(&grid);

    let mut visit: HashSet<(usize, usize)> = HashSet::new();
    // visit.insert((2usize, 3usize));

    let mut closed_islands = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 && !visit.contains(&(r,c)) {
                closed_islands += dfs(&grid, r as i32, c as i32, &mut visit);
            }
        }
    }
    
    println!("closed islands = {:?}", closed_islands);
    //dbg!(visit);
}

fn dfs(grid: &Vec<Vec<i32>>, 
    r: i32, 
    c: i32, 
    visit: &mut HashSet<(usize, usize)>) -> usize {
    
    let rows = grid.len();
    let cols = grid[0].len();
    if r < 0 || c < 0 || r as usize == rows || c as usize == cols {return 0;}
    if grid[r as usize][c as usize] == 1 || visit.contains(&(r as usize,c as usize)) {return 1;}

    visit.insert((r as usize,c as usize));
    let res = dfs(grid, r+1, c, visit)*
                        dfs(grid, r-1, c, visit)*
                        dfs(grid, r, c+1, visit)*
                        dfs(grid, r, c-1, visit);
    //dbg!(&res);
    res
}
