use std::collections::HashMap;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main() {
    let mut array = vec![];
    read!(number_of_queries as usize);
    let mut queries: Vec<Vec<usize>> = Vec::new();
    for _ in 0..number_of_queries {
        read_vec!(query as usize);
        queries.push(query);
    }

    array = solve(array, queries);

    array.into_iter().rev().for_each(|x| print!("{x} "));
    println!();
}

fn solve(mut array: Vec<usize>, queries: Vec<Vec<usize>>) -> Vec<usize> {
    let mut transformations: HashMap<usize, usize> = HashMap::new();
    for mut query in queries.into_iter().rev() {
        match query.remove(0) {
            1 => {
                if transformations.contains_key(&query[0]) {
                    array.append(&mut vec![*transformations.get(&query[0]).unwrap()]);
                } else {
                    array.append(&mut vec![query.remove(0)]);
                }
            }
            2 => {
                let p = transformations.get(&query[1]).unwrap_or(&query[1]).clone();
                transformations
                    .entry(query[0])
                    .and_modify(|e| *e = p)
                    .or_insert(query[1]);
            }
            _ => panic!("Invalid query"),
        }
    }
    return array;
}
