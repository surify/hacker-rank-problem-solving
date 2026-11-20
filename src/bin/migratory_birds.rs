// https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true
use std::collections::HashMap;
fn main() {

}

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut bird_sighting_map: HashMap<i32, u32> = HashMap::new();
    for each in arr {
        let entry = bird_sighting_map.entry(*each).or_insert(0);
        *entry += 1;
    }

    let mut max_count = 0;
    let mut max_bird_id = arr[0];

    for (key, value) in bird_sighting_map.iter() {
        if *value > max_count {
            max_count = *value;
            max_bird_id = *key;
        } else if *value == max_count && *key < max_bird_id {
            max_bird_id = *key;
        }
    }

    max_bird_id
}
