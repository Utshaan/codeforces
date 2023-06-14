fn main() {
    println!("{:?}", solve(7, vec![1, 2, 0, 4, 0, 5, 6]));
    println!("{:?}", solve(10, vec![0, 4, 0, 1, 2, 0, 4, 0, 5, 6]));
}

fn solve(size: usize, list: Vec<usize>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..size).collect();
    let mut ans: Vec<usize> = vec![0; size];
    let mut rev = false;
    for num in list.into_iter().rev() {
        match rev {
            true => {
                let i = indices.remove(0);
                ans.remove(i);
                ans.insert(i, num);
            }
            false => {
                let i = indices.pop().unwrap();
                ans.remove(i);
                ans.insert(i, num);
            }
        }
        if num == 0 {
            rev = !rev;
        }
    }
    ans
}
