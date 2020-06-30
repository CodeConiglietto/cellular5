use crate::{mutagen_args::*,constants::*};
use rand::prelude::*;
//One of these for each one-way colour relation
#[derive(Clone, Copy)]
pub struct Rule {
    pub life_neighbours: [bool; MAX_NEIGHBOUR_ARRAY_COUNT], //How many neighbours we need to be born
    pub death_neighbours: [bool; MAX_NEIGHBOUR_ARRAY_COUNT], //How many neighbours we need to be killed
}

//One of these per colour
#[derive(Clone, Copy)]
pub struct RuleSet {
    pub rules: [Rule; MAX_COLORS],
}

pub fn generate_random_neighbour_list() -> [bool; MAX_NEIGHBOUR_ARRAY_COUNT] {
    [
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
    ]
}

pub fn generate_random_rule() -> Rule {
    Rule {
        life_neighbours: generate_random_neighbour_list(),
        death_neighbours: generate_random_neighbour_list(),
    }
}

pub fn generate_random_rule_set() -> RuleSet {
    RuleSet {
        rules: [
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
        ],
    }
}

pub fn mutate_rule_set(rule_set: &mut RuleSet) {
    rule_set.rules[random::<usize>() % MAX_COLORS].life_neighbours
        [random::<usize>() % MAX_NEIGHBOUR_ARRAY_COUNT] = random::<bool>();
    rule_set.rules[random::<usize>() % MAX_COLORS].death_neighbours
        [random::<usize>() % MAX_NEIGHBOUR_ARRAY_COUNT] = random::<bool>();
}
