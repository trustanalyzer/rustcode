use::std::collections::{HashMap};

fn main() {
    let root = ["1","2","3","4","null","2","4","null","null","4"].to_vec();
    let trees: Vec<Option<i32>>  = root
    .into_iter()
    .map(|e| {
        match e.parse() {
            Ok(i) => Some(i),
            Err(_) => None
        }
    })
    .collect();
    
    println!("{trees:?}");

    //let mut serialization_map = HashMap::new();
    for i in 0..trees.len() {
        if let Some(n) = trees[i]{
            let mut ser = n.to_string();

            let mut c = i;
            while c < trees.len(){
                if 2*c+1 < trees.len() {
                    match trees[2*c+1] {
                        Some(c1) => ser.push_str(&c1.to_string()),
                        None => ser.push_str("null"),
                        };
                }

                if 2*c+2 < trees.len() {
                    match trees[2*c+2] {
                        Some(c2) => ser.push_str(&c2.to_string()),
                        None => ser.push_str("null"),
                    };
                };

                c = 2*c+1;
            }
        }
    }

}
    }
}
