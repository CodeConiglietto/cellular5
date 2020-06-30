use crate::{
    constants::*,
    data_set::*, 
    node_set::*, 
    coordinate_set::*,
    datatype::{continuous::*, discrete::*, points::*},
    history::*,
};

#[derive(Clone, Debug)]
pub struct GenArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
}

#[derive(Clone, Debug)]
pub struct MutArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a mut DataSet,
}

#[derive(Clone, Debug)]
pub struct ComArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
}

#[derive(Clone, Debug)]
pub struct UpdArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a mut DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
}

impl<'a> UpdArg<'a> {
    pub fn to_com_arg(&'a self) -> ComArg<'a> {
        ComArg {
            nodes: &self.nodes,
            data: &self.data,
            coords: self.coords,
            history: self.history,
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