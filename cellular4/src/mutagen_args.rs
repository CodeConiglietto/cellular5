use mutagen::Reborrow;

use crate::prelude::*;

pub trait MutagenArg {
    fn depth(&self) -> usize;
}

#[derive(Debug)]
pub struct GenArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
    pub current_t: usize,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub image_preloader: &'a Preloader<Image>,
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, GenArg<'a>> for GenArg<'b> {
    fn reborrow(&'a mut self) -> GenArg<'a> {
        GenArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
            current_t: self.current_t,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
        }
    }
}

impl<'a> mutagen::State for GenArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for GenArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }
}

#[derive(Debug)]
pub struct MutArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub depth: usize,
    pub current_t: usize,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub image_preloader: &'a Preloader<Image>,
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, MutArg<'a>> for MutArg<'b> {
    fn reborrow(&'a mut self) -> MutArg<'a> {
        MutArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            depth: self.depth,
            current_t: self.current_t,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
        }
    }
}

impl<'a> From<MutArg<'a>> for GenArg<'a> {
    fn from(arg: MutArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            depth: arg.depth,
            current_t: arg.current_t,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            image_preloader: arg.image_preloader,
            profiler: arg.profiler,
        }
    }
}

impl<'a> mutagen::State for MutArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for MutArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }
}

#[derive(Clone, Debug)]
pub struct ComArg<'a> {
    pub nodes: &'a [NodeSet],
    pub data: &'a DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
    pub current_t: usize,
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
            coordinate_set: self.coordinate_set,
            history: &self.history,
            depth: self.depth,
            current_t: self.current_t,
        }
    }
}

impl<'a> mutagen::State for ComArg<'a> {}

impl<'a> MutagenArg for ComArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }
}

#[derive(Debug)]
pub struct UpdArg<'a> {
    pub nodes: &'a mut [NodeSet],
    pub data: &'a mut DataSet,
    pub coordinate_set: CoordinateSet,
    pub history: &'a History,
    pub depth: usize,
    pub current_t: usize,
    pub image_preloader: &'a Preloader<Image>,
    pub profiler: &'a mut Option<MutagenProfiler>,
}

impl<'a, 'b: 'a> Reborrow<'a, 'b, UpdArg<'a>> for UpdArg<'b> {
    fn reborrow(&'a mut self) -> UpdArg<'a> {
        UpdArg {
            nodes: &mut self.nodes,
            data: &mut self.data,
            coordinate_set: self.coordinate_set,
            history: &self.history,
            depth: self.depth,
            current_t: self.current_t,
            image_preloader: &self.image_preloader,
            profiler: &mut self.profiler,
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
            current_t: arg.current_t,
        }
    }
}

impl<'a> From<GenArg<'a>> for UpdArg<'a> {
    fn from(arg: GenArg<'a>) -> Self {
        Self {
            nodes: arg.nodes,
            data: arg.data,
            depth: arg.depth,
            current_t: arg.current_t,
            coordinate_set: arg.coordinate_set,
            history: arg.history,
            image_preloader: arg.image_preloader,
            profiler: arg.profiler,
        }
    }
}

impl<'a> mutagen::State for UpdArg<'a> {
    fn handle_event(&mut self, event: mutagen::Event) {
        if let Some(profiler) = &mut self.profiler {
            profiler.handle_event(event);
        }
    }
}

impl<'a> MutagenArg for UpdArg<'a> {
    fn depth(&self) -> usize {
        self.depth.saturating_sub(1) // Subtract 1 since NodeBox adds 1 earlier than the mutagen code will see it
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UpdateState<'a> {
    //the set of coordinates for the update
    pub coordinate_set: CoordinateSet,
    //cell array to read from
    pub history: &'a History,
}

impl<'a> From<GenArg<'a>> for () {
    fn from(_arg: GenArg<'a>) -> Self {}
}

impl<'a> From<MutArg<'a>> for () {
    fn from(_arg: MutArg<'a>) -> Self {}
}
