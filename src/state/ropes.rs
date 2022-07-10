use super::TICK_DURATION;
use glam::DVec2;
use rand::seq::IteratorRandom;
use slab::Slab;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

const GRAVITY: f64 = 1000.;
const REPETITIONS: u8 = 8;

const TICK_GRAVITY: f64 = GRAVITY * TICK_DURATION * TICK_DURATION;

#[derive(Clone)]
pub struct Ropes {
    points: Slab<Point>,
    sticks: HashSet<Stick>,
}

impl Ropes {
    pub fn new() -> Self {
        Self {
            points: Slab::new(),
            sticks: HashSet::new(),
        }
    }

    pub fn tick(&mut self) {
        for (_, point) in self.points.iter_mut() {
            if !point.locked {
                let last = point.position;
                point.position += point.position - point.last_position;
                point.last_position = last;
                point.position.y += TICK_GRAVITY;
            }
        }

        for _ in 0..REPETITIONS {
            let mut rng = rand::thread_rng();
            let len = self.sticks.len();
            let sticks = self.sticks.iter();
            for Stick {
                points: (key1, key2),
                length,
            } in sticks.choose_multiple(&mut rng, len)
            {
                let (point1, point2) = (&self.points[*key1], &self.points[*key2]);
                let centre = (point1.position + point2.position) / 2.;
                let offset = (point1.position - point2.position).normalize() * *length / 2.;
                let (locked1, locked2) = (point1.locked, point2.locked);
                if !locked1 {
                    self.points[*key1].position = centre + offset;
                }
                if !locked2 {
                    self.points[*key2].position = centre - offset;
                }
            }
        }
    }

    pub fn add_point(&mut self, position: DVec2) {
        self.points.insert(Point::new(position));
    }

    pub fn add_stick(&mut self, key1: usize, key2: usize) {
        if let (Some(point1), Some(point2)) = (self.points.get(key1), self.points.get(key2)) {
            self.sticks.insert(Stick::new(key1, key2, point1, point2));
        }
    }

    pub fn get_point(&self, position: DVec2, distance: f64) -> Option<usize> {
        let distance_squared = distance * distance;
        for (key, point) in self.points.iter() {
            if (position - point.position).length_squared() < distance_squared {
                return Some(key);
            }
        }
        None
    }

    pub fn toggle_locked(&mut self, key: usize) {
        if let Some(point) = self.points.get_mut(key) {
            point.locked = !point.locked;
        }
    }

    pub fn remove_points(&mut self, last: DVec2, current: DVec2, radius: f64) {
        self.points.retain(
            |key,
             Point {
                 position,
                 last_position,
                 ..
             }| {
                if intersects_point(last, current, *last_position, *position, radius) {
                    self.sticks
                        .retain(|stick| key != stick.points.0 && key != stick.points.1);
                    return false;
                }
                true
            },
        )
    }

    pub fn remove_sticks(&mut self, last: DVec2, current: DVec2) {
        self.sticks.retain(
            |Stick {
                 points: (key1, key2),
                 ..
             }| {
                let (point1, point2) = (&self.points[*key1], &self.points[*key2]);
                let (a0, a1) = (point1.last_position, point1.position);
                let (b0, b1) = (point2.last_position, point2.position);
                !intersects_stick(last, current, a0, a1, b0, b1)
            },
        );
    }

    pub fn get_points(&self, t: f64) -> impl Iterator<Item = (DVec2, bool)> + '_ {
        self.points
            .iter()
            .map(move |(_, point)| (point.interpolate(t), point.locked))
    }

    pub fn get_sticks(&self, t: f64) -> impl Iterator<Item = (DVec2, DVec2)> + '_ {
        self.sticks.iter().map(
            move |Stick {
                      points: (key1, key2),
                      ..
                  }| {
                (
                    self.points[*key1].interpolate(t),
                    self.points[*key2].interpolate(t),
                )
            },
        )
    }

    pub fn get_position(&self, key: usize, t: f64) -> DVec2 {
        self.points[key].interpolate(t)
    }
}

#[derive(Clone)]
pub struct Point {
    position: DVec2,
    last_position: DVec2,
    locked: bool,
}

impl Point {
    fn new(position: DVec2) -> Self {
        Self {
            position,
            last_position: position,
            locked: false,
        }
    }

    fn interpolate(&self, t: f64) -> DVec2 {
        self.last_position.lerp(self.position, t)
    }
}

#[derive(Clone)]
pub struct Stick {
    points: (usize, usize),
    length: f64,
}

impl Stick {
    fn new(key1: usize, key2: usize, point1: &Point, point2: &Point) -> Self {
        Self {
            points: (key1, key2),
            length: (point1.position - point2.position).length(),
        }
    }
}

impl PartialEq for Stick {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points || (self.points.1, self.points.0) == other.points
    }
}

impl Eq for Stick {}

impl Hash for Stick {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.points.0 <= self.points.1 {
            self.points.hash(state);
        } else {
            (self.points.1, self.points.0).hash(state);
        }
    }
}

fn intersects_point(m0: DVec2, m1: DVec2, c0: DVec2, c1: DVec2, radius: f64) -> bool {
    let m0 = m0 - c0;
    let m1 = m1 - c1;
    let c = m0.dot(m0);
    let r = radius * radius;
    if c < r {
        true
    } else if m1.dot(m1) < r {
        true
    } else {
        let diff = m1 - m0;
        let a = diff.dot(diff);
        if a != 0. {
            let b = m0.dot(diff);
            let t = -b / a;
            if t > 0. && t < 1. && a * c - b * b < r * a {
                return true;
            }
        }
        false
    }
}

fn intersects_stick(m0: DVec2, m1: DVec2, a0: DVec2, a1: DVec2, b0: DVec2, b1: DVec2) -> bool {
    let m0 = m0 - a0;
    let m1 = m1 - a1;
    let b0 = b0 - a0;
    let b1 = b1 - a1;
    let a = (m1 - m0).perp_dot(b1 - b0);
    let b = m0.perp_dot(b1) + m1.perp_dot(b0) + 2. * b0.perp_dot(m0);
    let c = m0.perp_dot(b0);
    if a == 0. {
        if b == 0. {
            if c == 0. && valid(m0, m1, b0, b1, 0.5) {
                return true;
            }
        } else if valid(m0, m1, b0, b1, -c / b) {
            return true;
        }
    } else {
        let det = b * b - 4. * a * c;
        if det >= 0. {
            let det = det.sqrt();
            if valid(m0, m1, b0, b1, (-b - det) / (2. * a)) {
                return true;
            }
            if valid(m0, m1, b0, b1, (-b + det) / (2. * a)) {
                return true;
            }
        }
    }
    false
}

fn valid(m0: DVec2, m1: DVec2, b0: DVec2, b1: DVec2, t: f64) -> bool {
    if t >= 0. && t < 1. {
        let m = m0.lerp(m1, t);
        let b = b0.lerp(b1, t);
        let dot = m.dot(b);
        return dot > 0. && dot < b.dot(b);
    }
    false
}
