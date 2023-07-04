
const N: i32 = 3;

fn main() {
    let mut res: Vec<String> = Vec::new();
    backtrack(0,0, &mut res, &mut String::from(""));
    println!("{res:?}");
}

fn backtrack(open: i32, closed: i32, res: &mut Vec<String>, string: &mut String) -> () {

    if closed == N {
        res.push(string.clone());
        return ;
    }

    if open < N {
        string.push('(');
        backtrack(open+1, closed, res, string);
        string.pop();
    }

    if closed < open {
        string.push(')');
        backtrack(open, closed+1, res, string);
        string.pop();
    }
}