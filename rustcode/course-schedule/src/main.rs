use std::collections::{HashSet, HashMap};

fn main() {
    let n = 5;
    let prereq = vec![(0,1), (0,2), (1,3), (1,4), 
    (3,4)];
    let mut adj_list = HashMap::new();
    for i in 0..n {
        adj_list.insert(i, vec![]);
    };
    for (crs, pre) in prereq {
        adj_list.get_mut(&crs).map(|val| val.push(pre));
    };

    dbg!(&adj_list);

    let mut visited = HashSet::new();
    // let mut res = true;
    // for crs in 0..n {
    //     res &= dfs(crs, &adj_list, &mut visited);
    // };

    // dbg!(res);

    let mut output = vec![];
    for crs in 0..n {
        match ts(crs, &adj_list, &mut visited) {
            None => { 
                println!("-1");
                break;
            },
            Some(vector) => output.extend(vector),
        }
    };

    dbg!(output);
    
}

fn dfs(crs: i32, adj_list: &HashMap<i32, Vec<i32>>, visit: &mut HashSet<i32>) -> bool {
    if visit.contains(&crs) {return false;}
    if adj_list[&crs].len() == 0 {return true;}

    visit.insert(crs);
    if let Some(vector) = adj_list.get(&crs){
        for pre in vector {
            if !dfs(*pre, adj_list, visit) {return false;}
        }
    }
    visit.remove(&crs);
    true
}

fn ts(crs: i32, 
    adj_list: &HashMap<i32, Vec<i32>>, 
    visit: &mut HashSet<i32>) -> Option<Vec<i32>> {

    if visit.contains(&crs) {return None;}
    if adj_list[&crs].len() == 0 {
        return Some(vec![crs]);
    }

    visit.insert(crs);
    let mut output = vec![];
    if let Some(vector) = adj_list.get(&crs) {
        for pre in vector {
            match ts(*pre, adj_list, visit) {
                None => {return None;},
                Some(vect) => output.extend(vect),
            }
        }
    }
    visit.remove(&crs);
    output.push(crs);
    Some(output)
}