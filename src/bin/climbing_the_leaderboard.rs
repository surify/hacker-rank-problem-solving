use std::collections::HashSet;

fn main() {
    // let ranks = climbing_leaderboard(&vec![100, 90, 90, 80], &vec![70, 80, 105]);
    // let ranks = climbing_leaderboard(&vec![100, 100, 50, 40, 40, 20, 10], &vec![5, 25, 50, 120]);
    let ranks = climbing_leaderboard(&vec![100, 90, 90, 80, 75, 60], &vec![50, 65, 77, 90, 102]);
    println!("Ranks: {:?}", ranks);
}

fn climbing_leaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut unique_ranked: Vec<i32> = Vec::new();
    let mut seen_scores: HashSet<i32> = HashSet::new();

    for each in ranked {
        if !seen_scores.contains(each) {
            seen_scores.insert(*each);
            unique_ranked.push(*each);
        }
    }

    let mut final_ranks = Vec::new();
    for each_score in player {
        let insert_index = insert_in_order(&mut unique_ranked, *each_score);
        final_ranks.push((insert_index + 1) as i32);
    }

    final_ranks
}

/* inserts elem in arr(already sorted in descending order) if it doesn't already exist.
 * returns the inserted index
 */
fn insert_in_order(arr: &mut Vec<i32>, elem: i32) -> usize {
    let mut insert_index: Option<usize> = None;
    let mut should_insert = false;
    for (index, each) in arr.iter().enumerate() {
        if *each <= elem {
            insert_index = Some(index);
            if *each < elem {
                should_insert = true;
            }
            break;
        }

        if *each > elem {
            if index < arr.len() - 1 {
                continue;
            }
            insert_index = Some(index + 1);
            should_insert = true;
            break;
        }
    }

    let insert_index = insert_index.expect("Edge case");
    if should_insert {
        arr.insert(insert_index, elem);
    }
    insert_index
}
