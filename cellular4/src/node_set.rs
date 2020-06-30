use crate::{
    arena_wrappers::*,
    node::{color_nodes::*, color_blend_nodes::*, continuous_nodes::*, coord_map_nodes::*, discrete_nodes::*, distance_function_nodes::*, matrix_nodes::*, noise_nodes::*, point_nodes::*, point_set_nodes::*},
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
    fn insert(&mut self, t: ColorBlendNodes) -> Index { self.color_blend_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&ColorBlendNodes> { self.color_blend_nodes.get(idx) }
}
impl Storage<BitColorNodes> for NodeSet {
    fn insert(&mut self, t: BitColorNodes) -> Index { self.bit_color_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&BitColorNodes> { self.bit_color_nodes.get(idx) }
}
impl Storage<ByteColorNodes> for NodeSet {
    fn insert(&mut self, t: ByteColorNodes) -> Index { self.byte_color_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&ByteColorNodes> { self.byte_color_nodes.get(idx) }
}
impl Storage<FloatColorNodes> for NodeSet {
    fn insert(&mut self, t: FloatColorNodes) -> Index { self.float_color_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&FloatColorNodes> { self.float_color_nodes.get(idx) }
}
impl Storage<AngleNodes> for NodeSet {
    fn insert(&mut self, t: AngleNodes) -> Index { self.angle_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&AngleNodes> { self.angle_nodes.get(idx) }
}
impl Storage<UNFloatNodes> for NodeSet {
    fn insert(&mut self, t: UNFloatNodes) -> Index { self.unfloat_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&UNFloatNodes> { self.unfloat_nodes.get(idx) }
}
impl Storage<SNFloatNodes> for NodeSet {
    fn insert(&mut self, t: SNFloatNodes) -> Index { self.snfloat_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&SNFloatNodes> { self.snfloat_nodes.get(idx) }
}
impl Storage<CoordMapNodes> for NodeSet {
    fn insert(&mut self, t: CoordMapNodes) -> Index { self.coord_map_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&CoordMapNodes> { self.coord_map_nodes.get(idx) }
}
impl Storage<BooleanNodes> for NodeSet {
    fn insert(&mut self, t: BooleanNodes) -> Index { self.boolean_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&BooleanNodes> { self.boolean_nodes.get(idx) }
}
impl Storage<NibbleNodes> for NodeSet {
    fn insert(&mut self, t: NibbleNodes) -> Index { self.nibble_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&NibbleNodes> { self.nibble_nodes.get(idx) }
}
impl Storage<ByteNodes> for NodeSet {
    fn insert(&mut self, t: ByteNodes) -> Index { self.byte_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&ByteNodes> { self.byte_nodes.get(idx) }
}
impl Storage<DistanceFunctionNodes> for NodeSet {
    fn insert(&mut self, t: DistanceFunctionNodes) -> Index { self.distance_function_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&DistanceFunctionNodes> { self.distance_function_nodes.get(idx) }
}
impl Storage<SNFloatMatrix3Nodes> for NodeSet {
    fn insert(&mut self, t: SNFloatMatrix3Nodes) -> Index { self.snfloat_matrix3_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&SNFloatMatrix3Nodes> { self.snfloat_matrix3_nodes.get(idx) }
}
impl Storage<NoiseNodes> for NodeSet {
    fn insert(&mut self, t: NoiseNodes) -> Index { self.noise_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&NoiseNodes> { self.noise_nodes.get(idx) }
}
impl Storage<SNPointNodes> for NodeSet {
    fn insert(&mut self, t: SNPointNodes) -> Index { self.snpoint_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&SNPointNodes> { self.snpoint_nodes.get(idx) }
}
impl Storage<PointSetNodes> for NodeSet {
    fn insert(&mut self, t: PointSetNodes) -> Index { self.point_set_nodes.insert(t) }
    fn get(&self, idx: Index) -> Option<&PointSetNodes> { self.point_set_nodes.get(idx) }
}
