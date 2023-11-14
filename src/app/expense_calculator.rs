use std::error::Error;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Contribution {
    name: String,
    contribution: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExpenseCalcApp {
    pub input: String,
    pub contributions: Vec<Contribution>
}

impl Default for ExpenseCalcApp {
    fn default() -> Self {
        ExpenseCalcApp { input: String::new(), contributions: vec![] }
    }
}

impl ExpenseCalcApp {
    pub fn parse_data_from_input(&mut self) -> Result<(), Box<dyn Error>> {
        self.contributions = vec![];
        for line in self.input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err("Incomplete data".into());
            }

            self.contributions.push(Contribution { name: parts[0].to_string(), contribution: parts[1].parse::<u32>()? })
        }

        Ok(())
    }
}