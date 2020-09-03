use crate::{
    arena_wrappers::*,
    mutagen_args::*,
    node::{
        color_blend_nodes::*, color_nodes::*, continuous_nodes::*, coord_map_nodes::*,
        discrete_nodes::*, distance_function_nodes::*, matrix_nodes::*, noise_nodes::*,
        point_nodes::*, point_set_nodes::*,
    },
};
use generational_arena::*;

use mutagen::{State, Updatable, UpdatableRecursively};

#[derive(Debug, UpdatableRecursively)]
pub struct NodeSet {
    //color_blend
    color_blend_nodes: Metarena<ColorBlendNodes>,
    //color
    bit_color_nodes: Metarena<BitColorNodes>,
    byte_color_nodes: Metarena<ByteColorNodes>,
    float_color_nodes: Metarena<FloatColorNodes>,
    //continuous
    angle_nodes: Metarena<AngleNodes>,
    unfloat_nodes: Metarena<UNFloatNodes>,
    snfloat_nodes: Metarena<SNFloatNodes>,
    //coord_map
    coord_map_nodes: Metarena<CoordMapNodes>,
    //discrete
    boolean_nodes: Metarena<BooleanNodes>,
    nibble_nodes: Metarena<NibbleNodes>,
    byte_nodes: Metarena<ByteNodes>,
    //distance_function
    distance_function_nodes: Metarena<DistanceFunctionNodes>,
    //matrix
    snfloat_matrix3_nodes: Metarena<SNFloatMatrix3Nodes>,
    //noise
    noise_nodes: Metarena<NoiseNodes>,
    //point
    snpoint_nodes: Metarena<SNPointNodes>,
    //point_set
    point_set_nodes: Metarena<PointSetNodes>,
}

impl NodeSet {
    pub fn new() -> NodeSet {
        NodeSet {
            //color_blend
            color_blend_nodes: Metarena::new(),
            //color
            bit_color_nodes: Metarena::new(),
            byte_color_nodes: Metarena::new(),
            float_color_nodes: Metarena::new(),
            //continuous
            angle_nodes: Metarena::new(),
            unfloat_nodes: Metarena::new(),
            snfloat_nodes: Metarena::new(),
            //coord_map
            coord_map_nodes: Metarena::new(),
            //discrete
            boolean_nodes: Metarena::new(),
            nibble_nodes: Metarena::new(),
            byte_nodes: Metarena::new(),
            //distance_function
            distance_function_nodes: Metarena::new(),
            //matrix
            snfloat_matrix3_nodes: Metarena::new(),
            //noise
            noise_nodes: Metarena::new(),
            //point
            snpoint_nodes: Metarena::new(),
            //point_set
            point_set_nodes: Metarena::new(),
        }
    }
}

impl Default for NodeSet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Updatable<'a> for NodeSet {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: State, _arg: Self::UpdateArg) {}
}

impl Storage<ColorBlendNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<ColorBlendNodes>> {
        &self.color_blend_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<ColorBlendNodes>> {
        &mut self.color_blend_nodes.value
    }
}

impl Storage<BitColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<BitColorNodes>> {
        &self.bit_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<BitColorNodes>> {
        &mut self.bit_color_nodes.value
    }
}

impl Storage<ByteColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<ByteColorNodes>> {
        &self.byte_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<ByteColorNodes>> {
        &mut self.byte_color_nodes.value
    }
}

impl Storage<FloatColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<FloatColorNodes>> {
        &self.float_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<FloatColorNodes>> {
        &mut self.float_color_nodes.value
    }
}

impl Storage<AngleNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<AngleNodes>> {
        &self.angle_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<AngleNodes>> {
        &mut self.angle_nodes.value
    }
}

impl Storage<UNFloatNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<UNFloatNodes>> {
        &self.unfloat_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<UNFloatNodes>> {
        &mut self.unfloat_nodes.value
    }
}

impl Storage<SNFloatNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNFloatNodes>> {
        &self.snfloat_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNFloatNodes>> {
        &mut self.snfloat_nodes.value
    }
}

impl Storage<CoordMapNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<CoordMapNodes>> {
        &self.coord_map_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<CoordMapNodes>> {
        &mut self.coord_map_nodes.value
    }
}

impl Storage<BooleanNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<BooleanNodes>> {
        &self.boolean_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<BooleanNodes>> {
        &mut self.boolean_nodes.value
    }
}

impl Storage<NibbleNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<NibbleNodes>> {
        &self.nibble_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<NibbleNodes>> {
        &mut self.nibble_nodes.value
    }
}

impl Storage<ByteNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<ByteNodes>> {
        &self.byte_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<ByteNodes>> {
        &mut self.byte_nodes.value
    }
}

impl Storage<DistanceFunctionNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<DistanceFunctionNodes>> {
        &self.distance_function_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<DistanceFunctionNodes>> {
        &mut self.distance_function_nodes.value
    }
}

impl Storage<SNFloatMatrix3Nodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNFloatMatrix3Nodes>> {
        &self.snfloat_matrix3_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNFloatMatrix3Nodes>> {
        &mut self.snfloat_matrix3_nodes.value
    }
}

impl Storage<NoiseNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<NoiseNodes>> {
        &self.noise_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<NoiseNodes>> {
        &mut self.noise_nodes.value
    }
}

impl Storage<SNPointNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNPointNodes>> {
        &self.snpoint_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNPointNodes>> {
        &mut self.snpoint_nodes.value
    }
}

impl Storage<PointSetNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<PointSetNodes>> {
        &self.point_set_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<PointSetNodes>> {
        &mut self.point_set_nodes.value
    }
}
