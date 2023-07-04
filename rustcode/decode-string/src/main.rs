

fn main() {
    let s1 = String::from("3[a]2[bc]");
    let s2 = String::from("3[a2[c]]");

    println!("{}", decode_string(&s1));
    println!("{}", decode_string(&s2));
}

fn decode_string(s: &String) -> String {

    let mut stack = Vec::new();
    for c in s.chars() {

        //dbg!(c, &stack);
        if c != ']' {
            stack.push(c.to_string());
        
        } else {
            
            let mut substr = String::new();
            while let Some(mut s) = stack.pop() {
                if s.as_str() == "[" {
                    break;
                }
                s.push_str(&substr);
                substr = s;
                }

            let mut multiplier = String::new();
            while let Some(mut s) = stack.pop() {
                if s.parse::<i32>().is_ok() {
                    s.push_str(&multiplier);
                    multiplier = s;
                } else {
                    stack.push(s);
                    break;
                }
            }

            let mut new_string = String::from("");
            for _ in 0..multiplier.parse::<usize>().unwrap() {
                new_string.push_str(&substr);
            }

            stack.push(new_string);

        }
    }

    //println!("{stack:?}");

    stack.join("")

}
