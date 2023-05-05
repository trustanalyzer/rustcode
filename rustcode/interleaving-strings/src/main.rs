fn main() {
    let mut ctr: usize = 0;
    let string_1 = String::from("aabcc");
    let string_2 = String::from("dbbca");
    let string_3 = String::from("aadbbcbcac");

    let res = dfs(&string_1, &string_2, &string_3, 0, 0, &mut ctr);
    dbg!(res, ctr);

}

fn dfs(string_1: &String, 
    string_2: &String, 
    string_3: &String,
    i: usize, 
    j: usize, 
    ctr: &mut usize) -> bool {

        *ctr += 1usize;
        let k = i + j;
        if k == string_3.len() {return true;}
        
        if string_1.chars().nth(i) != string_3.chars().nth(k) 
        && string_2.chars().nth(j) != string_3.chars().nth(k) {
            return false;
        }

        dfs(string_1, string_2, string_3, i+1, j, ctr) || dfs(string_1, string_2, string_3, i, j+1, ctr)

    }


