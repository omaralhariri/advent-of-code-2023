use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Cannot read file");
    let lines: Vec<&str> = file.lines().collect();
    let mut failed_games = vec![];
    let mut all_sum: usize = 0;
    let mut sum_power = 0;

    for line in lines.iter() {
        let colon = line.find(":").unwrap();
        let game_no: usize = line.get(5..colon).unwrap().parse::<usize>().unwrap();
        all_sum += game_no;

        let game = line.get(colon + 2..).unwrap();
        let game_rounds: Vec<&str> = game.split(";").collect();
        let trimmed_game_rounds: Vec<&str> = game_rounds.iter().map(|round| round.trim()).collect();
        let rounds: Vec<_> = trimmed_game_rounds
            .iter()
            .map(|round| round.split(",").collect::<Vec<_>>())
            .collect();

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in rounds.iter() {
            let trimmed: Vec<&str> = round.iter().map(|r| r.trim()).collect();
            for num_color in trimmed.iter() {
                let space = num_color.find(" ").unwrap();
                let color = num_color.get(space + 1..).unwrap();
                let num = num_color.get(..space).unwrap().parse::<usize>().unwrap();

                match color {
                    "red" => {
                        if num > 12 {
                            if !failed_games.contains(&game_no) {
                                failed_games.push(game_no);
                            }
                        }
                        if num > min_red {
                            min_red = num;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            if !failed_games.contains(&game_no) {
                                failed_games.push(game_no);
                            }
                        }
                        if num > min_green {
                            min_green = num;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            if !failed_games.contains(&game_no) {
                                failed_games.push(game_no);
                            }
                        }
                        if num > min_blue {
                            min_blue = num;
                        }
                    }
                    _ => {}
                };
            }
        }

        sum_power += min_red * min_green * min_blue;
    }

    let sum: usize = failed_games.iter().sum();
    println!(
        "Sum Failed: {:?}, Sum Success: {:?}, Sum Power: {:?}",
        sum,
        all_sum - sum,
        sum_power
    );
}
