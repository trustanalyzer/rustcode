
#[derive(Debug, Default)]
struct Node {
    val: i32,
    is_leaf: bool,
    top_left: Option<Box<Node>>,
    top_right: Option<Box<Node>>,
    bottom_left: Option<Box<Node>>,
    bottom_right: Option<Box<Node>>,
}

fn main() {
    let mut grid = Vec::new();
    grid.push([1,1,1,1,0,0,0,0].to_vec());
    grid.push([1,1,1,1,0,0,0,0].to_vec());
    grid.push([1,1,1,1,1,1,1,1].to_vec());
    grid.push([1,1,1,1,1,1,1,1].to_vec());
    grid.push([1,1,1,1,0,0,0,0].to_vec());
    grid.push([1,1,1,1,0,0,0,0].to_vec());
    grid.push([1,1,1,1,0,0,0,0].to_vec());
    grid.push([1,1,1,1,0,0,0,1].to_vec());

    // dbg!(grid);
    let tree = dfs(&grid, grid.len(), 0, 0);
    dbg!(tree);
}

fn dfs(grid: &Vec<Vec<i32>>, n: usize, r: usize, c: usize) -> Option<Box<Node>> {
    
    let mut all_same = true;
    for i in 0..n {
        for j in 0..n {
            if grid[r][c] != grid[r+i][c+j] {
                all_same = false;
                break;
            }
        }
    }

    if all_same {
        return Some(Box::new(Node {
            val: grid[r][c],
            is_leaf: true,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }));
    }

    let top_left = dfs(grid, n/2, r, c);
    let top_right = dfs(grid, n/2, r, c+n/2);
    let bottom_left = dfs(grid, n/2, r+n/2, c);
    let bottom_right = dfs(grid, n/2, r+n/2, c+n/2);
    Some(Box::new(Node{val:0, is_leaf:true, top_left, top_right, bottom_left, bottom_right}))
}
