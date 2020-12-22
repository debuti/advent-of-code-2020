use std::collections::{HashSet, VecDeque};

const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<VecDeque<u32>> = data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .enumerate()
                .filter(|&(i, x)| i > 0 && x.len() > 0)
                .map(|(_, x)| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    step1(data.clone());
    step2(data.clone());
}

fn step1(mut data: Vec<VecDeque<u32>>) {
    let mut round = 1;
    loop {
        debugln!("-- Round {} --", round);
        let mut flip: Vec<u32> = Vec::new();
        let mut id = 0;
        for player in &mut data {
            id += 1;
            debugln!("Player {}'s deck: {:?}", id, player);
            flip.push(player.pop_front().unwrap());
        }
        let winner = flip
            .iter()
            .position(|x| x == flip.iter().max().unwrap())
            .unwrap();
        debugln!("Player {} wins the round!\n", winner + 1);
        flip.sort_by(|a, b| b.cmp(a));
        for card in flip {
            data[winner].push_back(card);
        }
        // Check if somebody won
        if data.iter().filter(|x| x.len() > 0).count() == 1 {
            break;
        }
        round += 1;
    }
    debugln!("== Post-game results ==");
    for (idx, player) in data.iter().enumerate() {
        debugln!("Player {}'s deck: {:?}", idx + 1, player);
    }
    println!(
        "Step 1 answer: {}\n",
        data.iter()
            .filter(|x| x.len() > 0)
            .next()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * (1 + i as u32))
            .sum::<u32>()
    );
}

fn step2(mut data: Vec<VecDeque<u32>>) {
    fn game(data: &mut Vec<VecDeque<u32>>, depth: &mut u32) -> usize {
        debugln!("=== Game {} ===\n", depth);
        let mut history: HashSet<Vec<VecDeque<u32>>> = HashSet::new();
        let mut round = 1;
        let gameid = depth.clone();
        loop {
            debugln!("-- Round {} (Game {}) --", round, gameid);

            // History rule
            {
                if history.contains(data) {
                    debugln!("Player 1 won this round by the history rule");
                    break;
                }
                // Insert the current configuration into history
                history.insert(data.clone());
            }

            let mut flip: Vec<u32> = Vec::new();
            // Draw some cards
            {
                let mut id = 0;
                for player in &mut *data {
                    id += 1;
                    debugln!("Player {}'s deck: {:?}", id, player);
                    flip.push(player.pop_front().unwrap());
                }
            }

            let mut flag = true;
            // Recursive rule
            {
                for (i, f) in flip.iter().enumerate() {
                    if (data[i].len() as u32) < *f {
                        flag = false;
                        break;
                    }
                }
            }

            // Retrieve the winner
            let winner = if flag {
                debugln!("Playing a sub-game to determine the winner...\n");
                *depth += 1;
                let winner = game(
                    &mut data
                        .clone()
                        .iter()
                        .enumerate()
                        .map(|(i, x)| {
                            x.iter()
                                .enumerate()
                                .filter(|&(j, _)| (j as u32) < flip[i])
                                .map(|(_, &c)| c)
                                .collect()
                        })
                        .collect(),
                    depth,
                );
                debugln!("...anyway, back to game {}.\n", gameid);
                winner
            } else {
                flip.iter()
                    .position(|x| x == flip.iter().max().unwrap())
                    .unwrap()
            };
            debugln!(
                "Player {} wins the round {} of game {}!\n",
                winner + 1,
                round,
                gameid
            );

            // Pay the winner (Sadly with this modification this game can only be played by two people)
            data[winner].push_back(flip.remove(winner));
            data[winner].push_back(flip.pop().unwrap());

            // Check if somebody won
            if data.iter().filter(|x| x.len() > 0).count() == 1 {
                break;
            }

            round += 1;
        }

        let winner = data
            .iter()
            .enumerate()
            .filter(|(_, x)| x.len() > 0)
            .map(|(i, _)| i)
            .next()
            .unwrap();
        debugln!("The winner of game {} is player {}!\n", gameid, winner + 1);
        winner
    }

    let mut depth = 1;
    let winner = game(&mut data, &mut depth);
    debugln!("== Post-game results ==");
    for (idx, player) in data.iter().enumerate() {
        debugln!("Player {}'s deck: {:?}", idx + 1, player);
    }
    println!(
        "Step 2 answer: {}",
        data[winner]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x * (1 + i as u32))
            .sum::<u32>()
    );
}
