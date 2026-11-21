fn main() {
    // let result = electronics_shop(&vec![3, 1], &vec![5, 2, 8], 10);
    let result = electronics_shop(&vec![4], &vec![5], 5);
    println!("Highest possible purchase in budget: {result}");
}

fn electronics_shop(keyboards: &[u32], drives: &[u32], budget: u32) -> i32 {
    let keyboards = sorted_desc(keyboards);
    let drives = sorted_desc(drives);

    let mut current_max = u32::MIN;
    for each_keyboard in &keyboards {
        for each_drive in &drives {
            let bill_amount = *each_keyboard + *each_drive;
            if bill_amount == budget {
                return budget as i32;
            }
            if bill_amount < budget {
                if bill_amount > current_max {
                    current_max = bill_amount;
                }
                break;
            }
        }
    }

    if current_max == u32::MIN {
        -1
    } else {
        current_max as i32
    }
}

fn sorted_desc(arr: &[u32]) -> Vec<u32> {
    let mut arr = arr.to_vec();
    arr.sort_unstable_by(|a,b| b.cmp(a));

    arr
}
