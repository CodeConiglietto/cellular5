use mutagen::Reborrow;

use crate::{coordinate_set::*, data_set::*, datatype::points::*, history::*, node_set::*};

#[derive(Debug)]
pub struct GenArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, GenArg<'a>> for GenArg<'b> {
    fn reborrow(&'a mut self) -> GenArg<'a> {
        GenArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
        }
    }
}

#[derive(Debug)]
pub struct MutArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, MutArg<'a>> for MutArg<'b> {
    fn reborrow(&'a mut self) -> MutArg<'a> {
        MutArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
        }
    }
}

impl<'a> From<MutArg<'a>> for GenArg<'a> {
    fn from(arg: MutArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            depth: arg.depth,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
}

impl<'a> ComArg<'a> {
    pub fn replace_coords(self, other: &SNPoint) -> Self {
        let mut new = self.clone();

        new.coordinate_set.x = other.x();
        new.coordinate_set.y = other.y();

        new
    }
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ComArg<'a>> for ComArg<'b> {
    fn reborrow(&'a mut self) -> ComArg<'a> {
        ComArg {
            nodes: &self.nodes,
            data: &self.data,
            coordinate_set: self.coordinate_set.clone(),
            history: &self.history,
            depth: self.depth,
        }
    }
}

#[derive(Debug)]
pub struct UpdArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a mut DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, UpdArg<'a>> for UpdArg<'b> {
    fn reborrow(&'a mut self) -> UpdArg<'a> {
        UpdArg {
            nodes: &self.nodes,
            data: &mut self.data,
            coordinate_set: self.coordinate_set.clone(),
            history: &self.history,
            depth: self.depth,
        }
    }
}

impl<'a> From<UpdArg<'a>> for ComArg<'a> {
    fn from(arg: UpdArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            depth: arg.depth,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //the set of coordinates for the update
    pub coordinate_set: CoordinateSet,
    //cell array to read from
    pub history: &'a History,
}
