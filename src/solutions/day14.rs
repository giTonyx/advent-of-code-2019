use crate::solver::Solver;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Reaction>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .map(|l| l.unwrap())
            .map(|l| Reaction::from_string(l))
            .collect::<Vec<Reaction>>()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut factory = Factory::new(input);
        factory.produce_one("FUEL".to_string())
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut factory = Factory::new(input);
        let mut fuel = 0;
        let mut total_ore: i64 = 1_000_000_000_000;

        let boostrap = 1670000; // to speed up things. Could have been done better.
        total_ore -= factory.produce("FUEL".to_string(), boostrap) as i64;
        fuel += boostrap;

        loop {
            total_ore -= factory.produce_one("FUEL".to_string()) as i64;
            if total_ore < 0 {
                break;
            }
            fuel += 1;
        }
        fuel
    }
}

struct Factory {
    recipes: HashMap<String, Reaction>,
    leftovers: HashMap<String, u64>,
}

impl Factory {
    pub fn new(reactions: &Vec<Reaction>) -> Factory {
        let mut recipe_map = HashMap::new();
        for reaction in reactions {
            recipe_map.insert(reaction.output.name.clone(), reaction.clone());
        }
        Factory {
            recipes: recipe_map,
            leftovers: HashMap::new(),
        }
    }

    fn consume(&mut self, element: String, quantity: u64) -> u64 {
        let quantity_in_stock = min(
            quantity,
            self.leftovers
                .get(&element.clone())
                .unwrap_or(&0u64)
                .clone(),
        );

        self.leftovers.insert(
            element.clone(),
            self.leftovers.get(&element).unwrap_or(&0u64).clone() - quantity_in_stock,
        );
        quantity - quantity_in_stock
    }

    fn store(&mut self, element: String, quantity: u64) {
        if quantity > 0 {
            self.leftovers.insert(
                element.clone(),
                self.leftovers.get(&element).unwrap_or(&0u64).clone() + quantity,
            );
        }
    }

    pub fn produce_one(&mut self, element: String) -> u64 {
        self.produce(element, 1)
    }

    pub fn produce(&mut self, element: String, quantity: u64) -> u64 {
        let mut ore_cost = 0;
        let mut queue = VecDeque::new();
        queue.push_back(Ingredient {
            name: element.clone(),
            count: quantity,
        });

        while !queue.is_empty() {
            let next_element = queue.pop_front().unwrap();
            let needed = self.consume(next_element.name.clone(), next_element.count);
            if needed > 0 {
                let reaction = self
                    .recipes
                    .get(&next_element.name.clone())
                    .unwrap()
                    .clone();
                let multiplier = needed / reaction.output.count
                    + if (needed % reaction.output.count) == 0 {
                        0
                    } else {
                        1
                    };
                self.store(
                    next_element.name.clone(),
                    (multiplier * reaction.output.count) - needed,
                );
                for ingredient in &reaction.ingredients {
                    if ingredient.name == "ORE".to_string() {
                        ore_cost += ingredient.count * multiplier;
                    } else {
                        queue.push_back(Ingredient {
                            name: ingredient.name.clone(),
                            count: ingredient.count * multiplier,
                        });
                    }
                }
            }
        }
        ore_cost
    }
}

#[derive(Clone)]
pub struct Ingredient {
    name: String,
    count: u64,
}

impl Ingredient {
    pub fn from_string(data: String) -> Ingredient {
        let parts = data.split(" ").collect::<Vec<&str>>();
        Ingredient {
            name: parts[1].trim().to_string(),
            count: parts[0].trim().parse::<u64>().unwrap(),
        }
    }
}
#[derive(Clone)]
pub struct Reaction {
    output: Ingredient,
    ingredients: Vec<Ingredient>,
}

impl Reaction {
    pub fn from_string(data: String) -> Reaction {
        let parts = data.split("=>").collect::<Vec<&str>>();
        Reaction {
            output: Ingredient::from_string(parts[1].trim().to_string()),
            ingredients: parts[0]
                .split(",")
                .map(|s| Ingredient::from_string(s.trim().to_string()))
                .collect::<Vec<Ingredient>>(),
        }
    }
}

#[test]
fn test_produce() {
    let mut reactions = Vec::new();
    reactions.push(Reaction::from_string("10 ORE => 3 TEST".to_string()));
    reactions.push(Reaction::from_string("2 TEST => 4 FUEL".to_string()));
    let mut factory = Factory::new(&reactions);
    assert!(factory.produce_one("TEST".to_string()) == 10);
    assert!(factory.produce_one("FUEL".to_string()) == 0);
}

#[test]
fn ingredient_from_string() {
    let i1 = Ingredient::from_string("10 ORE".to_string());
    assert!(i1.count == 10);
    assert!(i1.name == "ORE".to_string());
}

#[test]
fn reaction_from_string() {
    let r1 = Reaction::from_string("5 GHVJ, 1 RGKB, 1 GCTBC => 6 HKMV".to_string());
    assert!(r1.output.count == 6);
    assert!(r1.output.name == "HKMV".to_string());
    assert!(r1.ingredients.len() == 3);
}
