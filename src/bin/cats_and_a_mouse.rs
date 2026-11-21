/* https://www.hackerrank.com/challenges/cats-and-a-mouse/problem?isFullScreen=true */
fn main() {
    println!("{}", cats_and_a_mouse(1, 3, 2));
}

fn cats_and_a_mouse(a: i32, b: i32, m: i32) -> String {
    let cat_a_distance = (m - a).abs();
    let cat_b_distance = (m - b).abs();

    let result: &str;
    if cat_a_distance < cat_b_distance {
        result = "Cat A";
    } else if cat_b_distance < cat_a_distance {
        result = "Cat B";
    } else {
        result = "Mouse C";
    }

    String::from(result)
}
