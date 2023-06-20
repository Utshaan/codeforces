macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn main() {
    read!(test_cases as usize);
    let mut answers: Vec<u64> = Vec::new();

    for _ in 0..test_cases {
        read!(node as u64);
        answers.append(&mut vec![solve(node)]);
    }

    answers.into_iter().for_each(|x| println!("{x}"));
}

fn solve(mut node: u64) -> u64 {
    let mut ans = 1;
    while node != 1 {
        ans += node;
        node /= 2;
    }
    ans
}
