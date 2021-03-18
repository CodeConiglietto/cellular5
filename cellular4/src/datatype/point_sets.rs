use std::{
    f32::consts::{PI, SQRT_2},
    ops::Index,
    sync::Arc,
};

use float_ord::FloatOrd;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use ndarray::Array2;
use rand::prelude::*;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct PointSet {
    points: Arc<Vec<SNPoint>>,
    generator: PointSetGenerator,
}

impl PointSet {
    #[track_caller]
    pub fn new(points: Arc<Vec<SNPoint>>, generator: PointSetGenerator) -> Self {
        assert!(points.len() > 0);
        assert!(points.len() <= 256);
        Self { points, generator }
    }

    pub fn get_offsets(&self, width: usize, height: usize) -> Vec<SNPoint> {
        let unit_x = 1.0 / width as f32;
        let unit_y = 1.0 / height as f32;
        let scale = SNPoint::new(Point2::new(unit_x, unit_y));

        self.points.iter().map(|p| p.scale_point(scale)).collect()
    }

    pub fn points(&self) -> &[SNPoint] {
        &*self.points
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn replace(&mut self, new_points: Arc<Vec<SNPoint>>) {
        *self = Self::new(new_points, self.generator)
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
        *self.points.choose(&mut thread_rng()).unwrap()
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        PointSetGenerator::random(rng).generate_point_set(rng)
    }
}

impl Default for PointSet {
    fn default() -> Self {
        PointSet::new(Arc::new(origin()), PointSetGenerator::Origin)
    }
}

impl Index<usize> for PointSet {
    type Output = SNPoint;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.points()[idx]
    }
}

impl Index<Byte> for PointSet {
    type Output = SNPoint;
    fn index(&self, idx: Byte) -> &Self::Output {
        &self[usize::from(idx.into_inner())]
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

    fn generate_rng<R: Rng + ?Sized>(rng: &mut R, _arg: GenArg<'a>) -> Self {
        Self::random(rng)
    }
}

impl<'a> Mutatable<'a> for PointSet {
    type MutArg = MutArg<'a>;
    fn mutate_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, _arg: MutArg<'a>) {
        *self = Self::random(rng);
    }
}

impl<'a> Updatable<'a> for PointSet {
    type UpdateArg = UpdArg<'a>;

    fn update(&mut self, _arg: UpdArg<'a>) {}
}

impl<'a> UpdatableRecursively<'a> for PointSet {
    fn update_recursively(&mut self, _arg: UpdArg<'a>) {}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PointSetGenerator {
    // Reasonable default - The Empty set is liable to crash some algorithms
    Origin,

    Moore,
    VonNeumann,
    UniformGrid {
        x_count: Nibble,
        y_count: Nibble,
    },
    SparseGrid {
        x_count: Nibble,
        y_count: Nibble,
        x_mod: Boolean,
        y_mod: Boolean,
    },
    HexGrid {
        x_count: Nibble,
        y_count: Nibble,
    },
    TriGrid {
        x_count: Nibble,
        y_count: Nibble,
    },
    UniformDistribution {
        count: Byte,
    },
    Poisson {
        count: Byte,
        radius: UNFloat,
    },
    Spiral {
        count: Byte,
        scalar: UNFloat,
        maximum: Angle,
        linear: Boolean,
        nonlinearity_factor_halved: UNFloat, //This is the easiest way to introduce a variable nonlinearity which includes both squaring and square rooting
    },
    RandomRings {
        max_rings: Nibble,
    },
    LinearIncreasingRings {
        max_count: Byte,         //full count will be less than this
        ring_size_delta: Nibble, //full count will be less than this
    },
    FibonacciRings {
        max_count: Byte, //full count will be less than this
    },
    //TODO add fibonacci spiral also
    SquaredRings {
        max_count: Byte, //full count will be less than this
    },
}

impl PointSetGenerator {
    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        match rng.gen_range(0, 13) {
            // Skip Origin
            0 => PointSetGenerator::Moore,
            1 => PointSetGenerator::VonNeumann,
            2 => PointSetGenerator::UniformGrid {
                x_count: Nibble::random(rng),
                y_count: Nibble::random(rng),
            },
            3 => PointSetGenerator::SparseGrid {
                x_count: Nibble::random(rng),
                y_count: Nibble::random(rng),
                x_mod: Boolean::random(rng),
                y_mod: Boolean::random(rng),
            },
            4 => PointSetGenerator::TriGrid {
                x_count: Nibble::random(rng),
                y_count: Nibble::random(rng),
            },
            5 => PointSetGenerator::HexGrid {
                x_count: Nibble::random(rng),
                y_count: Nibble::random(rng),
            },
            6 => PointSetGenerator::UniformDistribution {
                count: Byte::random(rng),
            },
            7 => PointSetGenerator::Poisson {
                count: Byte::random(rng),
                radius: UNFloat::random(rng),
            },
            8 => PointSetGenerator::Spiral {
                count: Byte::random(rng),
                scalar: UNFloat::random(rng),
                maximum: Angle::random(rng),
                linear: Boolean::random(rng),
                nonlinearity_factor_halved: UNFloat::random(rng),
            },
            9 => PointSetGenerator::RandomRings {
                max_rings: Nibble::random(rng),
            },
            10 => PointSetGenerator::LinearIncreasingRings {
                max_count: Byte::random(rng),
                ring_size_delta: Nibble::random(rng),
            },
            11 => PointSetGenerator::FibonacciRings {
                max_count: Byte::random(rng),
            },
            12 => PointSetGenerator::SquaredRings {
                max_count: Byte::random(rng),
            },
            _ => unreachable!(),
        }
    }

    pub fn generate_point_set<R: Rng + ?Sized>(&self, rng: &mut R) -> PointSet {
        let points = match self {
            PointSetGenerator::Origin => origin(),
            PointSetGenerator::Moore => moore(),
            PointSetGenerator::VonNeumann => von_neumann(),
            PointSetGenerator::UniformGrid { x_count, y_count } => {
                let x_count = x_count.into_inner() + 1;
                let y_count = y_count.into_inner() + 1;

                let x_ratio = 1.0 / x_count as f32;
                let y_ratio = 1.0 / y_count as f32;

                (0..x_count)
                    .flat_map(|x| {
                        (0..y_count).map(move |y| {
                            SNPoint::new(Point2::new(
                                2.0 * (x_ratio * x as f32 + x_ratio * 0.5) - 1.0,
                                2.0 * (y_ratio * y as f32 + y_ratio * 0.5) - 1.0,
                            ))
                        })
                    })
                    .collect()
            }
            PointSetGenerator::SparseGrid {
                x_count,
                y_count,
                x_mod,
                y_mod,
            } => {
                let x_count = x_count.into_inner() + 1;
                let y_count = y_count.into_inner() + 1;

                let x_count = if x_count % 2 == 0 {
                    x_count + 1
                } else {
                    x_count
                };
                let y_count = if y_count % 2 == 0 {
                    y_count + 1
                } else {
                    y_count
                };

                let x_mod = if x_mod.into_inner() { 1 } else { 0 };
                let y_mod = if y_mod.into_inner() { 1 } else { 0 };

                let x_ratio = 1.0 / x_count as f32;
                let y_ratio = 1.0 / y_count as f32;

                (0..x_count)
                    .flat_map(|x| {
                        (0..y_count)
                            .filter(move |y| !(x % 2 == x_mod && y % 2 == y_mod))
                            .map(move |y| {
                                SNPoint::new(Point2::new(
                                    2.0 * (x_ratio * x as f32 + x_ratio * 0.5) - 1.0,
                                    2.0 * (y_ratio * y as f32 + y_ratio * 0.5) - 1.0,
                                ))
                            })
                    })
                    .collect()
            }
            PointSetGenerator::TriGrid { x_count, y_count } => {
                let x_count = x_count.into_inner() + 1;
                let y_count = y_count.into_inner() + 1;

                let x_ratio = 1.0 / x_count as f32;
                let y_ratio = 1.0 / y_count as f32;
                (0..x_count)
                    .flat_map(|x| {
                        (0..y_count).map(move |y| {
                            SNPoint::new(Point2::new(
                                2.0 * (x_ratio * x as f32
                                    + if y % 2 == 0 {
                                        0.25 * x_ratio
                                    } else {
                                        0.75 * x_ratio
                                    })
                                    - 1.0,
                                2.0 * (y_ratio * y as f32 + y_ratio * 0.5) - 1.0,
                            ))
                        })
                    })
                    .collect()
            }
            PointSetGenerator::HexGrid { x_count, y_count } => {
                let x_count = x_count.into_inner() + 1;
                let y_count = y_count.into_inner() + 1;

                //I think x needs to be even and y needs to be odd to ensure this works properly around the right and bottom edges
                let x_count = match x_count % 3 {
                    0 => x_count + 2,
                    1 => x_count + 1,
                    2 => x_count,
                    _ => unreachable!(),
                };
                let y_count = if y_count % 2 == 1 {
                    y_count + 1
                } else {
                    y_count
                };

                let x_ratio = 1.0 / x_count as f32;
                let y_ratio = 1.0 / y_count as f32;
                (0..x_count)
                    .flat_map(|x| {
                        (0..y_count)
                            .filter(move |y| !(y % 2 == x % 3))
                            .map(move |y| {
                                SNPoint::new(Point2::new(
                                    2.0 * (x_ratio * x as f32
                                        + if y % 2 == 0 {
                                            0.25 * x_ratio
                                        } else {
                                            0.75 * x_ratio
                                        })
                                        - 1.0,
                                    2.0 * (y_ratio * y as f32 + y_ratio * 0.5) - 1.0,
                                ))
                            })
                    })
                    .collect()
            }
            PointSetGenerator::UniformDistribution { count } => {
                uniform(rng, count.into_inner().max(2) as usize)
            }
            PointSetGenerator::Poisson { count, radius } => {
                let normaliser = SFloatNormaliser::generate_rng(rng, ());

                poisson(
                    rng,
                    count.into_inner().max(4) as usize,
                    (2.0 * radius.into_inner() / (count.into_inner() as f32).sqrt().max(2.0))
                        .max(0.01),
                    normaliser,
                )
            }
            PointSetGenerator::Spiral {
                count,
                scalar,
                maximum,
                linear,
                nonlinearity_factor_halved,
            } => {
                let count = count.into_inner().max(1);
                let scalar = scalar.into_inner();
                let maximum = maximum.into_inner();
                let linear = linear.into_inner();
                let nonlinearity_factor = nonlinearity_factor_halved.into_inner() * 2.0;

                (0..count)
                    .map(|i| {
                        let rho = i as f32 / count as f32;

                        let theta = count as f32
                            * maximum
                            * scalar
                            * if linear {
                                rho
                            } else {
                                rho.powf(nonlinearity_factor)
                            };
                        SNPoint::from_snfloats(
                            SNFloat::new(rho * f32::sin(theta)),
                            SNFloat::new(rho * f32::cos(theta)),
                        )
                    })
                    .collect()
            }
            PointSetGenerator::RandomRings { max_rings } => {
                let mut sequence = Vec::new();

                let max_rings = max_rings.into_inner() + 1;

                for _ in 0..max_rings {
                    sequence.push(Nibble::random(rng).into_inner() + 1);
                }

                let sequence_value_count = sequence.len();

                sequence
                    .iter()
                    .enumerate()
                    .flat_map(|(index, point_count)| {
                        (0..*point_count).map(move |i| {
                            let theta = i as f32 * (2.0 * PI / *point_count as f32) - PI;
                            let rho = index as f32 * 1.0 / sequence_value_count as f32;

                            SNPoint::from_snfloats(
                                SNFloat::new(rho * f32::sin(theta)),
                                SNFloat::new(rho * f32::cos(theta)),
                            )
                        })
                    })
                    .collect()
            }
            PointSetGenerator::LinearIncreasingRings {
                max_count,
                ring_size_delta,
            } => {
                let mut prev_total: u16 = 0;
                let mut new_total: u16 = 1;

                let mut total_total: u16 = 0;

                let ring_size_delta = ring_size_delta.into_inner() as u16;

                let mut sequence = Vec::new();

                let max_count = max_count.into_inner().max(1);

                loop {
                    let current_total = new_total;
                    new_total = prev_total + ring_size_delta;
                    prev_total = current_total;

                    total_total += new_total;

                    if total_total <= max_count as u16 || sequence.is_empty() {
                        sequence.push(prev_total);
                    } else {
                        break;
                    }
                }

                let sequence_value_count = sequence.len();

                sequence
                    .iter()
                    .enumerate()
                    .flat_map(|(index, point_count)| {
                        (0..*point_count).map(move |i| {
                            let theta = i as f32 * (2.0 * PI / *point_count as f32) - PI;
                            let rho = index as f32 * 1.0 / sequence_value_count as f32;

                            SNPoint::from_snfloats(
                                SNFloat::new(rho * f32::sin(theta)),
                                SNFloat::new(rho * f32::cos(theta)),
                            )
                        })
                    })
                    .collect()
            }
            PointSetGenerator::FibonacciRings { max_count } => {
                let mut prev_total: u16 = 0;
                let mut new_total: u16 = 1;

                let mut total_total: u16 = 0;

                let mut sequence = Vec::new();

                let max_count = max_count.into_inner().max(1);

                loop {
                    let current_total = new_total;
                    new_total += prev_total;
                    prev_total = current_total;

                    total_total += new_total;

                    if total_total <= max_count as u16 || sequence.is_empty() {
                        sequence.push(prev_total);
                    } else {
                        break;
                    }
                }

                let sequence_value_count = sequence.len();

                sequence
                    .iter()
                    .enumerate()
                    .flat_map(|(index, point_count)| {
                        (0..*point_count).map(move |i| {
                            let theta = i as f32 * (2.0 * PI / *point_count as f32) - PI;
                            let rho = index as f32 * 1.0 / sequence_value_count as f32;

                            SNPoint::from_snfloats(
                                SNFloat::new(rho * f32::sin(theta)),
                                SNFloat::new(rho * f32::cos(theta)),
                            )
                        })
                    })
                    .collect()
            }
            PointSetGenerator::SquaredRings { max_count } => {
                let mut prev_total: u16 = 0;
                let mut new_total: u16 = 1;

                let mut total_total: u16 = 0;

                let mut sequence = Vec::new();

                let max_count = max_count.into_inner().max(1);

                loop {
                    let current_total = new_total;
                    new_total = prev_total * 2;
                    prev_total = current_total;

                    total_total += new_total;

                    if total_total <= max_count as u16 || sequence.is_empty() {
                        sequence.push(prev_total);
                    } else {
                        break;
                    }
                }

                let sequence_value_count = sequence.len();

                sequence
                    .iter()
                    .enumerate()
                    .flat_map(|(index, point_count)| {
                        (0..*point_count).map(move |i| {
                            let theta = i as f32 * (2.0 * PI / *point_count as f32) - PI;
                            let rho = index as f32 * 1.0 / sequence_value_count as f32;

                            SNPoint::from_snfloats(
                                SNFloat::new(rho * f32::sin(theta)),
                                SNFloat::new(rho * f32::cos(theta)),
                            )
                        })
                    })
                    .collect()
            }
        };

        assert!(
            points.len() > 0,
            "assertion failed: points.len() > 0, generator is {:?}",
            self
        );

        PointSet::new(Arc::new(points), *self)
    }

    fn load(&self) -> PointSet {
        self.generate_point_set(&mut rand::thread_rng())
    }
}

impl Default for PointSetGenerator {
    fn default() -> Self {
        PointSetGenerator::Origin
    }
}

fn origin() -> Vec<SNPoint> {
    vec![SNPoint::zero()]
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
    normaliser: SFloatNormaliser,
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
