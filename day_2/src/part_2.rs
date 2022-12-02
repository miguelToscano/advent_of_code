fn get_opponents_play(encoded_play: &str) -> Play {
    match encoded_play {
        "B" => Play::Paper,
        "A" => Play::Rock,
        "C" => Play::Scissors,
        _ => panic!("Invalid input"),
    }
}

fn get_my_play(encoded_play: &str) -> Play {
    match encoded_play {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("Invalid input"),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Play {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
pub enum ExpectedResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
pub struct MatchResult {
    pub opponents_play: Play,
    pub my_play: Play,
}

impl MatchResult {
    fn get_my_play_score(&self) -> u64 {
        match self.my_play {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn get_play_result_score(&self) -> u64 {
        // DRAW
        if self.my_play == self.opponents_play {
            return 3;
        }

        // MY PLAY WINS
        if self.my_play == Play::Rock && self.opponents_play == Play::Scissors || self.my_play == Play::Scissors && self.opponents_play == Play::Paper || self.my_play == Play::Paper && self.opponents_play == Play::Rock {
            return 6;
        }

        // MY PLAY LOSES
        return 0;
    }

    pub fn get_total_score(&self) -> u64 {
        self.get_my_play_score() + self.get_play_result_score()
    }
}

pub fn get_expected_result(expected_result: &str) -> ExpectedResult {
    match expected_result {
        "X" => ExpectedResult::Lose,
        "Y" => ExpectedResult::Draw,
        "Z" => ExpectedResult::Win,
        _ => panic!("Invalid input"),
    }
}

pub fn get_my_play_for_expected_result(opponents_play: Play, expected_result: ExpectedResult) -> Play {
    match expected_result {
        ExpectedResult::Draw => opponents_play,
        ExpectedResult::Win => match opponents_play {
            Play::Rock => Play::Paper,
            Play::Scissors => Play::Rock,
            Play::Paper => Play::Scissors
        },
        ExpectedResult::Lose => match opponents_play {
            Play::Rock => Play::Scissors,
            Play::Scissors => Play::Paper,
            Play::Paper => Play::Rock
        },
    }
}

pub fn get_input() -> Vec<MatchResult> {
    let input = vec!["C Z","C Z","A X","A X","B Z","B Z","B Z","A Z","B X","A X","A X","A X","C Z","C Z","C X","A X","A X","A X","C Z","B Z","C Z","A Y","B Z","A X","C Y","A X","A X","C Y","C Z","A Y","B Z","A X","C Y","B Z","B Z","B Z","A X","C X","C X","B Z","A X","C Z","A X","B Z","A Y","C X","A X","C Z","C Z","B Z","C Y","C X","C X","C X","C X","A Y","C Y","C Z","C Y","A X","C Y","A X","B Z","A Y","C X","A X","B Y","A X","C X","C X","C Z","A X","C X","A X","C X","B Z","A Z","B Y","B Z","B Z","A X","C Y","B X","A X","A X","B Z","A X","C X","C Z","C Z","A X","C Z","C Z","C Z","C X","C Z","A X","B Z","B Y","C Z","B Z","B Z","A X","B Z","C Y","C Z","A X","A Y","C Z","A X","B Y","C Y","C Z","A X","C Y","C X","A X","C X","A X","C Z","A X","C Y","A Y","C Z","C X","C Y","C Y","A Y","A Y","C Y","C Y","A Y","A X","C Z","C Z","C X","C X","C Z","A Y","C Z","C Z","A Y","A Y","B Z","A X","B Z","C Y","B Z","C Z","C Y","C Y","B Y","C X","A X","A X","A X","A Y","A X","C Z","C Z","C Z","A X","B Z","A X","B Z","C Y","C Z","A Y","C Y","A X","A X","C Z","B X","C Z","C Z","B Z","B Z","B Z","C Z","C X","C Z","A X","B Z","C Z","C X","A X","A Y","B Y","A X","B Z","A X","C Z","C Z","C X","B Z","C Z","C Z","C X","A X","A X","B Y","C Y","C Z","B Y","A X","B Z","A X","B X","B Z","C X","C X","C Y","C X","B Y","B Z","C X","A Y","C X","C Y","A X","C Z","C Z","C X","B Z","A Y","B Z","A Y","B X","A X","C Z","B Z","C X","C X","C X","A X","C X","B Z","C Z","B Y","B Z","A X","A X","C Z","B Z","A X","A Z","C Y","B Z","C X","C Z","A Y","C X","A X","A X","A Y","B Z","A X","B Z","A Y","B X","C X","C Y","A X","C Z","B X","C Z","C X","C X","C Z","A X","B Z","C Z","C X","C Z","B Y","A X","C X","C Y","A X","C Z","A X","B Z","C X","C X","B Z","C Z","C Z","C Y","A X","C Z","C X","A X","C Y","C Z","C Z","C Y","B Y","B Z","A X","B Y","C X","A X","A X","C Y","C Z","B Z","C X","C X","B Z","C Z","A X","A Y","A X","C Y","A Y","B Y","A X","A X","C X","C X","B Z","A Y","B Z","C Z","A X","A Y","A X","C X","A X","C Z","B Y","C Z","A X","B Z","C Z","C Z","A X","A X","B Z","B Z","B Z","A X","C X","A X","A X","C Y","C Y","C X","C X","A X","A Y","B Y","C Z","C Z","B Y","C X","C Z","A Y","C X","C Z","C X","B Z","C Y","C Z","C X","C Y","C Z","B Z","C Z","B Z","A X","B Y","B Y","C Z","B Z","C X","C Z","C Z","C Z","B Y","B Z","C Z","B Y","A X","A X","C X","A X","C X","A X","B Z","A X","A X","A X","C Z","A Y","A X","C Z","C X","A X","C X","A X","B Z","C Z","C Z","C Z","A X","A X","C Z","A X","B Z","C X","C Z","C Y","B Z","A X","B Z","A X","A Z","B Y","B Z","B Z","B Z","A Y","C Z","A X","B Y","C X","B X","B Z","C X","B Y","C Z","A X","C X","B Z","A X","B Z","A X","A X","B Y","C Y","C X","C X","C Z","A X","C Z","A X","B Y","C X","C Y","B Y","B Y","A Y","C X","A Y","C Z","C Y","A X","C Z","C Z","B Z","A X","A X","A X","A X","B Z","C Z","C X","C X","C Z","A X","B Z","B Y","C Y","A Y","C X","B Z","A Y","B Z","C Z","B Z","C X","A X","B Z","A X","A Y","C Y","B Z","B Z","C X","C Z","C Z","C Z","A X","B Z","A Y","A X","C Y","A Y","B Z","A Y","C Z","A Y","A Y","C X","A X","B Z","A Y","B Z","C X","A X","B Y","B Z","C Z","C Z","B Z","A X","A X","B Y","C Z","C Y","B Z","C Z","A Y","A X","C X","C Z","A X","A X","A Z","C X","C X","B X","B Z","A Y","C X","A Y","C X","C Z","A Y","B Z","A X","B X","A Y","C X","C Y","A Y","C X","B Y","C Y","A X","B Z","A X","C Z","A X","A X","C Z","C Z","B X","C X","A Y","B Z","C Z","A X","A X","C Z","C Y","A X","C X","C Z","C Y","C X","C Y","C X","C Z","C Z","B Z","A X","A Y","B Z","A X","B Z","C X","B Y","A X","A X","A X","A X","B Z","C Z","B Z","B Y","C Z","C X","C Z","C X","B Z","C Z","A X","C Z","C Z","A Y","C X","A X","B Z","A X","B Z","B Y","A X","A X","A X","A Y","C Z","A X","B Y","A Y","B Y","C Y","A X","A X","C Y","A X","C Z","B Z","C X","C Z","A X","B Z","C Z","B Z","B Z","B Z","C X","C Z","A X","A Y","C X","C Z","B Z","B Y","C Z","B Y","B Z","C Z","A Y","B Z","C X","C Y","A X","B Y","A X","C Z","C X","C Z","A X","C X","B Z","B X","C Z","B Z","A Y","A X","A Y","C Z","C Y","A X","B X","A Z","C Y","C X","C Y","C X","C Y","B Z","C X","B Y","C Z","C Y","B Z","A Y","B Y","C Z","C X","B Z","A Y","C Z","C Z","C Z","C Y","C X","C X","B Y","C Z","A Y","C Z","C X","B Y","C X","B Z","B Z","B Z","B Z","C Y","C Z","C X","B X","C Z","C Z","C Z","C X","B Y","B Z","C X","A X","C Y","B X","C X","C X","C Z","A X","A X","B Z","B Z","A X","C Z","A X","A Y","C X","B X","C Y","C Y","A X","A X","C Z","C Z","B Y","B Y","B Z","C X","A X","A Z","C Z","B Y","C Y","C X","C Y","B Z","C X","A X","A X","C Z","C Y","B X","C Z","B Z","A X","C X","A Y","C Y","C Z","B Z","C Z","A X","C Y","B Z","C Y","B X","C Z","C Z","A X","A X","C X","C Z","C X","C Y","C Z","A Y","C X","C Z","B Y","B Y","C Z","A X","C Y","A X","C X","C X","B Y","C Z","C Z","A X","A Y","C Z","B Z","C Y","A X","A X","C Z","C Z","A X","C Z","C Z","B Z","B Z","A X","A Y","A X","C Z","C X","A X","B X","C Z","C Z","B Y","A X","B Z","C X","C Z","C Z","C Z","C X","A Y","A X","A Y","A X","A Y","C Z","C Z","B Z","B Y","A X","B Z","C Z","C X","C Y","C Z","A X","C Y","B Y","B Z","C Z","B Y","C X","C Z","B Y","B Z","B Y","C X","C Z","C X","C Z","B Y","B Z","C X","A X","A Y","B Z","B Y","A X","A X","C Z","A X","A X","C X","A X","C Z","C X","A X","B X","A Y","C X","A X","C Y","A X","C Z","A X","A X","C Z","A X","A X","C Z","A X","B Y","B Z","A X","C X","A X","C X","B Z","C Y","A Y","C Z","B Z","A X","C X","A X","B Y","C X","B Y","B Z","A X","B Z","C X","B Z","A X","A X","C X","A X","C X","A Y","C Z","A X","B Y","B Z","C Z","C Z","A X","C Y","B Z","C X","B Z","B Z","C Z","A X","C X","C X","A Y","A X","C Z","B Z","A X","B X","A X","B Z","B Z","C Z","B Y","C Z","A X","C Z","C Z","C Z","C X","A X","A X","C Y","B Y","C Z","B Y","A X","B Z","A Y","C Y","B Y","C Z","C X","C X","A X","B Y","A X","A X","C Z","C X","C Y","A X","A X","A X","C X","B Z","B Y","A X","B Y","C X","C Z","B Z","A X","C X","C Z","B Y","A X","C Z","B Z","B Z","A X","B Y","A X","B Y","C Z","C Z","B Z","C Z","C Y","A X","B Z","C Z","A Y","C Z","B Z","B Z","C Z","B Y","C Z","C Z","B Z","A Y","C X","C Y","B Z","C X","C Z","A X","A X","C Y","A Z","C Z","C X","B Z","B Z","C Y","A X","A Y","C Z","A X","A X","B Z","A X","C X","C X","B X","B X","A Z","B Y","A X","A Y","C Z","A X","A X","B Z","B Z","C Y","A X","C Y","C Z","A Y","C Y","B X","C X","A X","B Z","A X","A X","A X","A X","A X","B X","A X","C Z","B Z","B Z","B X","A X","C Z","B Z","C Z","C Z","C Y","A X","C Y","C Y","C Z","A Y","C Z","A X","C X","C Z","A X","A Y","C Z","C X","A X","C X","B Y","C Z","B Z","A X","C Y","B Z","B Y","A X","C Z","A X","B Z","C Z","C Y","C Z","A X","A X","B Z","B Y","A X","C X","C Y","B X","C Z","A Y","C X","B Y","B Z","A Y","C Y","C Z","C Z","C X","A X","A X","C X","C X","C Z","C X","C X","B X","B Z","B Z","C X","C Z","C X","A X","A X","A X","C Z","C Y","B Z","C X","C X","A X","C X","C Y","A X","B Y","A X","C Z","C Z","B Z","A X","A X","A X","B Z","C X","A X","B Y","B Z","C X","B Y","C X","B Y","C Z","B X","A X","A X","B Z","A X","A X","A X","B Z","A X","C Z","C X","C Z","A X","C Y","C Y","C Z","C Z","C Z","A X","C Z","C X","C Z","C Z","A X","B Z","C X","C Y","B X","B Z","B Z","A X","C Y","B Y","C Z","B Z","C Z","C X","A Z","C Z","B X","C X","C Z","C Z","C X","C Z","C Z","A X","B Z","C Z","C X","A X","B Z","C Y","C Y","B Y","C X","A X","A X","C Z","B Z","C X","C X","C X","B Y","C Z","B X","C X","C Z","A X","A X","C Z","C Z","C X","C X","C Z","A X","C Z","C X","C Z","C Z","A X","C Z","A X","A X","A X","B Z","A X","B Z","C Z","C Z","A X","C Z","C Z","B Z","C Z","A Y","C X","A X","A X","C X","A X","C X","C X","A Y","B Y","C Z","B Z","C X","B Y","C Y","C X","C Z","A X","C Y","B Z","B Y","A X","B Y","B Y","A X","C X","C X","A X","A Y","C Y","B Z","A X","B Z","A Y","B Y","C Z","C Z","C Z","A Y","B Y","B Y","C Z","B Z","C Z","A X","C Y","A X","B Y","B Z","C X","C Y","A X","B Z","B Z","A X","B Z","C Y","A X","C X","C Z","C Z","A X","C Y","A X","C X","C X","B Z","B Y","A X","C Y","B Z","B Y","A X","C Z","A X","B Y","A X","A X","A X","C X","A X","C Z","B Z","C Z","A X","A X","B Y","C X","C Y","C Z","A X","C Z","B Z","A X","B Z","A X","B Y","B Z","C Z","C Z","C Y","C X","A X","C Z","A Y","A X","B Z","B Y","A X","C X","C Z","B Z","A X","C Z","C X","C Y","A X","A X","C Z","A X","A X","C X","C X","A X","C X","C Z","A X","A X","C Y","A Y","B Z","C X","C X","B Y","B Z","A X","B Z","C Z","C Z","A Y","C Y","C Z","A X","C Z","A X","C Y","C X","B X","B Z","A X","A X","B Z","A X","C X","C X","A Y","A X","A X","C Y","B Z","A X","B Y","C X","B Z","C Y","B Y","A X","C Z","B Y","B Z","A X","B Y","C X","C Z","A X","B Z","A X","C Y","C Z","A X","A X","C Y","C Y","C X","A Y","C Z","B Z","A X","A Y","C Z","C Z","C Z","C Z","B Y","A Y","A X","A X","C X","A X","B Y","C Z","B Z","A Y","C X","C X","A Y","A X","A X","C X","C Z","C X","A X","A Y","C X","C Z","C X","A X","A X","C Z","C X","A X","C X","C Z","A X","A X","C Z","B Y","A X","C Z","C Y","C Z","C Z","B X","B Z","B X","A X","C Z","B X","C Z","A X","C Z","A Y","A Z","B Z","C X","A X","A X","A X","A X","A X","C Z","A X","B X","A Y","A X","C X","A X","A Y","A Y","B Y","C X","A Y","A X","C Z","C Z","C Y","B Z","C X","C Z","A Y","C X","C Y","C X","A X","B Y","C Z","C Z","A X","C Z","B Z","B Z","C X","C X","A X","B Y","C X","C X","A X","C Z","B Y","C Y","C Z","A X","B Z","C Z","B Y","C X","A X","B Y","B Z","A X","C Y","B Z","C Z","C X","A X","C X","A X","B Y","C X","A X","C Y","B Z","B Z","B Z","C Y","A X","A X","A Y","C Z","B X","B Z","A X","A X","C X","A X","C X","A X","C X","A X","A X","A X","A Y","A Y","B Y","A X","B Y","A X","B Y","B Z","B Y","A Y","C Z","C Z","C Z","C Z","A X","C Z","C X","C Y","C Z","B Z","B Z","B Z","C Y","C Z","B Z","C X","A Y","A X","C X","C Y","A Y","B Y","C X","C X","B Y","C Z","A X","C X","C X","C X","C X","C X","C X","B Z","C Y","A X","A X","B Y","B Z","A X","C X","C Z","A X","B X","B Z","C Y","B Z","C Z","C Y","A X","C Z","B Y","C X","A X","A Y","A X","C X","A Y","A Y","C Z","B Z","C Z","C X","C Y","B Y","A X","C X","C Z","B Y","A X","B X","B Z","C Z","B Z","B Z","A X","A Y","A X","A X","B Z","C Z","A X","A Y","C Z","C Z","C Y","A X","B Z","C X","C X","A Y","A X","C X","C Y","C Z","C Y","C X","A X","B Z","C Y","B Z","C X","A X","A X","C X","C Z","C Y","C Y","A X","A Y","C Z","A Y","B Y","C Y","A Y","C Z","B Y","C X","A X","B Z","C Z","A X","A X","B Z","C X","B Z","C Z","C Z","A Y","A X","C X","A X","B Z","C X","A X","A X","B Y","A Z","B Z","C X","C Z","C X","C Z","C X","A X","C Z","B Y","C Y","C Z","A X","C Z","A X","A Z","A X","A X","C Z","A X","B Z","C X","B Z","A X","C X","A X","C X","C Z","B Z","A Y","C Y","C Z","C Z","C X","C Z","A X","A X","B Y","B Z","C Y","B Z","A Y","A X","A X","B Z","B X","A X","B Y","B Z","C X","A X","A X","C Z","B Y","C X","B Y","A Y","A Z","A X","B Y","C X","C Z","C Z","C Y","C Z","A X","B X","C X","A Y","A X","C Z","C Y","C X","B Z","A X","C X","C Z","A X","C Z","A X","C X","C X","A Y","C Z","A X","C X","C Z","C Y","A X","A Y","A Y","C Z","C X","A X","C Z","A Y","A Y","A X","B Z","C Z","B Y","C Z","B Y","C X","C Z","B Y","A X","A X","B Z","A X","C Z","A X","A X","B Y","A X","B Z","A Y","A X","A Y","C Z","B Z","A X","B X","B Y","B Z","A X","A Z","B Y","B Z","C X","B Z","A X","B Z","C Z","B Y","C X","A X","A Y","B Y","A Y","C X","C X","B Y","A Y","B Z","C Z","B Z","A Y","A X","B Z","B Z","A Y","C Z","C Z","B Z","C X","B Z","B Y","B Z","B Z","A X","A X","C X","C Z","B Z","A X","B Z","C Y","C Z","A Z","A X","A X","C Z","A X","C Z","C Z","C Z","B Z","A X","A Y","B X","A X","A X","A X","C Z","B X","B X","A X","C Z","A X","B X","A X","A X","B Z","A X","C Z","C Y","A X","B Z","A X","C Z","B Z","A X","A X","B Y","B Y","C Y","A X","B Z","C Y","A X","C Z","B Z","A X","C Z","B Z","C X","B X","B Z","C Z","A X","B Z","A X","A X","A X","A X","C Z","B Z","B Y","C Z","A X","C Z","A X","C X","C X","A X","B Y","C X","B Z","A Y","C Y","B Z","C Y","C X","A X","B Z","C X","B Y","A X","A Y","B Y","B Y","A X","C X","C Z","B Z","A Y","A X","B Y","C X","B Z","B X","C Y","B X","C X","B Y","C X","A X","C Z","B Z","B Y","B Y","A X","A X","A Z","B Z","B Z","C X","C Y","A Y","C Z","C X","A X","C Z","C X","B Z","A Y","B Z","C Z","B Z","B Z","C X","B Z","C X","C X","B Z","A X","B Y","B Z","A X","C X","A X","C Y","A X","C X","B X","C Z","C Y","C Z","B Z","C X","A X","A X","A Y","C Z","C Y","A Y","C Z","B Z","C Z","C Y","A X","A X","C X","C Z","C Z","A Z","C Z","A X","B Z","C X","C Z","A X","B Z","C X","A X","A Y","B Y","C X","A X","A X","C X","C Y","B Z","C X","A X","C X","B Z","B Y","A Y","C Z","A X","C Z","C Z","A X","C Y","B Z","A Y","B Y","B Y","A X","A X","C X","C X","B Y","C Z","C Z","C Z","C X","C Y","C Z","B Z","C Z","A Y","C Z","A X","B Z","B Y","B Z","A X","A X","A X","A X","C X","C Z","B Z","C X","B Z","C Z","A X","C X","C Z","B Y","A X","C X","A X","B Z","B Z","A X","A X","B Y","C Y","C Y","C X","A X","B Z","C Y","B Z","A Y","B Y","B Y","A X","B Z","A X","C X","C Z","A X","B Z","A Y","C Y","B Z","C Z","C Y","A X","C Y","A X","C X","B Z","C X","A X","C Z","A X","B Y","B Z","C X","C Y","B X","A X","B Z","A X","B Y","C X","C X","C Y","C Y","C X","B Y","C Z","C X","B Z","B Y","C X","A Y","C Z","C Z","C X","B X","B Y","A X","A X","C Z","C X","C Y","A X","C Y","A Y","C Z","C X","A X","C Z","C X","A X","B X","C X","C X","C X","B Y","B Z","C X","C Z","A Y","B Y","C X","C X","A X","B Z","C Z","A X","C Y","C Z","A Y","B Z","C X","C Z","A X","C Z","B Z","B Y","B Z","A X","C X","A Y","C X","C Z","B Y","C Y","C Z","C Z","A X","A X","A X","C Z","B Y","C Z","A X","A X","B Z","B Y","B Z","C Z","B Z","A Y","C Y","C Z","B Y","A X","A X","C Z","B Y","C Z","A X","C X","B Y","A Y","B Z","A X","A X","A X","B Y","A X","C Z","B Y","C Z","B Y","C Z","C X","C Y","C X","A X","A X","A X","C Z","C X","C X","B Y","A Y","B Z","B X","C Z","B Y","A X","C Y","B Z","C X","A X","A X","A X","A X","C X","B Y","A X","A X","B Y","A X","B Z","C Z","A X","A X","A X","A X","B Z","A X","C Z","C X","C Y","B Z","C Z","A X","C Y","C Z","A X","A X","C X","B Z","C X","B Z","C Z","A X","A X","A Y","B Y","C Z","B Y","A X","C X","C Z","C Z","C X","A X","A X","C Z","A X","A Y","A X","A X","C Z","C X","C X","C Z","C X","A X","C Z","C Z","B Z","A Y","A Y","B Y","A X","A X","C X","C X","A X","A X","C X","C X","B Y","C Z","A X","B Y","A X","A X","A X","C X","C Y","B Z","B Y","B Z","C Z","C Z","C X","C Z","A X","B Z","A Y","C Z","B Z","A X","C Y","B Z","C Z","C Y","A X","B Y","C Z","A X","A X","A X","C Z","C Z","C X","A Y","C X","B Z","B Y","A X","C X","C X","A Y","A Y","A X","B X","B Z","B Y","B Y","A X","C Y","A X","C Z","C X","C X","C Z","B Z","B Z","C Y","C X","B Z","C Z","A Y","C Y","A X","B Z","A X","C Y","B Z","B Y","C Z","A X","A X","C Z","B Z","B Z","C Z","C Y","C Z","C Z","C X","A Y","A Y","B Z","C Z","B Y","C X","C X","A Z","C Z","A X","A X","C X","A Y","C X","A X","A X","A X","C Y","A X","B Y","A X","B Y","A X","A X","A Y","C X","C Z","A X","C X","B Y","B Z","B Z","A Y","C Z","C X","C X","B Z","B Z","C X","B Y","A X","B Y","A X","A X","C Y","B X","C X","A Z","A Z","A X","C Z","C Y","C Z","C Y"];
    let mut results = vec![];

    for item in input {
        let mut parsed_item = item.split(' ');
        let opponents_play = get_opponents_play(parsed_item.next().unwrap());
        let expected_result = get_expected_result(parsed_item.next().unwrap());
        results.push(MatchResult { opponents_play: opponents_play.clone(), my_play: get_my_play_for_expected_result(opponents_play, expected_result) })
    }
    return results;
}

pub fn get_answer() {
    let results = get_input();

    println!("{:?}", results);

    let mut total_score: u64= 0;

    for result in results {
        total_score = total_score + result.get_total_score();
    }

    println!("Total score is: {:?}", total_score);
}