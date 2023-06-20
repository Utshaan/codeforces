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
    read!(test_cases as usize);
    let mut answers: Vec<usize> = Vec::new();

    for _ in 0..test_cases {
        read!(number_of_nodes as usize);

        let mut node_count: HashMap<usize, usize> = HashMap::new();
        let mut uncounted: Vec<(usize, usize)> = Vec::new();

        for _ in 1..number_of_nodes {
            read_vec!(node as usize);
            if node[0] > node[1] {
                uncounted.append(&mut vec![(node[1], node[0])]);
            } else {
                uncounted.append(&mut vec![(node[0], node[1])]);
            }
        }

        node_count = count(uncounted, node_count);

        println!("{:?}", node_count);

        read!(q as usize);

        let mut queries: Vec<(usize, usize)> = Vec::new();

        for _ in 0..q {
            read_vec!(query as usize);
            if query[0] > query[1] {
                queries.append(&mut vec![(query[1], query[0])]);
            } else {
                queries.append(&mut vec![(query[0], query[1])]);
            }
        }
    }
}

fn count(
    vec_of_branches: Vec<(usize, usize)>,
    mut map: HashMap<usize, usize>,
) -> HashMap<usize, usize> {
    println!("{:?}", vec_of_branches);
    for (lesser, more) in vec_of_branches.into_iter().rev() {
        if map.contains_key(&lesser) && !map.contains_key(&more) {
            map.insert(more, 1);
        } else if map.contains_key(&lesser) {
            for i in 1..=lesser {
                if map.contains_key(&i) {
                    map.entry(i).and_modify(|e| *e += 1);
                    println!("{i} = {:?}", map.get(&i));
                } else {
                    map.insert(i, 1);
                }
            }
        } else {
            map.insert(lesser, 1);
        }
        if !map.contains_key(&more) {
            map.insert(more, 1);
        }
    }
    map
}
