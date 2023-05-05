fn main() {
    let (n, k)= (3, 1);
    let flights = [[0,1,100], [1,2,100], [0,2,500]].to_vec();
    let (src, dest) = (0, 2);


    let res = shortest_path(&flights, n, k, src, dest);
    dbg!(res);
}

fn shortest_path(flights: &Vec<[i32;3]>, n: i32, k: i32, src: i32, dest: i32) -> i32 {

    let mut prices = Vec::new();
    for i in 0..n {
        if i == src {
            prices.push(0);
        } else {
            prices.push(i32::MAX);
        }
    }

    for _ in 0..=k {

        let mut temp = prices.clone();
        for [u,v,w] in flights.iter() {
            if let Some(val) = prices.iter().nth(*u as usize) {
                if *val < i32::MAX && *val + w < *temp.iter().nth(*v as usize).unwrap() {
                    *temp.iter_mut().nth(*v as usize).unwrap() = val + w;
                }
            }
        prices = temp.clone();
    }   
}

*prices.iter().nth(dest as usize).unwrap()
}
    
