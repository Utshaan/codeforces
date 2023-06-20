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
    let mut ans: Vec<String> = Vec::new();
    read!(test_cases as usize);
    for _ in 0..test_cases {
        read!(n as usize);
        read_vec!(pos as usize);
        ans.append(&mut vec![solve(n, pos)]);
    }
    ans.into_iter().for_each(|response| println!("{response}"));
}

fn solve(n: usize, pos: Vec<usize>) -> String {
    let mut resolve = 0;
    let mut count: HashMap<usize, usize> = HashMap::new();
    let mut max = 0;
    for p in pos.iter() {
        if *p > max {
            max = *p;
        }
        count.entry(*p).and_modify(|e| *e += 1).or_insert(1);
    }

    for i in 0..max {
        if i != 0 {
            if !count.contains_key(&i) || count.get(&i) > count.get(&(i - 1)) {
                resolve = 1;
            }
        } else if !count.contains_key(&i) {
            return "NO".to_owned();
        }
    }

    if resolve == 0 && n == pos.len() {
        return "YES".to_owned();
    } else {
        return "NO".to_owned();
    }
}
