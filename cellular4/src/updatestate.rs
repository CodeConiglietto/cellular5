use crate::{
    constants::*,
    datatype::{continuous::*, discrete::*, points::*},
    History,
};

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //the set of coordinates for the update
    pub coordinate_set: CoordinateSet,
    //cell array to read from
    pub history: &'a History,
}

impl UpdateState<'_> {
    pub fn replace_coords(self, other: &SNPoint) -> Self {
        let mut new = self.clone();

        new.coordinate_set.x = other.x();
        new.coordinate_set.y = other.y();

        new
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CoordinateSet {
    //coordinates of update position
    //Needs to be floating point to allow for proper scaling
    pub x: SNFloat,
    pub y: SNFloat,
    //current game sync tic
    pub t: f32,
}

impl CoordinateSet {
    pub fn get_coord_shifted(self, shift_x: SNFloat, shift_y: SNFloat, shift_t: SNFloat) -> Self {
        CoordinateSet {
            x: self.x.sawtooth_add(shift_x),
            y: self.y.sawtooth_add(shift_y),
            t: self.t + shift_t.into_inner(),
        }
    }

    pub fn get_coord_scaled(self, scale_x: SNFloat, scale_y: SNFloat, scale_t: SNFloat) -> Self {
        CoordinateSet {
            x: SNFloat::new(self.x.into_inner() * scale_x.into_inner()),
            y: SNFloat::new(self.y.into_inner() * scale_y.into_inner()),
            t: self.t * scale_t.into_inner(),
        }
    }

    pub fn get_coord_point(self) -> SNPoint {
        SNPoint::from_snfloats(self.x, self.y)
    }

    pub fn get_byte_t(&self) -> Byte {
        Byte::new((self.t as u64 % CONSTS.byte_possible_values as u64) as u8)
    }

    //todo refactor into constant
    pub fn get_unfloat_t(&self) -> UNFloat {
        UNFloat::new_triangle(self.t / 500.0)
    }

    pub fn xy(&self) -> SNPoint {
        SNPoint::from_snfloats(self.x, self.y)
    }
}
