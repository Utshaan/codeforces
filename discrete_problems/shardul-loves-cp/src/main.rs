fn main() {
    let mut array = vec![];
    array = solve(
        array,
        vec![
            vec![2, 1, 4],
            vec![1, 1],
            vec![1, 4],
            vec![1, 2],
            vec![2, 2, 4],
            vec![2, 4, 3],
            vec![1, 2],
            vec![2, 2, 7],
        ],
    );

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
