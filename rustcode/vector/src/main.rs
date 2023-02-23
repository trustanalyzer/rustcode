fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("third element is {:?}", third);

    match v.get(2) {
        Some(third) => println!("third element is {third}"),
        None => println!("there is no third element"),
    }

    let fifth = v.get(5);
    if let Some(fifth) = fifth {
        println!("fifth element is {fifth}");
    } else {
        println!("no fifth element!");
    }

    let mut v = v.clone();
    for i in &mut v {
        *i += 5;
    }
    println!("{:?}", v);


}
