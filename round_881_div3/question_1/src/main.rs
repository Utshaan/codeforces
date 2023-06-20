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
        read!(size as usize);
        read_vec!(a as usize);
        answers.append(&mut vec![solve(size, a)]);
    }

    answers.into_iter().for_each(|ans| println!("{ans}"));
}

fn solve(size: usize, mut a: Vec<usize>) -> usize {
    if size == 1 {
        return 0;
    } else {
        let mut ans = 0;
        a.sort();
        for i in 0..size / 2 {
            ans += a[size - i - 1] - a[i];
        }
        ans
    }
}
