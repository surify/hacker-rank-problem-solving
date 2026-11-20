// https://www.hackerrank.com/challenges/the-birthday-bar/problem

fn main() {

}

/*fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let m = m as usize;
    if s.len() < m {
        return 0;
    }

    let mut result = 0;
    for i in 0..=(s.len() - m) {
        if s[i..i + m].iter().sum::<i32>() == d {
            result += 1;
        }
    }

    result
}*/

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {

    let mut result = 0;
    for w in s.windows(m as usize) {
        if w.iter().sum::<i32>() == d {
            result += 1;
        }
    }
    result
}
