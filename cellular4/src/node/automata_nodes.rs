use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{continuous::*, discrete::*},
    mutagen_args::*,
    node::{discrete_nodes::*, point_set_nodes::*, Node},
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(gen_arg = type GenArg<'a>, mut_arg = type MutArg<'a>)]
pub enum BinaryAutomataNodes {
    Majority {
        child: Box<BooleanNodes>,
        point_set: Box<PointSetNodes>,
    },
}

impl Node for BinaryAutomataNodes {
    type Output = Boolean;

    fn compute(&self, compute_arg: ComArg) -> Self::Output {
        use BinaryAutomataNodes::*;

        match self {
            Majority { child, point_set } => {
                let mut true_count = 0;
                let offsets = point_set
                    .compute(compute_arg)
                    .get_offsets(CONSTS.cell_array_width, CONSTS.cell_array_height);

                //this might blow up
                for point in &offsets {
                    let offset_state = UpdateState {
                        coordinate_set: compute_arg.coordinate_set.get_coord_shifted(
                            point.x(),
                            point.y(),
                            SNFloat::new(0.0),
                        ),
                        history: compute_arg.history,
                    };

                    if child.compute(&offset_state, compute_arg).into_inner() {
                        true_count += 1;
                    }
                }

                Boolean {
                    value: true_count > offsets.len() / 2,
                }
            }
        }
    }
}

impl<'a> Updatable<'a> for BinaryAutomataNodes {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: &'a mut UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}
