#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
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

    println!("{:?}", array);
}

fn solve(mut array: Vec<usize>, queries: Vec<Vec<usize>>) -> Vec<usize> {
    for mut query in queries {
        match query.remove(0) {
            1 => {
                array.append(&mut vec![query.remove(0)]);
            }
            2 => {
                array = array
                    .into_iter()
                    .map(|x| if x == query[0] { query[1] } else { x })
                    .collect();
            }
            _ => panic!("Invalid query"),
        }
    }
    return array;
}
