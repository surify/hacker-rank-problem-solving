// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem
fn main() {

}

// fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
//     let mut pair_count = 0;
//     for i in 0..n-1 {
//         for j in (i + 1)..n {
//             if (ar[i as usize] + ar[j as usize]) % k == 0 {
//                 pair_count += 1;
//             }
//         }
//     }
//
//     pair_count
// }

fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut pair_count = 0;
    for (i, &a) in ar[..ar.len() - 1].iter().enumerate() {
        for (j, &b) in ar[i + 1..].iter().enumerate() {
            if (a + b) % k == 0 {
                pair_count += 1;
            }
        }
    }

    pair_count
}
