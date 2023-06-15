use std::collections::HashSet;

fn main() {
    let mut res = Vec::new();
    let (mut col, mut pos_diag, mut neg_diag) = 
    (HashSet::new(), HashSet::new(), HashSet::new());
    backtrack(0, 4, &mut res, &mut col, &mut pos_diag, &mut neg_diag);

    println!("{res:?}");
    //backtrack_iterative(4);
}

fn backtrack(r: i32, n: i32, res: &mut Vec<String>, 
    col: &mut HashSet<i32>, pos_diag: &mut HashSet<i32>, 
    neg_diag: &mut HashSet<i32>) -> () {
    
    if r == n {
        res.push("success".to_string());
    } else {

    for c in 0..n {
        //dbg!(&r, &c);
        if col.contains(&c) || pos_diag.contains(&(&r+&c)) || neg_diag.contains(&(&r-&c)) {
            continue;
        }

        col.insert(c);
        pos_diag.insert(r+c);
        neg_diag.insert(r-c);
        backtrack(r+1, n, res, col, pos_diag, neg_diag);
        col.remove(&c);
        pos_diag.remove(&(&r+&c));
        neg_diag.remove(&(&r-&c));

    }
}

}

fn backtrack_iterative(n: usize) -> () {

    let mut stack = Vec::new();
    for c in 0..n as i32 {
        stack.push((0,c));
    } 

    let mut path = Vec::new();
    let mut visited_col = HashSet::new();
    let mut visited_pos_diag = HashSet::new();
    let mut visited_neg_diag = HashSet::new();
    
    while stack.len() > 0 {
        if let Some((r, c)) = stack.pop() {
            
            dbg!(&path);
            if r == n as i32 {
                println!("{path:?}");
                path.clear();
                visited_col.clear();
                visited_neg_diag.clear();
                visited_pos_diag.clear();
                break;
            }

            visited_col.insert(c);
            visited_pos_diag.insert(r+c);
            visited_neg_diag.insert(r-c);
            path.push((r, c));
            for c in 0..n as i32 {
                if !visited_col.contains(&c) && 
                !visited_pos_diag.contains(&(r+1+c)) &&
                !visited_neg_diag.contains(&(r+1-c))
                {
                    stack.push((r+1, c));
                }
            }    
        }
        //dbg!(&path);
    }
}
