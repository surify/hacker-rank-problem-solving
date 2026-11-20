/* https://www.hackerrank.com/challenges/sock-merchant/problem?isFullScreen=true */
use std::collections::HashSet;

fn main() {

}
fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    let mut pair_count = 0;
    for item in ar {
        if set.contains(item) {
            pair_count += 1;
            set.remove(item);
        } else {
            set.insert(*item);
        }
    }

    pair_count
}
