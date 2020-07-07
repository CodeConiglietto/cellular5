use crate::{
    arena_wrappers::*,
    coordinate_set::*,
    datatype::{buffers::*, colors::*, continuous::*, discrete::*, distance_functions::*, image::*, matrices::*, noisefunctions::*, point_sets::*, points::*},
};

use generational_arena::*;

#[derive(Debug)]
pub struct DataSet {
    //color
    bit_colors: Arena<BitColor>,
    byte_colors: Arena<ByteColor>,
    float_colors: Arena<FloatColor>,
    //continuous
    angles: Arena<Angle>,
    unfloats: Arena<UNFloat>,
    snfloats: Arena<SNFloat>,
    //coord_set
    coordinate_sets: Arena<CoordinateSet>,
    //discrete
    booleans: Arena<Boolean>,
    nibbles: Arena<Nibble>,
    bytes: Arena<Byte>,
    //distance_function
    distance_functions: Arena<DistanceFunction>,
    //matrix
    snfloat_matrix3s: Arena<SNFloatMatrix3>,
    //noise
    noise_functions: Arena<NoiseFunctions>,
    //point
    snpoints: Arena<SNPoint>,
    //point_set
    point_sets: Arena<PointSet>,
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

//example
// impl Storage<FloatColor> for DataSet {
//     fn arena(&self) -> &Arena<FloatColor> {
//         &self.float_colors
//     }

//     fn arena_mut(&mut self) -> &mut Arena<FloatColor> {
//         &mut self.float_colors
//     }
// }

impl Storage<BitColor> for DataSet {
    fn arena(&self) -> &Arena<BitColor> {
        &self.bit_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<BitColor> {
        &mut self.bit_colors
    }
}
impl Storage<ByteColor> for DataSet {
    fn arena(&self) -> &Arena<ByteColor> {
        &self.byte_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<ByteColor> {
        &mut self.byte_colors
    }
}
impl Storage<FloatColor> for DataSet {
    fn arena(&self) -> &Arena<FloatColor> {
        &self.float_colors
    }
    fn arena_mut(&mut self) -> &mut Arena<FloatColor> {
        &mut self.float_colors
    }
}
impl Storage<Angle> for DataSet {
    fn arena(&self) -> &Arena<Angle> {
        &self.angles
    }
    fn arena_mut(&mut self) -> &mut Arena<Angle> {
        &mut self.angles
    }
}
impl Storage<UNFloat> for DataSet {
    fn arena(&self) -> &Arena<UNFloat> {
        &self.unfloats
    }

    fn arena_mut(&mut self) -> &mut Arena<UNFloat> {
        &mut self.unfloats
    }
}
impl Storage<SNFloat> for DataSet {
    fn arena(&self) -> &Arena<SNFloat> {
        &self.snfloats
    }
    fn arena_mut(&mut self) -> &mut Arena<SNFloat> {
        &mut self.snfloats
    }
}
impl Storage<CoordinateSet> for DataSet {
    fn arena(&self) -> &Arena<CoordinateSet> {
        &self.coordinate_sets
    }
    fn arena_mut(&mut self) -> &mut Arena<CoordinateSet> {
        &mut self.coordinate_sets
    }
}
impl Storage<Boolean> for DataSet {
    fn arena(&self) -> &Arena<Boolean> {
        &self.booleans
    }
    fn arena_mut(&mut self) -> &mut Arena<Boolean> {
        &mut self.booleans
    }
}
impl Storage<Nibble> for DataSet {
    fn arena(&self) -> &Arena<Nibble> {
        &self.nibbles
    }
    fn arena_mut(&mut self) -> &mut Arena<Nibble> {
        &mut self.nibbles
    }
}
impl Storage<Byte> for DataSet {
    fn arena(&self) -> &Arena<Byte> {
        &self.bytes
    }
    fn arena_mut(&mut self) -> &mut Arena<Byte> {
        &mut self.bytes
    }
}
impl Storage<DistanceFunction> for DataSet {
    fn arena(&self) -> &Arena<DistanceFunction> {
        &self.distance_functions
    }
    fn arena_mut(&mut self) -> &mut Arena<DistanceFunction> {
        &mut self.distance_functions
    }
}
impl Storage<SNFloatMatrix3> for DataSet {
    fn arena(&self) -> &Arena<SNFloatMatrix3> {
        &self.snfloat_matrix3s
    }
    fn arena_mut(&mut self) -> &mut Arena<SNFloatMatrix3> {
        &mut self.snfloat_matrix3s
    }
}
impl Storage<NoiseFunctions> for DataSet {
    fn arena(&self) -> &Arena<NoiseFunctions> {
        &self.noise_functions
    }
    fn arena_mut(&mut self) -> &mut Arena<NoiseFunctions> {
        &mut self.noise_functions
    }
}
impl Storage<SNPoint> for DataSet {
    fn arena(&self) -> &Arena<SNPoint> {
        &self.snpoints
    }
    fn arena_mut(&mut self) -> &mut Arena<SNPoint> {
        &mut self.snpoints
    }
}
impl Storage<PointSet> for DataSet {
    fn arena(&self) -> &Arena<PointSet> {
        &self.point_sets
    }
    fn arena_mut(&mut self) -> &mut Arena<PointSet> {
        &mut self.point_sets
    }
}