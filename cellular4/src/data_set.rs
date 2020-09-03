use crate::{
    arena_wrappers::*,
    coordinate_set::*,
    datatype::{
        colors::*, continuous::*, discrete::*, distance_functions::*, matrices::*,
        noisefunctions::*, point_sets::*, points::*,
    },
};

use generational_arena::*;

#[derive(Debug)]
pub struct DataSet {
    //color
    bit_colors: Arena<ArenaSlot<BitColor>>,
    byte_colors: Arena<ArenaSlot<ByteColor>>,
    float_colors: Arena<ArenaSlot<FloatColor>>,
    //continuous
    angles: Arena<ArenaSlot<Angle>>,
    unfloats: Arena<ArenaSlot<UNFloat>>,
    snfloats: Arena<ArenaSlot<SNFloat>>,
    //coord_set
    coordinate_sets: Arena<ArenaSlot<CoordinateSet>>,
    //discrete
    booleans: Arena<ArenaSlot<Boolean>>,
    nibbles: Arena<ArenaSlot<Nibble>>,
    bytes: Arena<ArenaSlot<Byte>>,
    //distance_function
    distance_functions: Arena<ArenaSlot<DistanceFunction>>,
    //matrix
    snfloat_matrix3s: Arena<ArenaSlot<SNFloatMatrix3>>,
    //noise
    noise_functions: Arena<ArenaSlot<NoiseFunctions>>,
    //point
    snpoints: Arena<ArenaSlot<SNPoint>>,
    //point_set
    point_sets: Arena<ArenaSlot<PointSet>>,
}

impl DataSet {
    pub fn new() -> Self {
        DataSet {
            //color
            bit_colors: Arena::new(),
            byte_colors: Arena::new(),
            float_colors: Arena::new(),
            //continuous
            angles: Arena::new(),
            unfloats: Arena::new(),
            snfloats: Arena::new(),
            //coord_set
            coordinate_sets: Arena::new(),
            //discrete
            booleans: Arena::new(),
            nibbles: Arena::new(),
            bytes: Arena::new(),
            //distance_function
            distance_functions: Arena::new(),
            //matrix
            snfloat_matrix3s: Arena::new(),
            //noise
            noise_functions: Arena::new(),
            //point
            snpoints: Arena::new(),
            //point_set
            point_sets: Arena::new(),
        }
    }
}

impl Default for DataSet {
    fn default() -> Self {
        Self::new()
    }
}

//example
// impl Storage<FloatColor> for DataSet {
//     fn arena(&self) -> &Arena<ArenaSlot<FloatColor>> {
//         &self.float_colors
//     }

//     fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<FloatColor>> {
//         &mut self.float_colors
//     }
// }

impl Storage<BitColor> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<BitColor>> {
        &self.bit_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<BitColor>> {
        &mut self.bit_colors
    }
}
impl Storage<ByteColor> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<ByteColor>> {
        &self.byte_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<ByteColor>> {
        &mut self.byte_colors
    }
}
impl Storage<FloatColor> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<FloatColor>> {
        &self.float_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<FloatColor>> {
        &mut self.float_colors
    }
}
impl Storage<Angle> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<Angle>> {
        &self.angles
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<Angle>> {
        &mut self.angles
    }
}
impl Storage<UNFloat> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<UNFloat>> {
        &self.unfloats
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<UNFloat>> {
        &mut self.unfloats
    }
}
impl Storage<SNFloat> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNFloat>> {
        &self.snfloats
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNFloat>> {
        &mut self.snfloats
    }
}
impl Storage<CoordinateSet> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<CoordinateSet>> {
        &self.coordinate_sets
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<CoordinateSet>> {
        &mut self.coordinate_sets
    }
}
impl Storage<Boolean> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<Boolean>> {
        &self.booleans
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<Boolean>> {
        &mut self.booleans
    }
}
impl Storage<Nibble> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<Nibble>> {
        &self.nibbles
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<Nibble>> {
        &mut self.nibbles
    }
}
impl Storage<Byte> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<Byte>> {
        &self.bytes
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<Byte>> {
        &mut self.bytes
    }
}
impl Storage<DistanceFunction> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<DistanceFunction>> {
        &self.distance_functions
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<DistanceFunction>> {
        &mut self.distance_functions
    }
}
impl Storage<SNFloatMatrix3> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNFloatMatrix3>> {
        &self.snfloat_matrix3s
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNFloatMatrix3>> {
        &mut self.snfloat_matrix3s
    }
}
impl Storage<NoiseFunctions> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<NoiseFunctions>> {
        &self.noise_functions
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<NoiseFunctions>> {
        &mut self.noise_functions
    }
}
impl Storage<SNPoint> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNPoint>> {
        &self.snpoints
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNPoint>> {
        &mut self.snpoints
    }
}
impl Storage<PointSet> for DataSet {
    fn arena(&self) -> &Arena<ArenaSlot<PointSet>> {
        &self.point_sets
    }
    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<PointSet>> {
        &mut self.point_sets
    }
}
