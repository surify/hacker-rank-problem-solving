use std::collections::HashSet;

fn main() {
    // let n = 440;
    // println!("{:?}", factorize_simple(n));
    // let result = get_total_x(&vec![2, 6], &vec![48, 96]);
    let result = get_total_x(&vec![2, 4], &vec![16, 32, 96]);
    println!("{}", result);
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let a_set: HashSet<i32> = a.iter().copied().collect();
    let b_set: HashSet<i32> = b.iter().copied().collect();

    let a_max = get_max(a);
    let b_min = get_min(b);

    if a_max > b_min {
        return 0;
    }

    let possible_result = factorize_simple(b_min);
    let mut result: HashSet<i32> = HashSet::new();

    /*
     * each_possible_item should satisfy two conditions
     * 1. It should be a factor of all elements in set b.
     * 2. All elements in set a should be factors of it.
     */
    for each_possible_item in &possible_result {
        if !are_factors_of(*each_possible_item, &a_set) {
            continue;
        }

        if !are_multiples_of(*each_possible_item, &b_set) {
            continue;
        }

        result.insert(*each_possible_item);
    }

    result.len() as i32
}

fn get_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &each in arr {
        if each > max {
            max = each;
        }
    }

    max
}

fn get_min(arr: &[i32]) -> i32 {
    let mut min = arr[0];
    for &each in arr {
        if each < min {
            min = each;
        }
    }

    min
}

/* check if all numbers in arr are factors of n */
fn are_factors_of(n: i32, arr: &HashSet<i32>) -> bool {
    for each in arr {
        if n % *each != 0 {
            return false;
        }
    }

    true
}

/* check if all numbers in arr are multiples of n */
fn are_multiples_of(n: i32, arr: &HashSet<i32>) -> bool {
    for each in arr {
        if *each % n != 0 {
            return false;
        }
    }

    true
}

fn factorize_simple(n: i32) -> HashSet<i32> {
    let max_factor;
    if n % 2 == 0 {
        max_factor = n / 2;
    } else {
        max_factor = n / 3;
    }
    let mut factors = HashSet::from([1, n]);
    for i in 2..=max_factor {
        if n % i == 0 {
            factors.insert(i);
        }
    }

    factors
}
