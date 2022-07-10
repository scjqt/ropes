mod rendering;
mod ropes;

use crate::input::{self, Inputs};
use glam::IVec2;
use ropes::Ropes;

const TPS: u64 = 32;
const RADIUS: f64 = 12.;

const TICK_DURATION: f64 = 1. / TPS as f64;

pub struct State {
    accumulator: f64,
    saved: Ropes,
    active: Ropes,
    camera: IVec2,
    simulating: bool,
    action: Action,
}

impl State {
    pub fn new() -> Self {
        Self {
            accumulator: 0.,
            saved: Ropes::new(),
            active: Ropes::new(),
            camera: IVec2::ZERO,
            simulating: false,
            action: Action::None,
        }
    }

    pub fn update(&mut self, dt: f64, inputs: &Inputs) {
        use input::Input::*;

        if self.simulating {
            match self.action {
                Action::Panning => {
                    if inputs[Alternate] {
                        if inputs[RightMouse] {
                            self.camera += inputs.last_mouse() - inputs.mouse_position();
                        }
                    } else {
                        self.action = Action::None;
                    }
                }
                Action::None => {
                    if inputs[RightMouse] && inputs[Alternate] {
                        self.action = Action::Panning;
                    }
                }
                Action::Deleting(_) => {
                    if !inputs[RightMouse] {
                        self.action = Action::None;
                    }
                }
                _ => (),
            }

            let mouse = inputs.mouse_position() + self.camera;

            self.accumulator += dt;
            while self.accumulator >= TICK_DURATION {
                if let Action::Deleting(last) = self.action {
                    self.active.remove_sticks(last.as_dvec2(), mouse.as_dvec2());
                    self.active
                        .remove_points(last.as_dvec2(), mouse.as_dvec2(), RADIUS);
                }
                if let Action::Panning = self.action {
                } else {
                    if inputs[RightMouse] {
                        self.action = Action::Deleting(inputs.mouse_position() + self.camera);
                    } else {
                        self.action = Action::None;
                    }
                }

                self.active.tick();
                self.accumulator -= TICK_DURATION;
            }
        } else {
            if inputs[Clear] {
                self.saved = Ropes::new();
                self.action.reset();
                self.camera = IVec2::ZERO;
            }

            if let Action::Panning = self.action {
                if inputs[Alternate] {
                    if inputs[RightMouse] {
                        self.camera += inputs.last_mouse() - inputs.mouse_position();
                    }
                } else {
                    self.action = Action::None;
                }
            }

            let mouse = inputs.mouse_position() + self.camera;

            match &mut self.action {
                Action::CreatingPoint => {
                    if !inputs[LeftMouse] {
                        if self
                            .saved
                            .get_point(mouse.as_dvec2(), RADIUS * 2.)
                            .is_none()
                        {
                            self.saved.add_point(mouse.as_dvec2());
                        }
                        self.action = Action::None;
                    }
                }
                Action::CreatingStick(key, end) => {
                    if let Some(key2) = self.saved.get_point(mouse.as_dvec2(), RADIUS) {
                        *end = StickEnd::Key(key2);
                    } else {
                        *end = StickEnd::Mouse(inputs.mouse_position());
                    }
                    if !inputs[LeftMouse] {
                        if let StickEnd::Key(key2) = *end {
                            if *key == key2 {
                                self.saved.toggle_locked(*key);
                            } else {
                                self.saved.add_stick(*key, key2);
                            }
                        }
                        self.action = Action::None;
                    }
                }
                Action::Deleting(last) => {
                    if inputs[RightMouse] {
                        self.saved
                            .remove_sticks((*last).as_dvec2(), mouse.as_dvec2());
                        self.saved
                            .remove_points((*last).as_dvec2(), mouse.as_dvec2(), RADIUS);
                        *last = mouse;
                    } else {
                        self.action = Action::None;
                    }
                }
                Action::CreatingLine(selected) => {
                    if inputs[Alternate] {
                        if inputs[RightMouse] {
                            self.action = Action::Panning;
                        } else {
                            if inputs[LeftMouse] {
                                if self
                                    .saved
                                    .get_point(mouse.as_dvec2(), RADIUS * 2.)
                                    .is_none()
                                {
                                    self.saved.add_point(mouse.as_dvec2());
                                }
                            }
                            if let Some(key2) = self.saved.get_point(mouse.as_dvec2(), RADIUS) {
                                if let Some((key, _)) = selected {
                                    if *key != key2 {
                                        self.saved.add_stick(*key, key2);
                                        *key = key2;
                                    }
                                } else {
                                    *selected = Some((key2, inputs.mouse_position()));
                                }
                            }
                            if let Some((_, mouse)) = selected {
                                *mouse = inputs.mouse_position();
                            }
                        }
                    } else {
                        self.action = Action::None;
                    }
                }
                Action::None => {
                    if inputs[Alternate] {
                        self.action = Action::CreatingLine(None);
                    } else if inputs[LeftMouse] {
                        if inputs[Alternate] {
                            self.action = Action::Panning;
                        } else {
                            if let Some(key) = self.saved.get_point(mouse.as_dvec2(), RADIUS) {
                                self.action = Action::CreatingStick(key, StickEnd::Key(key));
                            } else {
                                self.action = Action::CreatingPoint;
                            }
                        }
                    } else if inputs[RightMouse] {
                        self.action = Action::Deleting(mouse);
                    }
                }
                _ => (),
            }
        }

        if inputs[ToggleSimulating] && !inputs.last(ToggleSimulating) {
            self.simulating = !self.simulating;
            if self.simulating {
                self.accumulator = 0.;
                self.active = self.saved.clone();
                self.active.tick();
            }
            self.action.reset();
        }
    }
}

enum Action {
    CreatingPoint,
    CreatingStick(usize, StickEnd),
    CreatingLine(Option<(usize, IVec2)>),
    Deleting(IVec2),
    Panning,
    None,
}

impl Action {
    fn reset(&mut self) {
        if let Action::Panning = *self {
        } else {
            *self = Action::None;
        }
    }
}

enum StickEnd {
    Key(usize),
    Mouse(IVec2),
}
