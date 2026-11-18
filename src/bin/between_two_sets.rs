use std::collections::HashSet;

fn main() {
    let n = 440;
    println!("{:?}", factorize_simple(n));
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
        for each_a in &a_set {
            if *each_possible_item % each_a != 0 {
                continue;
            }
        }

        for each_b in &b_set {
            if *each_b % *each_possible_item != 0 {
                continue;
            }
        }

        result.insert(*each_possible_item);
    }

    // all numbers in set a should exist in factors
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

fn factorize_simple(n: i32) -> HashSet<i32> {
    let max_factor;
    if n % 2 == 0 {
        max_factor = n / 2;
    } else {
        max_factor = n / 3;
    }
    let mut factors = HashSet::from([1]);
    for i in 2..=max_factor {
        if n % i == 0 {
            factors.insert(i);
        }
    }

    factors
}
