use std::clone;
use std::collections::HashSet;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, IndexMut};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Contribution {
    name: String,
    contribution: f32,
}

impl PartialEq for Contribution {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.contribution == other.contribution
    }
}

impl Eq for Contribution {}

impl Hash for Contribution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}{}", self.name, self.contribution).hash(state);
    }
}

impl Contribution {
    fn update(&mut self, amount: f32) {
        self.contribution += amount;
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct ExpenseCalculatorApp {
    pub input: String,
    pub contributions: Vec<Contribution>,
    pub output: String,
    pub transaction_history: Vec<(String, String, f32)>,
}

impl ExpenseCalculatorApp {
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

        let mut people_who_have_payed_surplus: Vec<Contribution> = vec![];
        for entry in &self.contributions {
            if entry.contribution > target_contribution {
                people_who_have_payed_surplus.push(Contribution {
                    name: entry.name.clone(),
                    contribution: entry.contribution - target_contribution,
                })
            }
        }
        let mut people_who_have_to_pay: Vec<Contribution> = vec![];
        for entry in &self.contributions {
            if entry.contribution < target_contribution {
                people_who_have_to_pay.push(Contribution {
                    name: entry.name.clone(),
                    contribution: entry.contribution - target_contribution,
                })
            }
        }
        self.output += format!(
            "Surplus Payers: {:?}\nDebters: {:?}\n",
            people_who_have_payed_surplus, people_who_have_to_pay
        )
        .as_str();

        self.transaction_history = vec![];

        // Case where only one person has payed surplus
        if people_who_have_payed_surplus.len() == 1 {
            for entry in &people_who_have_to_pay {
                self.transaction_history.push((
                    entry.name.clone(),
                    people_who_have_payed_surplus[0].name.clone(),
                    -entry.contribution,
                ));
            }
            return;
        } else {
            let mut index_to_remove = Vec::new();
            let mut surplus_set = HashSet::new();
            let mut debt_set = HashSet::new();

            let people_who_have_payed_surplus_clone = people_who_have_payed_surplus.clone();
            let people_who_have_to_pay_clone = people_who_have_to_pay.clone();

            // Iterate over indices to avoid borrow checker issues
            for (i, entry1) in people_who_have_payed_surplus.iter().enumerate() {
                for (j, entry2) in people_who_have_to_pay.iter().enumerate() {
                    if entry1.contribution == -entry2.contribution
                        && !surplus_set.contains(entry1)
                        && !debt_set.contains(entry2)
                    {
                        // Found a match, mark the indices to remove
                        self.transaction_history.push((
                            entry2.name.clone(),
                            entry1.name.clone(),
                            entry1.contribution,
                        ));
                        surplus_set.insert(entry1.clone());
                        debt_set.insert(entry2.clone());
                        index_to_remove.push((i, j));
                    }
                }
            }

            let mut people_who_have_payed_surplus_filtered: Vec<Contribution> = Vec::new();
            let mut people_who_have_to_pay_filtered = Vec::new();

            for entry in people_who_have_payed_surplus_clone {
                if !surplus_set.contains(&entry) {
                    people_who_have_payed_surplus_filtered.push(entry);
                }
            }

            for entry in people_who_have_to_pay_clone {
                if !debt_set.contains(&entry) {
                    people_who_have_to_pay_filtered.push(entry.clone());
                }
            }

            people_who_have_payed_surplus_filtered
                .sort_by(|a, b| b.contribution.partial_cmp(&a.contribution).unwrap());
            people_who_have_to_pay_filtered
                .sort_by(|a, b| a.contribution.partial_cmp(&b.contribution).unwrap());

            self.output += format!(
                "Removing matching:\nSurplus Payers: {:?}\nDebters: {:?}\n",
                people_who_have_payed_surplus_filtered, people_who_have_to_pay_filtered
            )
            .as_str();

            // Now the messy part..., bin packing
            let mut surplus_index = 0;
            let mut debt_index = 0;

            while surplus_index < people_who_have_payed_surplus_filtered.len()
                && debt_index < people_who_have_to_pay_filtered.len()
            {
                let surplus_candidate = &mut people_who_have_payed_surplus_filtered[surplus_index];
                let debt_candidate = &mut people_who_have_to_pay_filtered[debt_index];
                if surplus_candidate.contribution < -debt_candidate.contribution {
                    self.transaction_history.push((
                        debt_candidate.name.clone(),
                        surplus_candidate.name.clone(),
                        surplus_candidate.contribution,
                    ));
                    surplus_index += 1;
                    debt_candidate.contribution += surplus_candidate.contribution;
                } else {
                    self.transaction_history.push((
                        debt_candidate.name.clone(),
                        surplus_candidate.name.clone(),
                        -debt_candidate.contribution,
                    ));
                    debt_index += 1;
                    surplus_candidate.contribution += debt_candidate.contribution;
                }
            }
        }
        self.output += format!("{:?}\n", self.transaction_history).as_str();
    }
}
