fn main() {
    println!("Valley count: {}", counting_valleys(14, "DDUDUUDDUUUDUUDD"));
}

fn counting_valleys(steps: i32, path: &str) -> i32 {
    let mut valleys = 0;
    let mut level = 0;
    for step in path.chars() {
        if step == 'D' {
            if level == 0 { // beginning of a valley
                valleys += 1;
            }
            level -= 1;
        } else {
            level += 1;
        }
    }
    valleys
}
