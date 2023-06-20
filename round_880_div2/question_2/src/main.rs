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
        read_vec!(data as usize);
        answers.append(&mut vec![solve(data[0], data[1], data[2])])
    }
    answers.into_iter().for_each(|ans| println!("{ans}"));
}

fn solve(n: usize, k: usize, g: usize) -> usize {
    if g % 2 == 0 {
        if (g / 2 - 1) * (n - 1) < k * g {
            return k * g - round(k * g - (n - 1) * (g / 2 - 1), g);
        } else {
            return k * g;
        }
    } else {
        if (g / 2) * (n - 1) < k * g {
            return k * g - round(k * g - (n - 1) * (g / 2), g);
        } else {
            return k * g;
        }
    }
}

fn round(x: usize, g: usize) -> usize {
    if x % g >= g / 2 {
        x + g - x % g
    } else {
        x - x % g
    }
}
