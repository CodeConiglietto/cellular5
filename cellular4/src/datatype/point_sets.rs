use std::{
    f32::consts::{PI, SQRT_2},
    sync::Arc,
};

use float_ord::FloatOrd;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use ndarray::Array2;
use rand::prelude::*;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::{
    datatype::{constraint_resolvers::*, continuous::*, discrete::*, points::*},
    mutagen_args::*,
};

#[derive(Clone, Debug)]
pub struct PointSet {
    pub points: Arc<Vec<SNPoint>>,
    pub generator: PointSetGenerator,
}

impl PointSet {
    pub fn new(points: Arc<Vec<SNPoint>>, generator: PointSetGenerator) -> Self {
        Self { points, generator }
    }

    pub fn get_offsets(&self, width: usize, height: usize) -> Vec<SNPoint> {
        let unit_x = 1.0 / width as f32;
        let unit_y = 1.0 / height as f32;
        let scale = SNPoint::new(Point2::new(unit_x, unit_y));

        self.points.iter().map(|p| p.scale_point(scale)).collect()
    }

    pub fn get_at(&self, index: usize) -> SNPoint {
        self.points[index]
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn get_closest_point(&self, other: SNPoint) -> SNPoint {
        *self
            .points
            .iter()
            .filter(|p| p.into_inner() != other.into_inner())
            .min_by_key(|p| FloatOrd(distance(&p.into_inner(), &other.into_inner())))
            .unwrap_or(&other)
    }

    pub fn get_furthest_point(&self, other: SNPoint) -> SNPoint {
        *self
            .points
            .iter()
            .filter(|p| p.into_inner() != other.into_inner())
            .max_by_key(|p| FloatOrd(distance(&p.into_inner(), &other.into_inner())))
            .unwrap_or(&other)
    }

    pub fn get_n_closest_points(&mut self, other: SNPoint, n: usize) -> &[SNPoint] {
        Arc::make_mut(&mut self.points).sort_by_key(|p| {
            let d = distance(&p.into_inner(), &other.into_inner());
            (d != 0.0, FloatOrd(d))
        });

        &self.points[0..n.min(self.points.len())]
    }

    pub fn get_random_point(&self) -> SNPoint {
        self.points.choose(&mut thread_rng()).unwrap().clone()
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        PointSetGenerator::random(rng).generate_point_set(rng)
    }
}

impl Serialize for PointSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.generator.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PointSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PointSetGenerator::deserialize(deserializer)?.load())
    }
}

impl<'a> Generatable<'a> for PointSet {
    type GenArg = GenArg<'a>;

    fn generate_rng<R: Rng + ?Sized>(
        rng: &mut R,
        _state: mutagen::State,
        _arg: GenArg<'a>,
    ) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for PointSet {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        _state: mutagen::State,
        _arg: MutArg<'a>,
    ) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for PointSet {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

impl<'a> UpdatableRecursively<'a> for PointSet {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: UpdArg<'a>) {
        match self {
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PointSetGenerator {
    Moore,
    VonNeumann,
    Uniform { count: Byte },
    Poisson { count: Byte, radius: UNFloat },
}

impl PointSetGenerator {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0, 4) {
            0 => PointSetGenerator::Moore,
            1 => PointSetGenerator::VonNeumann,
            2 => PointSetGenerator::Uniform {
                count: Byte::random(rng),
            },
            3 => PointSetGenerator::Poisson {
                count: Byte::random(rng),
                radius: UNFloat::random(rng),
            },
            _ => unreachable!(),
        }
    }

    pub fn generate_point_set<R: Rng + ?Sized>(&self, rng: &mut R) -> PointSet {
        let points = match self {
            PointSetGenerator::Moore => moore(),
            PointSetGenerator::VonNeumann => von_neumann(),
            PointSetGenerator::Uniform { count } => {
                uniform(rng, count.into_inner().max(2) as usize)
            }
            PointSetGenerator::Poisson { count, radius } => {
                let normaliser =
                    SNFloatNormaliser::generate_rng(rng, mutagen::State::default(), ());

                poisson(
                    rng,
                    count.into_inner().max(4) as usize,
                    (2.0 * radius.into_inner() / (count.into_inner() as f32).sqrt().max(2.0))
                        .max(0.01),
                    normaliser,
                )
            }
        };

        PointSet::new(Arc::new(points), *self)
    }

    fn load(&self) -> PointSet {
        self.generate_point_set(&mut rand::thread_rng())
    }
}

fn moore() -> Vec<SNPoint> {
    vec![
        SNPoint::from_snfloats(SNFloat::NEG_ONE, SNFloat::NEG_ONE),
        SNPoint::from_snfloats(SNFloat::NEG_ONE, SNFloat::ZERO),
        SNPoint::from_snfloats(SNFloat::NEG_ONE, SNFloat::ONE),
        SNPoint::from_snfloats(SNFloat::ZERO, SNFloat::NEG_ONE),
        SNPoint::from_snfloats(SNFloat::ZERO, SNFloat::ONE),
        SNPoint::from_snfloats(SNFloat::ONE, SNFloat::NEG_ONE),
        SNPoint::from_snfloats(SNFloat::ONE, SNFloat::ZERO),
        SNPoint::from_snfloats(SNFloat::ONE, SNFloat::ONE),
    ]
}

fn von_neumann() -> Vec<SNPoint> {
    vec![
        SNPoint::from_snfloats(SNFloat::ONE, SNFloat::ZERO),
        SNPoint::from_snfloats(SNFloat::NEG_ONE, SNFloat::ZERO),
        SNPoint::from_snfloats(SNFloat::ZERO, SNFloat::ONE),
        SNPoint::from_snfloats(SNFloat::ZERO, SNFloat::NEG_ONE),
    ]
}

pub fn uniform<R: Rng + ?Sized>(rng: &mut R, count: usize) -> Vec<SNPoint> {
    (0..count)
        .map(|_| SNPoint::new(Point2::new(rng.gen(), rng.gen())))
        .collect()
}

pub fn poisson<R: Rng + ?Sized>(
    rng: &mut R,
    count: usize,
    radius: f32,
    normaliser: SNFloatNormaliser,
) -> Vec<SNPoint> {
    assert!(radius > 0.0);
    assert!(count > 0);

    let cell_size = radius / SQRT_2;
    let grid_size = (1.0 / cell_size).ceil() as usize * 2;

    let p_to_grid = |p: SNPoint| {
        [
            (((p.x().into_inner() + 1.0) / cell_size).floor() as usize).min(grid_size - 1),
            (((p.y().into_inner() + 1.0) / cell_size).floor() as usize).min(grid_size - 1),
        ]
    };

    let mut grid: Array2<Option<u16>> = Array2::from_elem((grid_size, grid_size), None);
    let mut points = Vec::with_capacity(count);
    let mut active = Vec::with_capacity(count);

    let p0 = SNPoint::new(Point2::new(rng.gen(), rng.gen()));
    points.push(p0);
    active.push(0);
    grid[p_to_grid(p0)] = Some(0);

    // Arbitrary parameter for number of neighbouring points to attempt
    const K: usize = 30;

    while points.len() < count && !active.is_empty() {
        let active_idx = rng.gen_range(0, active.len());
        let p = points[active[active_idx]];
        let mut attempts = 0;

        let new_p = 'candidates: loop {
            attempts += 1;

            if attempts > K {
                break None;
            }

            let theta = rng.gen_range(0.0, 2.0 * PI);
            let r = rng.gen_range(radius, radius * 2.0);
            let dx = f32::cos(theta) * r;
            let dy = f32::sin(theta) * r;

            let new_p = SNPoint::from_snfloats(
                normaliser.normalise(p.x().into_inner() + dx),
                normaliser.normalise(p.y().into_inner() + dy),
            );

            let [gx, gy] = p_to_grid(new_p);

            for tx in -1i16..=1 {
                for ty in -1i16..=1 {
                    if let Some(i) = grid[[
                        ((gx as i16 + tx).max(0) as usize).min(grid_size - 1),
                        ((gy as i16 + ty).max(0) as usize).min(grid_size - 1),
                    ]] {
                        // TODO Parametrize to arbitrary distance functions
                        if distance(&points[i as usize].into_inner(), &new_p.into_inner()) <= radius
                        {
                            continue 'candidates;
                        }
                    }
                }
            }

            break Some(new_p);
        };

        if let Some(new_p) = new_p {
            grid[p_to_grid(new_p)] = Some(points.len() as u16);
            active.push(points.len());
            points.push(new_p);
        } else {
            active.remove(active_idx);
        }
    }

    points
}
