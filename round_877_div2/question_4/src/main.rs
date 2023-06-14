#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
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
    read_vec!(term_vec as u32);
    read_str!(s);
    let mut v = Vec::new();

    let mut given = String::from(s);

    for _ in 0..term_vec[1] {
        read!(q as usize);

        given = given
            .chars()
            .enumerate()
            .map(|(i, char)| {
                if i + 1 == q {
                    match char {
                        '(' => ')',
                        ')' => '(',
                        other => other,
                    }
                } else {
                    char
                }
            })
            .collect();

        println!("{} {}", given, q);

        v.append(&mut vec![solve(&given, term_vec[0])]);
    }
    for s in v {
        println!("{s}");
    }
}

fn solve(s: &str, n: u32) -> String {
    if n % 2 != 0 {
        return String::from("NO");
    }

    let mut left = 0;
    let mut right = 0;

    for par in s.chars() {
        if par == '(' {
            left += 1
        } else {
            right += 1
        }

        if left < right && (right - left) % 2 != 0 {
            return String::from("NO");
        }
    }

    return String::from("YES");
}
