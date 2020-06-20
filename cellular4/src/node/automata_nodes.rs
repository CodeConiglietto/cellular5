use mutagen::{Generatable, Mutagen, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{
    constants::*,
    datatype::{continuous::*, discrete::*, point_sets::*},
    node::{discrete_nodes::*, point_set_nodes::*, Node},
    updatestate::UpdateState,
};

#[derive(Generatable, UpdatableRecursively, Mutatable, Deserialize, Serialize, Debug)]
#[mutagen(mut_reroll = 0.1)]
pub enum BinaryAutomataNodes {
    Majority {
        child: Box<BooleanNodes>,
        point_set: Box<PointSetNodes>,
    },
}

impl<'a> Mutagen<'a> for BinaryAutomataNodes {
    type Arg = UpdateState<'a>;
}
impl Node for BinaryAutomataNodes {
    type Output = Boolean;

    fn compute(&self, state: UpdateState) -> Self::Output {
        use BinaryAutomataNodes::*;

        match self {
            Majority { child, point_set } => {
                let mut true_count = 0;
                let offsets = point_set
                    .compute(state)
                    .get_offsets(CONSTS.cell_array_width, CONSTS.cell_array_height);

                //this might blow up
                for point in &offsets {
                    let offset_state = UpdateState {
                        coordinate_set: state.coordinate_set.get_coord_shifted(
                            point.x(),
                            point.y(),
                            SNFloat::new(0.0),
                        ),
                        history: state.history,
                    };

                    if child.compute(offset_state).into_inner() {
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
    fn update(&mut self, _state: mutagen::State, _arg: UpdateState<'a>) {
        match self {
            _ => {}
        }
    }
}
