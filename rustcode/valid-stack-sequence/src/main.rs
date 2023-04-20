fn main() {
    let pushed = vec![1,2,3,4,5];
    let popped = vec![4,5,3,2,1];
    let res = is_valid(&pushed, &popped);
    dbg!(res);

}

fn is_valid(pushed: &Vec<i32>, popped: &Vec<i32>) -> bool {
    
    let mut stack = Vec::new();
    let mut i = 0;
    for num in pushed {
        
        stack.push(*num);
        while !stack.is_empty() && i < popped.len() &&  stack[stack.len()-1] == popped[i]{
            // dbg!(&stack,i);
            stack.pop();
            i += 1;
        };

    }
    stack.is_empty()
}
