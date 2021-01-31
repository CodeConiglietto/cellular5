use crate::{
    constants::*,
    datatype::{constraint_resolvers::*, continuous::*, discrete::*, points::*},
};

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
    pub fn get_coord_shifted(
        self,
        shift_x: SNFloat,
        shift_y: SNFloat,
        shift_t: SNFloat,
        normaliser: SFloatNormaliser,
    ) -> Self {
        CoordinateSet {
            x: normaliser.normalise(self.x.into_inner() + shift_x.into_inner()),
            y: normaliser.normalise(self.y.into_inner() + shift_y.into_inner()),
            t: self.t + shift_t.into_inner(),
        }
    }

    pub fn get_coord_scaled(
        self,
        scale_x: SNFloat,
        scale_y: SNFloat,
        scale_t: SNFloat,
        normaliser: SFloatNormaliser,
    ) -> Self {
        CoordinateSet {
            x: normaliser.normalise(self.x.into_inner() * scale_x.into_inner()),
            y: normaliser.normalise(self.y.into_inner() * scale_y.into_inner()),
            t: self.t * scale_t.into_inner(),
        }
    }

    pub fn get_coord_point(&self) -> SNPoint {
        SNPoint::from_snfloats(self.x, self.y)
    }

    //Gonna hack this to make it a triangle wave instead of a sawtooth
    pub fn get_byte_t(&self) -> Byte {
        let mut val = self.t as i64 % (CONSTS.byte_possible_values as i64 * 2);

        val = 256 - (256 - val).abs();

        Byte::new(val as u8)
    }

    //todo refactor divisor into constant
    pub fn get_unfloat_t(&self) -> UNFloat {
        UNFloat::new_triangle(self.t / 10.0 / CONSTS.time_scale_divisor)
    }
}
