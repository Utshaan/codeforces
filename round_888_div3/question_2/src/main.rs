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
    let mut answers = vec![0; test_cases];

    for i in 0..test_cases {
        read!(n as usize);
        read_vec!(arr as usize);
        answers[i] = solve(n, arr);
    }

    for ans in answers {
        match ans {
            0 => println!("YES"),
            _ => println!("NO"),
        }
    }
}

fn solve(n: usize, arr: Vec<usize>) -> usize {
    if n == 1 {
        return 0;
    }
    let mut odds = Vec::new();
    let mut even = Vec::new();

    for i in arr.iter() {
        if i % 2 == 0 {
            even.push(i);
        } else {
            odds.push(i);
        }
    }

    odds.sort();
    even.sort();

    odds.reverse();
    even.reverse();

    let arr: Vec<&usize> = arr
        .iter()
        .map(|x| {
            if x % 2 == 0 {
                even.pop().unwrap()
            } else {
                odds.pop().unwrap()
            }
        })
        .collect();

    let mut greatest = &0;

    for i in arr {
        if i >= greatest {
            greatest = i;
        } else {
            return 1;
        }
    }
    return 0;
}
