use std::cmp::max;

fn main() {
    let heights = vec![2,1,5,6,2,3];

    let max_area = max_area(&heights);
    println!("{max_area:?}");
}

fn max_area(heights: &Vec<i32>) -> i32 {

    let mut stack = Vec::new();
    let mut max_area = 0;

    for (i, h) in heights.iter().enumerate() {
        
        let mut start_index = i;
        while let Some((start, height)) = stack.pop() {
                
                if height > *h {

                    max_area = max(max_area, height * (i as i32 - start as i32));
                    start_index = start;

                } else {

                    stack.push((start, height));
                    break;

                }
                
        }

        stack.push((start_index, *h));

    }

    for (i, h) in stack {
        max_area = max(max_area, h*(heights.len() as i32 - i as i32));
    }

    max_area

}
