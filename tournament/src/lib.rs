use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;


const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";


enum Outcome { // for lack of a better word that is not Result
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {

    type Err = ();

    fn from_str(result: &str) -> Result<Self, Self::Err> {
        match result {
            "win" => Ok(Outcome::Win),
            "draw" => Ok(Outcome::Draw),
            "loss" => Ok(Outcome::Loss),
            _ => Err(()),
        }
    }

}

impl Not for Outcome {

    type Output = Self;

    fn not(self) -> Self {
        match self {
            Outcome::Win => Outcome::Loss,
            Outcome::Loss => Outcome::Win,
            Outcome::Draw => Outcome::Draw,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Team {
    name: String,
    win: u8,
    draw: u8,
    loss: u8,
}

impl Team {

    fn new(name: &str) -> Self {
        Team { name: name.to_string(), win: 0, draw: 0, loss: 0}
    }

    fn mp(&self) -> u8 {
        self.win + self.draw + self.loss
    }

    fn points(&self) -> u8 {
        self.win * 3 + self.draw
    }

    fn row(&self) -> String {
        format!("{:31}| {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name,
            self.mp(),
            self.win,
            self.draw,
            self.loss,
            self.points(),
        )
    }

    fn update(&mut self, outcome: &Outcome) {
        match outcome {
            &Outcome::Win => self.win += 1,
            &Outcome::Draw => self.draw += 1,
            &Outcome::Loss => self.loss += 1,
        }
    }

}


struct Tournament {
    tournament: HashMap<String, Team>,
}

impl Tournament {

    fn new() -> Self {
        Tournament { tournament: HashMap::new() }
    }

    fn tally(&mut self, input: &str) -> String {
        for line in input.lines() {
            let mut it = line.split(';');

            let local = it.next().unwrap();
            let visitor = it.next().unwrap();

            let result = it.next().unwrap();
            let outcome = result.parse().unwrap();

            self.update(local, &outcome);
            self.update(visitor, &!outcome);
        }

        let mut teams: Vec<&Team> = self.tournament.values().collect();

        teams.sort_by(|&local, &visitor| visitor.points().cmp(&local.points())
                .then(visitor.win.cmp(&local.win))
                .then(local.name.cmp(&visitor.name))
        );

        let mut table = Vec::new();
        table.push(HEADER.to_string());
        table.extend(teams.iter().map(|t| t.row()));
        table.join("\n")
    }

    fn update(&mut self, name: &str, outcome: &Outcome) {
        let entry = self.tournament.entry(name.to_string()).or_insert(Team::new(name));
        entry.update(outcome);
    }

}


pub fn tally(input: &str) -> String {
    Tournament::new().tally(input)
}
