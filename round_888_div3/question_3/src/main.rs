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
        read_vec!(vars as usize);
        read_vec!(colors as usize);

        answers[i] = solve(vars, colors);
    }

    for ans in answers {
        match ans {
            0 => println!("YES"),
            _ => println!("NO"),
        }
    }
}

fn solve(vars: Vec<usize>, mut colors: Vec<usize>) -> usize {
    let k = vars[1];

    let first = colors[0];
    let last = colors[vars[0] - 1];

    let mut first_count: usize = 0;
    let mut last_count: usize = 0;

    colors.reverse();

    while first_count < k {
        match colors.pop() {
            Some(i) => match i {
                k if k == first => first_count += 1,
                _ => continue,
            },
            None => return 1,
        }
    }
    if first != last {
        while last_count < k {
            match colors.pop() {
                Some(i) => match i {
                    k if k == last => last_count += 1,
                    _ => continue,
                },
                None => return 1,
            }
        }
    }

    return 0;
}
