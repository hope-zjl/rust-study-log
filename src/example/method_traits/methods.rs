struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            laps: Vec::new(),
        }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("长度: {}, 名字: {}", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("idx: {}, {}", idx, lap);
        }
    }

    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("{total}");
    }
}

fn main() {
    let mut race = Race::new("Race");
    race.add_lap(20);
    race.add_lap(70);
    race.print_laps();
    race.finish();
}
