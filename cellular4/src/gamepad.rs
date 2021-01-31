use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use ggez::{
    input::gamepad::{self, gilrs::ev::state::AxisData},
    Context,
};
use itertools::Itertools;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
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

    pub fn clear_in_use(&mut self) {
        for gamepad in self.gamepads.iter_mut() {
            gamepad.clear_in_use();
        }
    }
}

impl Index<GamepadId> for Gamepads {
    type Output = Gamepad;

    fn index(&self, idx: GamepadId) -> &Self::Output {
        &self.gamepads[idx.0]
    }
}

impl IndexMut<GamepadId> for Gamepads {
    fn index_mut(&mut self, idx: GamepadId) -> &mut Self::Output {
        &mut self.gamepads[idx.0]
    }
}

#[derive(Debug)]
pub struct Gamepad {
    pub id: GgGamepadId,
    pub button_states: ButtonStates,
    pub axis_states: AxisStates,
}

impl Gamepad {
    pub fn new(ctx: &Context, id: GgGamepadId) -> Self {
        let gamepad = gamepad::gamepad(ctx, id);
        Self {
            id,
            button_states: ButtonStates::new(&gamepad),
            axis_states: AxisStates::new(&gamepad),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        let gamepad = gamepad::gamepad(ctx, self.id);
        // FIXME Do not reset in_use here
        self.button_states.update(&gamepad);
        self.axis_states.update(&gamepad);
    }

    pub fn clear_in_use(&mut self) {
        self.button_states.clear_in_use();
        self.axis_states.clear_in_use();
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl GamepadButton {
    const VALUES: [GamepadButton; 14] = [
        GamepadButton::South,
        GamepadButton::East,
        GamepadButton::North,
        GamepadButton::West,
        GamepadButton::LeftTrigger,
        GamepadButton::LeftTrigger2,
        GamepadButton::RightTrigger,
        GamepadButton::RightTrigger2,
        GamepadButton::LeftThumb,
        GamepadButton::RightThumb,
        GamepadButton::DPadUp,
        GamepadButton::DPadDown,
        GamepadButton::DPadLeft,
        GamepadButton::DPadRight,
    ];
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

impl<'a> Generatable<'a> for GamepadButton {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: Self::GenArg) -> Self {
        let mut weights = [0.0; GamepadButton::VALUES.len()];

        for (w, btn) in weights.iter_mut().zip_eq(GamepadButton::VALUES.iter()) {
            *w = if arg
                .gamepads
                .gamepads
                .iter()
                .any(|g| !g.button_states.get(*btn).in_use)
            {
                100.0
            } else {
                1.0
            };
        }

        let idx = WeightedIndex::new(&weights).unwrap().sample(rng);

        GamepadButton::VALUES[idx]
    }
}

impl<'a> Mutatable<'a> for GamepadButton {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        *self = Self::generate_rng(rng, arg.into());
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
pub struct ButtonStates {
    pub south: ButtonState,
    pub east: ButtonState,
    pub north: ButtonState,
    pub west: ButtonState,
    pub left_trigger: ButtonState,
    pub left_trigger_2: ButtonState,
    pub right_trigger: ButtonState,
    pub right_trigger_2: ButtonState,
    pub left_thumb: ButtonState,
    pub right_thumb: ButtonState,
    pub d_pad_up: ButtonState,
    pub d_pad_down: ButtonState,
    pub d_pad_left: ButtonState,
    pub d_pad_right: ButtonState,
}

impl ButtonStates {
    pub fn new(gamepad: &GgGamepad) -> Self {
        Self {
            south: ButtonState::new(gamepad, GamepadButton::South),
            east: ButtonState::new(gamepad, GamepadButton::East),
            north: ButtonState::new(gamepad, GamepadButton::North),
            west: ButtonState::new(gamepad, GamepadButton::West),
            left_trigger: ButtonState::new(gamepad, GamepadButton::LeftTrigger),
            left_trigger_2: ButtonState::new(gamepad, GamepadButton::LeftTrigger2),
            right_trigger: ButtonState::new(gamepad, GamepadButton::RightTrigger),
            right_trigger_2: ButtonState::new(gamepad, GamepadButton::RightTrigger2),
            left_thumb: ButtonState::new(gamepad, GamepadButton::LeftThumb),
            right_thumb: ButtonState::new(gamepad, GamepadButton::RightThumb),
            d_pad_up: ButtonState::new(gamepad, GamepadButton::DPadUp),
            d_pad_down: ButtonState::new(gamepad, GamepadButton::DPadDown),
            d_pad_left: ButtonState::new(gamepad, GamepadButton::DPadLeft),
            d_pad_right: ButtonState::new(gamepad, GamepadButton::DPadRight),
        }
    }

    pub fn get(&self, btn: GamepadButton) -> &ButtonState {
        match btn {
            GamepadButton::South => &self.south,
            GamepadButton::East => &self.east,
            GamepadButton::North => &self.north,
            GamepadButton::West => &self.west,
            GamepadButton::LeftTrigger => &self.left_trigger,
            GamepadButton::LeftTrigger2 => &self.left_trigger_2,
            GamepadButton::RightTrigger => &self.right_trigger,
            GamepadButton::RightTrigger2 => &self.right_trigger_2,
            GamepadButton::LeftThumb => &self.left_thumb,
            GamepadButton::RightThumb => &self.right_thumb,
            GamepadButton::DPadUp => &self.d_pad_up,
            GamepadButton::DPadDown => &self.d_pad_down,
            GamepadButton::DPadLeft => &self.d_pad_left,
            GamepadButton::DPadRight => &self.d_pad_right,
        }
    }

    pub fn get_mut(&mut self, btn: GamepadButton) -> &mut ButtonState {
        match btn {
            GamepadButton::South => &mut self.south,
            GamepadButton::East => &mut self.east,
            GamepadButton::North => &mut self.north,
            GamepadButton::West => &mut self.west,
            GamepadButton::LeftTrigger => &mut self.left_trigger,
            GamepadButton::LeftTrigger2 => &mut self.left_trigger_2,
            GamepadButton::RightTrigger => &mut self.right_trigger,
            GamepadButton::RightTrigger2 => &mut self.right_trigger_2,
            GamepadButton::LeftThumb => &mut self.left_thumb,
            GamepadButton::RightThumb => &mut self.right_thumb,
            GamepadButton::DPadUp => &mut self.d_pad_up,
            GamepadButton::DPadDown => &mut self.d_pad_down,
            GamepadButton::DPadLeft => &mut self.d_pad_left,
            GamepadButton::DPadRight => &mut self.d_pad_right,
        }
    }

    pub fn update(&mut self, gamepad: &GgGamepad) {
        self.south.update(gamepad, GamepadButton::South);
        self.east.update(gamepad, GamepadButton::East);
        self.north.update(gamepad, GamepadButton::North);
        self.west.update(gamepad, GamepadButton::West);
        self.left_trigger
            .update(gamepad, GamepadButton::LeftTrigger);
        self.left_trigger_2
            .update(gamepad, GamepadButton::LeftTrigger2);
        self.right_trigger
            .update(gamepad, GamepadButton::RightTrigger);
        self.right_trigger_2
            .update(gamepad, GamepadButton::RightTrigger2);
        self.left_thumb.update(gamepad, GamepadButton::LeftThumb);
        self.right_thumb.update(gamepad, GamepadButton::RightThumb);
        self.d_pad_up.update(gamepad, GamepadButton::DPadUp);
        self.d_pad_down.update(gamepad, GamepadButton::DPadDown);
        self.d_pad_left.update(gamepad, GamepadButton::DPadLeft);
        self.d_pad_right.update(gamepad, GamepadButton::DPadRight);
    }

    pub fn clear_in_use(&mut self) {
        self.south.in_use = false;
        self.east.in_use = false;
        self.north.in_use = false;
        self.west.in_use = false;
        self.left_trigger.in_use = false;
        self.left_trigger_2.in_use = false;
        self.right_trigger.in_use = false;
        self.right_trigger_2.in_use = false;
        self.left_thumb.in_use = false;
        self.right_thumb.in_use = false;
        self.d_pad_up.in_use = false;
        self.d_pad_down.in_use = false;
        self.d_pad_left.in_use = false;
        self.d_pad_right.in_use = false;
    }
}

#[derive(Debug)]
pub struct ButtonState {
    pub is_pressed: bool,
    pub in_use: bool,
}

impl ButtonState {
    fn new(gamepad: &GgGamepad, btn: GamepadButton) -> Self {
        Self {
            is_pressed: gamepad.is_pressed(btn.into()),
            in_use: false,
        }
    }

    fn update(&mut self, gamepad: &GgGamepad, btn: GamepadButton) {
        self.is_pressed = gamepad.is_pressed(btn.into());
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

impl GamepadAxis {
    const VALUES: [GamepadAxis; 6] = [
        GamepadAxis::LeftStickX,
        GamepadAxis::LeftStickY,
        GamepadAxis::RightStickX,
        GamepadAxis::RightStickY,
        GamepadAxis::DPadX,
        GamepadAxis::DPadY,
        // TODO These are unmapped on Linux with an Xbox controller
        // Seems to be a ggez bug?
        //GamepadAxis::LeftZ,
        //GamepadAxis::RightZ,
    ];
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

impl<'a> Generatable<'a> for GamepadAxis {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: Self::GenArg) -> Self {
        let mut weights = [0.0; GamepadAxis::VALUES.len()];

        for (w, axis) in weights.iter_mut().zip_eq(GamepadAxis::VALUES.iter()) {
            *w = if arg
                .gamepads
                .gamepads
                .iter()
                .any(|g| !g.axis_states.get(*axis).in_use)
            {
                100.0
            } else {
                1.0
            };
        }

        let idx = WeightedIndex::new(&weights).unwrap().sample(rng);

        GamepadAxis::VALUES[idx]
    }
}

impl<'a> Mutatable<'a> for GamepadAxis {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        *self = Self::generate_rng(rng, arg.into());
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
pub struct AxisStates {
    pub left_stick_x: AxisState,
    pub left_stick_y: AxisState,
    pub right_stick_x: AxisState,
    pub right_stick_y: AxisState,
    pub d_pad_x: AxisState,
    pub d_pad_y: AxisState,
}

impl AxisStates {
    pub fn new(gamepad: &GgGamepad) -> Self {
        Self {
            left_stick_x: AxisState::new(gamepad, GamepadAxis::LeftStickX),
            left_stick_y: AxisState::new(gamepad, GamepadAxis::LeftStickY),
            right_stick_x: AxisState::new(gamepad, GamepadAxis::RightStickX),
            right_stick_y: AxisState::new(gamepad, GamepadAxis::RightStickY),
            d_pad_x: AxisState::new(gamepad, GamepadAxis::DPadX),
            d_pad_y: AxisState::new(gamepad, GamepadAxis::DPadY),
        }
    }

    pub fn get(&self, axis: GamepadAxis) -> &AxisState {
        match axis {
            GamepadAxis::LeftStickX => &self.left_stick_x,
            GamepadAxis::LeftStickY => &self.left_stick_y,
            GamepadAxis::RightStickX => &self.right_stick_x,
            GamepadAxis::RightStickY => &self.right_stick_y,
            GamepadAxis::DPadX => &self.d_pad_x,
            GamepadAxis::DPadY => &self.d_pad_y,
        }
    }

    pub fn get_mut(&mut self, axis: GamepadAxis) -> &mut AxisState {
        match axis {
            GamepadAxis::LeftStickX => &mut self.left_stick_x,
            GamepadAxis::LeftStickY => &mut self.left_stick_y,
            GamepadAxis::RightStickX => &mut self.right_stick_x,
            GamepadAxis::RightStickY => &mut self.right_stick_y,
            GamepadAxis::DPadX => &mut self.d_pad_x,
            GamepadAxis::DPadY => &mut self.d_pad_y,
        }
    }

    fn update(&mut self, gamepad: &GgGamepad) {
        self.left_stick_x.update(gamepad, GamepadAxis::LeftStickX);
        self.left_stick_y.update(gamepad, GamepadAxis::LeftStickY);
        self.right_stick_x.update(gamepad, GamepadAxis::RightStickX);
        self.right_stick_y.update(gamepad, GamepadAxis::RightStickY);
        self.d_pad_x.update(gamepad, GamepadAxis::DPadX);
        self.d_pad_y.update(gamepad, GamepadAxis::DPadY);
    }

    fn clear_in_use(&mut self) {
        self.left_stick_x.in_use = false;
        self.left_stick_y.in_use = false;
        self.right_stick_x.in_use = false;
        self.right_stick_y.in_use = false;
        self.d_pad_x.in_use = false;
        self.d_pad_y.in_use = false;
    }
}

#[derive(Debug)]
pub struct AxisState {
    pub value: f32,
    pub in_use: bool,
}

impl AxisState {
    pub fn new(gamepad: &GgGamepad, axis: GamepadAxis) -> Self {
        Self {
            value: gamepad
                .axis_data(axis.into())
                .map(AxisData::value)
                .unwrap_or(0.0),
            in_use: false,
        }
    }

    fn update(&mut self, gamepad: &GgGamepad, axis: GamepadAxis) {
        self.value = gamepad
            .axis_data(axis.into())
            .map(AxisData::value)
            .unwrap_or(0.0);
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

    const VALUES: [GamepadAxes2D; 3] = [
        GamepadAxes2D::LeftStick,
        GamepadAxes2D::RightStick,
        GamepadAxes2D::DPad,
    ];
}

impl<'a> Generatable<'a> for GamepadAxes2D {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, arg: Self::GenArg) -> Self {
        let mut weights = [0.0; GamepadAxes2D::VALUES.len()];

        for (w, axes) in weights.iter_mut().zip_eq(GamepadAxes2D::VALUES.iter()) {
            let (x, y) = axes.axes();

            *w = if arg
                .gamepads
                .gamepads
                .iter()
                .any(|g| !g.axis_states.get(x).in_use || !g.axis_states.get(y).in_use)
            {
                100.0
            } else {
                1.0
            };
        }

        let idx = WeightedIndex::new(&weights).unwrap().sample(rng);

        GamepadAxes2D::VALUES[idx]
    }
}

impl<'a> Mutatable<'a> for GamepadAxes2D {
    type MutArg = MutArg<'a>;

    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, arg: Self::MutArg) {
        *self = Self::generate_rng(rng, arg.into());
    }
}

impl<'a> Updatable<'a> for GamepadAxes2D {
    type UpdateArg = ();
    fn update(&mut self, _arg: Self::UpdateArg) {}
}

impl<'a> UpdatableRecursively<'a> for GamepadAxes2D {
    fn update_recursively(&mut self, _arg: Self::UpdateArg) {}
}
