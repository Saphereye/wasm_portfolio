use std::error::Error;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Contribution {
    name: String,
    contribution: f32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExpenseCalcApp {
    pub input: String,
    pub contributions: Vec<Contribution>,
    pub output: String,
}

impl Default for ExpenseCalcApp {
    fn default() -> Self {
        ExpenseCalcApp {
            input: String::new(),
            contributions: vec![],
            output: String::new(),
        }
    }
}

impl ExpenseCalcApp {
    pub fn find_contributions(&mut self) {
        self.output = String::new();

        self.contributions = vec![];
        for line in self.input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                eprintln!("Incomplete Data");
                return;
            }

            self.contributions.push(Contribution {
                name: parts[0].to_string(),
                contribution: match parts[1].parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Not a number");
                        return;
                    }
                },
            });
        }

        self.output += format!("{:?}\n", self.contributions).as_str();

        let target_contribution: f32 = {
            if self.contributions.is_empty() {
                0.0
            } else {
                self.contributions
                    .iter()
                    .map(|x| x.contribution)
                    .sum::<f32>()
                    / self.contributions.len() as f32
            }
        };

        self.output += format!("{:?}\n", target_contribution).as_str();


        // People with contribution == target_contribution can't contribute to cash flow
        self.contributions
            .retain(|entry| entry.contribution != target_contribution);
        
        // self.contributions
        //     .sort_by(|a, b| b.contribution.partial_cmp(&a.contribution).unwrap());
        
        let people_who_have_payed_surplus: Vec<&Contribution> = self.contributions.iter().filter(|&p| p.contribution - target_contribution > 0.0).collect();
        let people_who_have_to_pay: Vec<&Contribution> = self.contributions.iter().filter(|&p| p.contribution - target_contribution < 0.0).collect();
        self.output += format!("Surplus Payers: {:?}\nDebters: {:?}\n", people_who_have_payed_surplus, people_who_have_to_pay).as_str();
        
    }
}
