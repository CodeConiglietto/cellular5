use std::{collections::HashSet, ops::Index};

use ggez::{
    input::gamepad::{
        self,
        gilrs::ev::{state::GamepadState, Code},
    },
    Context,
};
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub use ggez::{
    event::{Axis as GgAxis, Button as GgButton},
    input::gamepad::{Gamepad as GgGamepad, GamepadId as GgGamepadId},
};

use crate::prelude::*;

#[derive(Debug)]
pub struct Gamepads {
    pub gamepads: Vec<Gamepad>,
    pub ids: HashSet<GgGamepadId>,
}

impl Gamepads {
    pub fn new() -> Self {
        Self {
            gamepads: Vec::new(),
            ids: HashSet::new(),
        }
    }

    pub fn register_gamepad(&mut self, ctx: &Context, id: GgGamepadId) {
        if !self.ids.contains(&id) {
            self.ids.insert(id);
            self.gamepads.push(Gamepad::new(ctx, id));
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        for gamepad in self.gamepads.iter_mut() {
            gamepad.update(ctx);
        }
    }
}

impl Index<GamepadId> for Gamepads {
    type Output = Gamepad;

    fn index(&self, idx: GamepadId) -> &Self::Output {
        &self.gamepads[idx.0]
    }
}

#[derive(Debug)]
pub struct Gamepad {
    id: GgGamepadId,
    state: GamepadState,
    button_map: ButtonMap,
    axis_map: AxisMap,
}

impl Gamepad {
    pub fn new(ctx: &Context, id: GgGamepadId) -> Self {
        let gamepad = gamepad::gamepad(ctx, id);
        Self {
            id,
            state: gamepad.state().clone(),
            button_map: ButtonMap::new(&gamepad),
            axis_map: AxisMap::new(&gamepad),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        self.state = gamepad::gamepad(ctx, self.id).state().clone();
    }

    pub fn button(&self, btn: GamepadButton) -> bool {
        if let Some(data) = self
            .button_map
            .map(btn)
            .and_then(|code| self.state.button_data(code))
        {
            data.is_pressed()
        } else {
            false
        }
    }

    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        if let Some(data) = self
            .axis_map
            .map(axis)
            .and_then(|code| self.state.axis_data(code))
        {
            data.value()
        } else {
            0.0
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct GamepadId(pub usize);

impl<'a> Generatable<'a> for GamepadId {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: GenArg<'a>) -> Self {
        GamepadId(rng.gen_range(0, arg.gamepads.gamepads.len()))
    }
}

impl<'a> Mutatable<'a> for GamepadId {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        *self = Self::generate_rng(rng, arg.into());
    }
}

impl<'a> Updatable<'a> for GamepadId {
    type UpdateArg = ();
    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for GamepadId {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}

#[derive(Clone, Copy, Debug, Generatable, Mutatable, Serialize, Deserialize)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum GamepadButton {
    South,
    East,
    North,
    West,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

impl From<GamepadButton> for GgButton {
    fn from(btn: GamepadButton) -> GgButton {
        match btn {
            GamepadButton::South => GgButton::South,
            GamepadButton::East => GgButton::East,
            GamepadButton::North => GgButton::North,
            GamepadButton::West => GgButton::West,
            GamepadButton::LeftTrigger => GgButton::LeftTrigger,
            GamepadButton::LeftTrigger2 => GgButton::LeftTrigger2,
            GamepadButton::RightTrigger => GgButton::RightTrigger,
            GamepadButton::RightTrigger2 => GgButton::RightTrigger2,
            GamepadButton::LeftThumb => GgButton::LeftThumb,
            GamepadButton::RightThumb => GgButton::RightThumb,
            GamepadButton::DPadUp => GgButton::DPadUp,
            GamepadButton::DPadDown => GgButton::DPadDown,
            GamepadButton::DPadLeft => GgButton::DPadLeft,
            GamepadButton::DPadRight => GgButton::DPadRight,
        }
    }
}

impl<'a> Updatable<'a> for GamepadButton {
    type UpdateArg = ();
    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for GamepadButton {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}

#[derive(Debug)]
struct ButtonMap {
    south: Option<Code>,
    east: Option<Code>,
    north: Option<Code>,
    west: Option<Code>,
    left_trigger: Option<Code>,
    left_trigger_2: Option<Code>,
    right_trigger: Option<Code>,
    right_trigger_2: Option<Code>,
    left_thumb: Option<Code>,
    right_thumb: Option<Code>,
    d_pad_up: Option<Code>,
    d_pad_down: Option<Code>,
    d_pad_left: Option<Code>,
    d_pad_right: Option<Code>,
}

impl ButtonMap {
    fn new(gamepad: &GgGamepad) -> Self {
        Self {
            south: gamepad.button_code(GamepadButton::South.into()),
            east: gamepad.button_code(GamepadButton::East.into()),
            north: gamepad.button_code(GamepadButton::North.into()),
            west: gamepad.button_code(GamepadButton::West.into()),
            left_trigger: gamepad.button_code(GamepadButton::LeftTrigger.into()),
            left_trigger_2: gamepad.button_code(GamepadButton::LeftTrigger2.into()),
            right_trigger: gamepad.button_code(GamepadButton::RightTrigger.into()),
            right_trigger_2: gamepad.button_code(GamepadButton::RightTrigger2.into()),
            left_thumb: gamepad.button_code(GamepadButton::LeftThumb.into()),
            right_thumb: gamepad.button_code(GamepadButton::RightThumb.into()),
            d_pad_up: gamepad.button_code(GamepadButton::DPadUp.into()),
            d_pad_down: gamepad.button_code(GamepadButton::DPadDown.into()),
            d_pad_left: gamepad.button_code(GamepadButton::DPadLeft.into()),
            d_pad_right: gamepad.button_code(GamepadButton::DPadRight.into()),
        }
    }

    fn map(&self, btn: GamepadButton) -> Option<Code> {
        match btn {
            GamepadButton::South => self.south,
            GamepadButton::East => self.east,
            GamepadButton::North => self.north,
            GamepadButton::West => self.west,
            GamepadButton::LeftTrigger => self.left_trigger,
            GamepadButton::LeftTrigger2 => self.left_trigger_2,
            GamepadButton::RightTrigger => self.right_trigger,
            GamepadButton::RightTrigger2 => self.right_trigger_2,
            GamepadButton::LeftThumb => self.left_thumb,
            GamepadButton::RightThumb => self.right_thumb,
            GamepadButton::DPadUp => self.d_pad_up,
            GamepadButton::DPadDown => self.d_pad_down,
            GamepadButton::DPadLeft => self.d_pad_left,
            GamepadButton::DPadRight => self.d_pad_right,
        }
    }
}

#[derive(Clone, Copy, Debug, Generatable, Mutatable, Serialize, Deserialize)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    DPadX,
    DPadY,
    // TODO These are unmapped on Linux with an Xbox controller
    // Seems to be a ggez bug?
    //LeftZ,
    //RightZ,
}

impl From<GamepadAxis> for GgAxis {
    fn from(axis: GamepadAxis) -> GgAxis {
        match axis {
            GamepadAxis::LeftStickX => GgAxis::LeftStickX,
            GamepadAxis::LeftStickY => GgAxis::LeftStickY,
            GamepadAxis::RightStickX => GgAxis::RightStickX,
            GamepadAxis::RightStickY => GgAxis::RightStickY,
            GamepadAxis::DPadX => GgAxis::DPadX,
            GamepadAxis::DPadY => GgAxis::DPadY,
        }
    }
}

impl<'a> Updatable<'a> for GamepadAxis {
    type UpdateArg = ();
    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for GamepadAxis {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}

#[derive(Debug)]
pub struct AxisMap {
    left_stick_x: Option<Code>,
    left_stick_y: Option<Code>,
    right_stick_x: Option<Code>,
    right_stick_y: Option<Code>,
    d_pad_x: Option<Code>,
    d_pad_y: Option<Code>,
}

impl AxisMap {
    fn new(gamepad: &GgGamepad) -> Self {
        Self {
            left_stick_x: gamepad.axis_code(GamepadAxis::LeftStickX.into()),
            left_stick_y: gamepad.axis_code(GamepadAxis::LeftStickY.into()),
            right_stick_x: gamepad.axis_code(GamepadAxis::RightStickX.into()),
            right_stick_y: gamepad.axis_code(GamepadAxis::RightStickY.into()),
            d_pad_x: gamepad.axis_code(GamepadAxis::DPadX.into()),
            d_pad_y: gamepad.axis_code(GamepadAxis::DPadY.into()),
        }
    }

    fn map(&self, axis: GamepadAxis) -> Option<Code> {
        match axis {
            GamepadAxis::LeftStickX => self.left_stick_x,
            GamepadAxis::LeftStickY => self.left_stick_y,
            GamepadAxis::RightStickX => self.right_stick_x,
            GamepadAxis::RightStickY => self.right_stick_y,
            GamepadAxis::DPadX => self.d_pad_x,
            GamepadAxis::DPadY => self.d_pad_y,
        }
    }
}

#[derive(Clone, Copy, Debug, Generatable, Mutatable, Serialize, Deserialize)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum GamepadAxes2D {
    LeftStick,
    RightStick,
    DPad,
}

impl GamepadAxes2D {
    pub fn axes(self) -> (GamepadAxis, GamepadAxis) {
        match self {
            GamepadAxes2D::LeftStick => (GamepadAxis::LeftStickX, GamepadAxis::LeftStickY),
            GamepadAxes2D::RightStick => (GamepadAxis::RightStickX, GamepadAxis::RightStickY),
            GamepadAxes2D::DPad => (GamepadAxis::DPadX, GamepadAxis::DPadY),
        }
    }
}

impl<'a> Updatable<'a> for GamepadAxes2D {
    type UpdateArg = ();
    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for GamepadAxes2D {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}
