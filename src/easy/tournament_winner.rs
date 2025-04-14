use std::collections::HashMap;

const HOME_TEAM_WON: i32 = 1;
const WINNING_POINTS: i32 = 3;

pub fn tournament_winner(competitions: Vec<Vec<String>>, results: Vec<i32>) -> String {
    let mut teams_points = HashMap::new();
    let mut winner = "".to_string();
    teams_points.insert(winner.clone(), 0);
    for (idx, competition) in competitions.iter().enumerate() {
        let result = results[idx];
        let home_team = &competition[0];
        let away_team = &competition[1];
        let winning_team = if result == HOME_TEAM_WON {
            home_team
        } else {
            away_team
        };
        update_points(winning_team, WINNING_POINTS,  &mut teams_points);

        if teams_points[winning_team] > teams_points[&winner] {
            winner = winning_team.clone();
        }
    }
    winner.to_string()
}

fn update_points(team: &String, points: i32, teams_points:  &mut HashMap<String, i32>) {
    *teams_points.entry(team.clone()).or_insert(0) += points;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tournament_winner() {
        let competitions = vec![
            vec!["HTML".to_string(), "C#".to_string()],
            vec!["C#".to_string(), "Python".to_string()],
            vec!["Python".to_string(), "HTML".to_string()],
        ];
        let results = vec![0, 0, 1];
        assert_eq!(tournament_winner(competitions, results), "Python");
    }
}
