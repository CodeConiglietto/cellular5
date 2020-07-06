use crate::{
    arena_wrappers::*,
    node::{
        color_blend_nodes::*, color_nodes::*, continuous_nodes::*, coord_map_nodes::*,
        discrete_nodes::*, distance_function_nodes::*, matrix_nodes::*, noise_nodes::*,
        point_nodes::*, point_set_nodes::*,
    },
};
use generational_arena::*;

#[derive(Debug)]
pub struct NodeSet {
    //color_blend
    color_blend_nodes: Arena<ColorBlendNodes>,
    //color
    bit_color_nodes: Arena<BitColorNodes>,
    byte_color_nodes: Arena<ByteColorNodes>,
    float_color_nodes: Arena<FloatColorNodes>,
    //continuous
    angle_nodes: Arena<AngleNodes>,
    unfloat_nodes: Arena<UNFloatNodes>,
    snfloat_nodes: Arena<SNFloatNodes>,
    //coord_map
    coord_map_nodes: Arena<CoordMapNodes>,
    //discrete
    boolean_nodes: Arena<BooleanNodes>,
    nibble_nodes: Arena<NibbleNodes>,
    byte_nodes: Arena<ByteNodes>,
    //distance_function
    distance_function_nodes: Arena<DistanceFunctionNodes>,
    //matrix
    snfloat_matrix3_nodes: Arena<SNFloatMatrix3Nodes>,
    //noise
    noise_nodes: Arena<NoiseNodes>,
    //point
    snpoint_nodes: Arena<SNPointNodes>,
    //point_set
    point_set_nodes: Arena<PointSetNodes>,
}

impl Storage<ColorBlendNodes> for NodeSet {
    fn arena(&self) -> &Arena<ColorBlendNodes> {
        &self.color_blend_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<ColorBlendNodes> {
        &mut self.color_blend_nodes
    }
}

impl Storage<BitColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<BitColorNodes> {
        &self.bit_color_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<BitColorNodes> {
        &mut self.bit_color_nodes
    }
}

impl Storage<ByteColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ByteColorNodes> {
        &self.byte_color_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<ByteColorNodes> {
        &mut self.byte_color_nodes
    }
}

impl Storage<FloatColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<FloatColorNodes> {
        &self.float_color_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<FloatColorNodes> {
        &mut self.float_color_nodes
    }
}

impl Storage<AngleNodes> for NodeSet {
    fn arena(&self) -> &Arena<AngleNodes> {
        &self.angle_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<AngleNodes> {
        &mut self.angle_nodes
    }
}

impl Storage<UNFloatNodes> for NodeSet {
    fn arena(&self) -> &Arena<UNFloatNodes> {
        &self.unfloat_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<UNFloatNodes> {
        &mut self.unfloat_nodes
    }
}

impl Storage<SNFloatNodes> for NodeSet {
    fn arena(&self) -> &Arena<SNFloatNodes> {
        &self.snfloat_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<SNFloatNodes> {
        &mut self.snfloat_nodes
    }
}

impl Storage<CoordMapNodes> for NodeSet {
    fn arena(&self) -> &Arena<CoordMapNodes> {
        &self.coord_map_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<CoordMapNodes> {
        &mut self.coord_map_nodes
    }
}

impl Storage<BooleanNodes> for NodeSet {
    fn arena(&self) -> &Arena<BooleanNodes> {
        &self.boolean_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<BooleanNodes> {
        &mut self.boolean_nodes
    }
}

impl Storage<NibbleNodes> for NodeSet {
    fn arena(&self) -> &Arena<NibbleNodes> {
        &self.nibble_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<NibbleNodes> {
        &mut self.nibble_nodes
    }
}

impl Storage<ByteNodes> for NodeSet {
    fn arena(&self) -> &Arena<ByteNodes> {
        &self.byte_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<ByteNodes> {
        &mut self.byte_nodes
    }
}

impl Storage<DistanceFunctionNodes> for NodeSet {
    fn arena(&self) -> &Arena<DistanceFunctionNodes> {
        &self.distance_function_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<DistanceFunctionNodes> {
        &mut self.distance_function_nodes
    }
}

impl Storage<SNFloatMatrix3Nodes> for NodeSet {
    fn arena(&self) -> &Arena<SNFloatMatrix3Nodes> {
        &self.snfloat_matrix3_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<SNFloatMatrix3Nodes> {
        &mut self.snfloat_matrix3_nodes
    }
}

impl Storage<NoiseNodes> for NodeSet {
    fn arena(&self) -> &Arena<NoiseNodes> {
        &self.noise_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<NoiseNodes> {
        &mut self.noise_nodes
    }
}

impl Storage<SNPointNodes> for NodeSet {
    fn arena(&self) -> &Arena<SNPointNodes> {
        &self.snpoint_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<SNPointNodes> {
        &mut self.snpoint_nodes
    }
}

impl Storage<PointSetNodes> for NodeSet {
    fn arena(&self) -> &Arena<PointSetNodes> {
        &self.point_set_nodes
    }

    fn arena_mut(&mut self) -> &mut Arena<PointSetNodes> {
        &mut self.point_set_nodes
    }
}
