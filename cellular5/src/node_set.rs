use generational_arena::*;
use mutagen::{Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{arena_wrappers::*, prelude::*, ArenaSlot};

#[derive(Default, Debug, UpdatableRecursively, Serialize, Deserialize)]
pub struct NodeSet {
    //color_blend
    color_blend_nodes: Metarena<ColorBlendNodes>,
    //color
    generic_color_nodes: Metarena<GenericColorNodes>,
    bit_color_nodes: Metarena<BitColorNodes>,
    byte_color_nodes: Metarena<ByteColorNodes>,
    float_color_nodes: Metarena<FloatColorNodes>,
    hsv_color_nodes: Metarena<HSVColorNodes>,
    cmyk_color_nodes: Metarena<CMYKColorNodes>,
    lab_color_nodes: Metarena<LABColorNodes>,
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
    uint_nodes: Metarena<UIntNodes>,
    sint_nodes: Metarena<SIntNodes>,
    //matrix
    snfloat_matrix3_nodes: Metarena<SNFloatMatrix3Nodes>,
    //point
    snpoint_nodes: Metarena<SNPointNodes>,
    //point_set
    point_set_nodes: Metarena<PointSetNodes>,
    //iterative_function
    iterative_function_nodes: Metarena<IterativeFunctionNodes>,
    //complex
    sncomplex_nodes: Metarena<SNComplexNodes>,
    //constraint_resolvers
    sfloat_normaliser_nodes: Metarena<SFloatNormaliserNodes>,
    ufloat_normaliser_nodes: Metarena<UFloatNormaliserNodes>,
    //frame_renderers
    frame_renderer_nodes: Metarena<FrameRendererNodes>,
}

impl NodeSet {
    pub fn new() -> NodeSet {
        Self::default()
    }

    //ensure this is updated for accurate counts. Don't rely on this for anything
    pub fn count_all(&self) -> usize {
        self.color_blend_nodes.len()
            + self.bit_color_nodes.len()
            + self.byte_color_nodes.len()
            + self.float_color_nodes.len()
            + self.hsv_color_nodes.len()
            + self.cmyk_color_nodes.len()
            + self.lab_color_nodes.len()
            + self.angle_nodes.len()
            + self.unfloat_nodes.len()
            + self.snfloat_nodes.len()
            + self.coord_map_nodes.len()
            + self.boolean_nodes.len()
            + self.nibble_nodes.len()
            + self.byte_nodes.len()
            + self.uint_nodes.len()
            + self.sint_nodes.len()
            + self.snfloat_matrix3_nodes.len()
            + self.snpoint_nodes.len()
            + self.point_set_nodes.len()
            + self.iterative_function_nodes.len()
            + self.sncomplex_nodes.len()
            + self.sfloat_normaliser_nodes.len()
            + self.ufloat_normaliser_nodes.len()
            + self.frame_renderer_nodes.len()
    }
}

impl<'a> Updatable<'a> for NodeSet {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl Storage<ColorBlendNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<ColorBlendNodes>> {
        &self.color_blend_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<ColorBlendNodes>> {
        &mut self.color_blend_nodes.value
    }
}

impl Storage<GenericColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<GenericColorNodes>> {
        &self.generic_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<GenericColorNodes>> {
        &mut self.generic_color_nodes.value
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

impl Storage<HSVColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<HSVColorNodes>> {
        &self.hsv_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<HSVColorNodes>> {
        &mut self.hsv_color_nodes.value
    }
}

impl Storage<CMYKColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<CMYKColorNodes>> {
        &self.cmyk_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<CMYKColorNodes>> {
        &mut self.cmyk_color_nodes.value
    }
}

impl Storage<LABColorNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<LABColorNodes>> {
        &self.lab_color_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<LABColorNodes>> {
        &mut self.lab_color_nodes.value
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

impl Storage<SNFloatMatrix3Nodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNFloatMatrix3Nodes>> {
        &self.snfloat_matrix3_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNFloatMatrix3Nodes>> {
        &mut self.snfloat_matrix3_nodes.value
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

impl Storage<IterativeFunctionNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<IterativeFunctionNodes>> {
        &self.iterative_function_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<IterativeFunctionNodes>> {
        &mut self.iterative_function_nodes.value
    }
}

impl Storage<SNComplexNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SNComplexNodes>> {
        &self.sncomplex_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SNComplexNodes>> {
        &mut self.sncomplex_nodes.value
    }
}

impl Storage<SFloatNormaliserNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SFloatNormaliserNodes>> {
        &self.sfloat_normaliser_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SFloatNormaliserNodes>> {
        &mut self.sfloat_normaliser_nodes.value
    }
}

impl Storage<UFloatNormaliserNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<UFloatNormaliserNodes>> {
        &self.ufloat_normaliser_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<UFloatNormaliserNodes>> {
        &mut self.ufloat_normaliser_nodes.value
    }
}

impl Storage<UIntNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<UIntNodes>> {
        &self.uint_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<UIntNodes>> {
        &mut self.uint_nodes.value
    }
}

impl Storage<SIntNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<SIntNodes>> {
        &self.sint_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<SIntNodes>> {
        &mut self.sint_nodes.value
    }
}

impl Storage<FrameRendererNodes> for NodeSet {
    fn arena(&self) -> &Arena<ArenaSlot<FrameRendererNodes>> {
        &self.frame_renderer_nodes.value
    }

    fn arena_mut(&mut self) -> &mut Arena<ArenaSlot<FrameRendererNodes>> {
        &mut self.frame_renderer_nodes.value
    }
}
