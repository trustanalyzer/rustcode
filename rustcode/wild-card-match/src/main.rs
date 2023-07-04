fn main() {
    let s = String::from("aab");
    let p = String::from("c*a*b");

    let res = dfs(0, 0, &s, &p);
    println!("{res:?}");

}

fn dfs(i: usize, j: usize, s: &String, p: &String) -> bool {

    if i >= s.len() && j >= p.len() {
        return true;
    }

    if j >= p.len() {
        return false;
    }

    if s.chars().nth(i) == p.chars().nth(j) || p.chars().nth(j) == Some('.') {
        return dfs(i+1, j, s, p);
    }
    
    if p.chars().nth(j+1) == Some('*') {
        return dfs(i, j+2, s, p);
    }

    true
}
