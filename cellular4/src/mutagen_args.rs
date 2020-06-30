use mutagen::Reborrow;

use crate::{
    constants::*,
    coordinate_set::*,
    data_set::*,
    datatype::{continuous::*, discrete::*, points::*},
    history::*,
    node_set::*,
};

#[derive(Debug)]
pub struct GenArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, GenArg<'a>> for GenArg<'b> {
    fn reborrow(borrow: &'a mut Self) -> GenArg<'a> {
        GenArg {
            nodes: &mut borrow.nodes,
            data: &mut borrow.data,
        }
    }
}

#[derive(Debug)]
pub struct MutArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a mut DataSet,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, MutArg<'a>> for MutArg<'b> {
    fn reborrow(borrow: &'a mut Self) -> MutArg<'a> {
        MutArg {
            nodes: &borrow.nodes,
            data: &mut borrow.data,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, ComArg<'a>> for ComArg<'b> {
    fn reborrow(borrow: &'a mut Self) -> ComArg<'a> {
        ComArg {
            nodes: &borrow.nodes,
            data: &borrow.data,
            coordinate_set: borrow.coordinate_set.clone(),
            history: &borrow.history,
        }
    }
}

#[derive(Debug)]
pub struct UpdArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a mut DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, UpdArg<'a>> for UpdArg<'b> {
    fn reborrow(borrow: &'a mut Self) -> UpdArg<'a> {
        UpdArg {
            nodes: &borrow.nodes,
            data: &mut borrow.data,
            coordinate_set: borrow.coordinate_set.clone(),
            history: &borrow.history,
        }
    }
}

impl<'a> From<UpdArg<'a>> for ComArg<'a> {
    fn from(arg: UpdArg<'a>) -> Self {
        Self {
            nodes: &arg.nodes,
            data: &arg.data,
            coordinate_set: arg.coordinate_set.clone(),
            history: arg.history,
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

impl UpdateState<'_> {
    pub fn replace_coords(self, other: &SNPoint) -> Self {
        let mut new = self.clone();

        new.coordinate_set.x = other.x();
        new.coordinate_set.y = other.y();

        new
    }
}
