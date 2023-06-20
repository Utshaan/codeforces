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
    read!(test_cases as i64);
    let mut answers: Vec<(i64, i64)> = Vec::new();

    for _ in 0..test_cases {
        read!(_size as i64);
        read_vec!(a as i64);
        answers.append(&mut vec![solve(a)]);
    }

    answers
        .into_iter()
        .for_each(|(sum, ops)| println!("{sum} {ops}"));
}

fn solve(mut a: Vec<i64>) -> (i64, i64) {
    a.append(&mut vec![1]);
    a = a.into_iter().filter(|&x| x != 0).collect();
    a.pop();
    if a.len() == 0 {
        return (0, 0);
    }
    let mut groups = 1;
    let start = a[0] > 0;
    for (i, &ele) in a.iter().enumerate() {
        if i != 0 {
            if ele.is_positive() != a[i - 1].is_positive() {
                groups += 1;
            }
        }
    }
    (
        a.into_iter().map(|x| x.abs()).sum(),
        if start {
            groups / 2
        } else {
            groups - groups / 2
        },
    )
}
