fn main() {
    let ranks = climbing_leaderboard(&vec![100, 90, 90, 80], &vec![70, 80, 105]);
    // let ranks = climbing_leaderboard(&vec![100, 100, 50, 40, 40, 20, 10], &vec![5, 25, 50, 120]);
    // let ranks = climbing_leaderboard(&vec![100, 90, 90, 80, 75, 60], &vec![50, 65, 77, 90, 102]);
    println!("Ranks: {:?}", ranks);
}

fn climbing_leaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let unique_ranked = get_unique_sorted(ranked);

    let mut leaderboard_index = 0;
    let mut player_index = 0;
    let mut leaderboard_score = unique_ranked[leaderboard_index];
    let mut player_score = player[player_index];
    let mut player_ranks = Vec::new();

    loop {
        if player_score > leaderboard_score {
            leaderboard_index += 1;
            if leaderboard_index == unique_ranked.len() {
                for _ in player_index..player.len() {
                    player_ranks.push(1);
                }
                break;
            } {
                leaderboard_score = unique_ranked[leaderboard_index];
            }
        } else {
            if player_score < leaderboard_score {
                player_ranks.push((unique_ranked.len() - leaderboard_index + 1) as i32);
            } else {
                player_ranks.push((unique_ranked.len() - leaderboard_index) as i32);
            }
            player_index += 1;
            if player_index == player.len() {
                break;
            } else {
                player_score = player[player_index];
            }
            continue;
        }
    }

    player_ranks
}


fn get_unique_sorted(arr: &[i32]) -> Vec<i32> {
    let mut last_seen = arr[arr.len() - 1];
    let mut result = vec![last_seen];

    for i in (0..arr.len() - 1).rev() {
        if arr[i] != last_seen {
            last_seen = arr[i];
            result.push(arr[i]);
        }
    }

    result
}
