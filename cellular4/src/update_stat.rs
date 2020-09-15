use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

use rand::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct UpdateStat {
    //Update stats are used to determine an approximation of the entropy of the current state
    //Update stats contain many values:
    //-Active cell count
    //--If the active cell count is high, we have a lot of change
    //--If the active cell count is low, we have a small amount of change
    //-Neighbour similarity
    //--If all neighbours are similar, we have close to a flat color
    //--If all neighbours are distinct, we have visual noise
    //-Plus a bunch more, it's decently self explanatory
    pub activity_value: f64,
    pub alpha_value: f64,
    pub local_similarity_value: f64,
    pub global_similarity_value: f64,
    pub cpu_usage: f64,
}

//TODO: add more heuristics of undesirability to inform mutation probability
//Sharpness
//Colourfulness
//
impl UpdateStat {
    pub fn should_mutate(&self) -> bool {
        (thread_rng().gen::<f64>() * self.mutation_likelihood()).powf(2.0) > thread_rng().gen::<f64>()
    }

    // pub fn should_mutate(&self) -> bool {
        //     self.activity_value < CONSTS.activity_value_lower_bound
        //         || self.alpha_value < CONSTS.alpha_value_lower_bound
        //         || self.local_similarity_value > CONSTS.local_similarity_upper_bound
        //         || self.global_similarity_value >= CONSTS.global_similarity_upper_bound
        // }

    pub fn mutation_likelihood(&self) -> f64 {
        (self.flatness() + self.noise() + self.stagnation() + self.transparency()) / 4.0
    }

    pub fn flatness(&self) -> f64 {
        ((1.0 - self.activity_value).powf(2.0) + self.local_similarity_value.powf(2.0) + self.global_similarity_value.powf(2.0)) / 3.0
    }

    pub fn noise(&self) -> f64 {
        (self.activity_value.powf(2.0) + (1.0 - self.local_similarity_value).powf(2.0)) / 2.0
    }

    pub fn stagnation(&self) -> f64 {
        (1.0 - self.activity_value).powf(2.0)
    }

    pub fn transparency(&self) -> f64 {
        (1.0 - self.alpha_value).powf(2.0)
    }

    //Function for dealing with floating point precision issues.
    pub fn clamp_values(self) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value.min(1.0).max(0.0),
            alpha_value: self.alpha_value.min(1.0).max(0.0),
            local_similarity_value: self.local_similarity_value.min(1.0).max(0.0),
            global_similarity_value: self.global_similarity_value.min(1.0).max(0.0),
            cpu_usage: self.cpu_usage.min(1.0).max(0.0),
        }
    }
}

impl Add<UpdateStat> for UpdateStat {
    type Output = UpdateStat;

    fn add(self, other: UpdateStat) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value + other.activity_value,
            alpha_value: self.alpha_value + other.alpha_value,
            local_similarity_value: self.local_similarity_value + other.local_similarity_value,
            global_similarity_value: self.global_similarity_value + other.global_similarity_value,
            cpu_usage: self.cpu_usage + other.cpu_usage,
        }
    }
}

impl Div<f64> for UpdateStat {
    type Output = UpdateStat;

    fn div(self, other: f64) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value / other,
            alpha_value: self.alpha_value / other,
            local_similarity_value: self.local_similarity_value / other,
            global_similarity_value: self.global_similarity_value / other,
            cpu_usage: self.cpu_usage / other,
        }
    }
}

impl Sum for UpdateStat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(UpdateStat::default(), |a, b| a + b)
    }
}

impl AddAssign<UpdateStat> for UpdateStat {
    fn add_assign(&mut self, other: UpdateStat) {
        *self = *self + other;
    }
}
