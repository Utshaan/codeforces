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
        read_vec!(vars as isize);
        read_vec!(heights as isize);
        answers[i] = solve(vars, heights);
    }
    for i in 0..test_cases {
        println!("{}", answers[i]);
    }
}

fn solve(vars: Vec<isize>, heights: Vec<isize>) -> usize {
    let m = vars[1];
    let k = vars[2];
    let h = vars[3];

    return heights
        .into_iter()
        .filter(|hi| ((hi - h).abs() % k == 0 && (hi - h).abs() <= (m - 1) * k && (hi - h) != 0))
        .count();
}
