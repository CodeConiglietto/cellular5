use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

use crate::constants::CONSTS;

#[derive(Default, Debug, Clone, Copy)]
pub struct UpdateStat {
    //Update stats are used to determine an approximation of the entropy of the current state
    //Update stats contain two values:
    //-Active cell count
    //--If the active cell count is high, we have a lot of change
    //--If the active cell count is low, we have a small amount of change
    //-Neighbour similarity
    //--If all neighbours are similar, we have close to a flat color
    //--If all neighbours are distinct, we have visual noise
    pub activity_value: f64,
    pub alpha_value: f64,
    pub local_similarity_value: f64,
    pub global_similarity_value: f64,
}

impl UpdateStat {
    pub fn should_mutate(&self) -> bool {
        self.activity_value < CONSTS.activity_value_lower_bound
            || self.alpha_value < CONSTS.alpha_value_lower_bound
            || self.local_similarity_value > CONSTS.local_similarity_upper_bound
            || self.global_similarity_value >= CONSTS.global_similarity_upper_bound
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
