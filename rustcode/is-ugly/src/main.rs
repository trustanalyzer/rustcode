fn main() {
    let n = 6u32;
    println!("{:?}", is_ugly(n));

    let n = 21u32;
    println!("{:?}", is_ugly(n));
}

fn is_ugly(mut n: u32) -> bool {

    for f in [2,3,5] {
        while n % f == 0 {
            n /= f;
        }
    }

    if n == 1 {return true;}
    false
}