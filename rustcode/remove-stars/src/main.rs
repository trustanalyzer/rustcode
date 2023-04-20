fn main() {
    let string = "leet**cod*e".to_owned();
    let res = remove_stars(&string);
    dbg!(res);
}

fn remove_stars(string: &String) -> String {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '*' => {
                if stack.len() > 0 {
                    stack.pop();
                }
            }
            _ => {stack.push(c);}
        }
    }

    stack.into_iter().collect()
}
