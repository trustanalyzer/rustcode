use std::collections::HashSet;

fn main() {

    let num_1 = 19;
    println!("{:?}", is_happy(num_1));

    let num_2 = 2;
    println!("{:?}", is_happy(num_2));
}

fn is_happy(mut num: u32) -> bool {

    let mut visited = HashSet::new();
    while !visited.contains(&num) {
        let temp = square_sum_digits(num);
        visited.insert(num);
        num = temp;
        //dbg!(&visited);
    }
    if num == 1 {return true;}

    false
}

fn square_sum_digits(mut n: u32) -> u32 {

    let mut res = 0u32;
    while n != 0 {
        res += (n%10).pow(2);
        n /= 10;

        //dbg!(&n, &res);
    }

    res
}
