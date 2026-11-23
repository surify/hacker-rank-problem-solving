use std::collections::HashMap;
fn main() {
    // let result = picking_numbers(&vec![4, 6, 5, 3, 3, 1]);
    let result = picking_numbers(&vec![1, 2, 2, 3, 1, 2]);
    println!("Result: {result}");
}

fn picking_numbers(a: &[i32]) -> i32 {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for each in a {
        let entry = frequency_map.entry(*each).or_insert(0);
        *entry += 1;
    }

    let mut numbers: Vec<i32> = frequency_map.keys().copied().collect();
    numbers.sort();

    let mut previous_element = numbers[0];
    let mut previous_frequency = frequency_map.get(&numbers[0]).copied().unwrap_or(0);
    let mut max_length = previous_frequency;

    if numbers.len() == 1 {
        return max_length;
    }

    for elem in &numbers[1..] {
        let frequency = frequency_map.get(elem).copied().unwrap_or(0);
        if frequency > max_length {
            max_length = frequency;
        }

        if *elem - previous_element == 1 && frequency + previous_frequency > max_length {
            max_length = frequency + previous_frequency;
        }

        previous_element = *elem;
        previous_frequency = frequency;
    }

    max_length
}
