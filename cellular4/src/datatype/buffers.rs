use std::{
    fmt::{self, Debug, Formatter},
    iter,
    ops::{Index, IndexMut},
};

use bresenham::Bresenham;
use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use nalgebra::*;
use ndarray::prelude::*;
use rand::prelude::*;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

use crate::prelude::*;

pub struct Buffer<T> {
    array: Array2<T>,
}

impl<T> Buffer<T> {
    pub fn new(array: Array2<T>) -> Self {
        Self { array }
    }

    pub fn point_to_uint(&self, coords: SNPoint) -> Point2<usize> {
        let (height, width) = self.array.dim();

        Point2::new(
            ((coords.x().to_unsigned().into_inner() * width as f32) as usize).min(width - 1),
            ((coords.y().to_unsigned().into_inner() * height as f32) as usize).min(height - 1),
        )
    }

    pub fn info(&self) -> BufferInfo {
        let (height, width) = self.array.dim();
        BufferInfo { width, height }
    }
}

impl<T: Clone> Buffer<T> {
    pub fn draw_line(&mut self, from: SNPoint, to: SNPoint, value: T) {
        let from_uint = self.point_to_uint(from);
        let from_bresenham = (from_uint.x as isize, from_uint.y as isize);

        let to_uint = self.point_to_uint(to);
        let to_bresenham = (to_uint.x as isize, to_uint.y as isize);

        for point_bresenham in
            Bresenham::new(from_bresenham, to_bresenham).chain(iter::once(to_bresenham))
        {
            let point_uint = Point2::new(point_bresenham.0 as usize, point_bresenham.1 as usize);
            self[point_uint] = value.clone();
        }
    }

    pub fn draw_dot(&mut self, pos: SNPoint, value: T) {
        let point_uint = self.point_to_uint(pos);
        self[point_uint] = value;
    }
}

impl<T> Index<SNPoint> for Buffer<T> {
    type Output = T;

    fn index(&self, index: SNPoint) -> &Self::Output {
        let p = self.point_to_uint(index);
        &self[p]
    }
}

impl<T> IndexMut<SNPoint> for Buffer<T> {
    fn index_mut(&mut self, index: SNPoint) -> &mut Self::Output {
        let p = self.point_to_uint(index);
        &mut self[p]
    }
}

impl<T> Index<Point2<usize>> for Buffer<T> {
    type Output = T;

    fn index(&self, index: Point2<usize>) -> &Self::Output {
        &self.array[[index.y, index.x]]
    }
}

impl<T> IndexMut<Point2<usize>> for Buffer<T> {
    fn index_mut(&mut self, index: Point2<usize>) -> &mut Self::Output {
        &mut self.array[[index.y, index.x]]
    }
}

impl<T> Debug for Buffer<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Buffer")
            .field("dimensions", &self.array.dim())
            .field("type", &std::any::type_name::<T>())
            .finish()
    }
}

impl<T> Serialize for Buffer<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.info().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Buffer<T>
where
    T: Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BufferInfo::deserialize(deserializer)?.load())
    }
}

impl<'a, T: Default + Generatable<'a>> Generatable<'a> for Buffer<T> {
    type GenArg = T::GenArg;

    fn generate_rng<R: Rng + ?Sized>(
        _rng: &mut R,
        _state: mutagen::State,
        _arg: Self::GenArg,
    ) -> Self {
        Self::new(Array2::from_shape_fn(
            (CONSTS.cell_array_height, CONSTS.cell_array_width),
            |(_y, _x)| T::default(), //generate_rng(rng, state, arg.clone()),
        ))
    }
}

impl<'a, T: Mutatable<'a>> Mutatable<'a> for Buffer<T> {
    type MutArg = T::MutArg;

    fn mutate_rng<R: Rng + ?Sized>(
        &mut self,
        _rng: &mut R,
        _state: mutagen::State,
        _arg: Self::MutArg,
    ) {
        //TODO: find a way to mutate this that doesn't look like a rainbow static explosion
        // for inner in self.array.iter_mut() {
        //     inner.mutate_rng(rng, state, arg.clone());
        // }
    }
}

impl<'a, T: Updatable<'a>> Updatable<'a> for Buffer<T> {
    type UpdateArg = T::UpdateArg;

    fn update(&mut self, _state: mutagen::State, _arg: Self::UpdateArg) {}
}

impl<'a, T: UpdatableRecursively<'a>> UpdatableRecursively<'a> for Buffer<T> {
    fn update_recursively(&mut self, _state: mutagen::State, _arg: Self::UpdateArg) {}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BufferInfo {
    width: usize,
    height: usize,
}

impl BufferInfo {
    fn load<T>(&self) -> Buffer<T>
    where
        T: Default,
    {
        Buffer::new(Array2::default([self.height, self.width]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ndarray::array;

    #[test]
    fn point_to_uint_tests() {
        let buffer = Buffer::new(Array2::from_elem((100, 100), 0u32));

        test_point_to_uint(&buffer, (-1.0, -1.0), (0, 0));
        test_point_to_uint(&buffer, (0.0, 0.0), (50, 50));
        test_point_to_uint(&buffer, (1.0, 1.0), (99, 99));
    }

    fn test_point_to_uint<T>(buffer: &Buffer<T>, p: (f32, f32), expected: (usize, usize)) {
        assert_eq!(
            buffer.point_to_uint(SNPoint::new(Point2::new(p.0, p.1))),
            Point2::new(expected.0, expected.1)
        );
    }

    #[test]
    fn draw_line_tests() {
        test_draw_line(
            (-1.0, -1.0),
            (-0.01, -0.01),
            array![
                [1, 0, 0, 0], // comment for cargo fmt
                [0, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        );

        test_draw_line(
            (-1.0, -1.0),
            (0.0, 0.0),
            array![
                [1, 0, 0, 0], // comment for cargo fmt
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
        );

        test_draw_line(
            (-1.0, -1.0),
            (1.0, 1.0),
            array![
                [1, 0, 0, 0], // comment for cargo fmt
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        );

        test_draw_line(
            (1.0, -1.0),
            (1.0, 1.0),
            array![
                [0, 0, 0, 1], // comment for cargo fmt
                [0, 0, 0, 1],
                [0, 0, 0, 1],
                [0, 0, 0, 1],
            ],
        );

        test_draw_line(
            (-1.0, 1.0),
            (1.0, -1.0),
            array![
                [0, 0, 0, 1], // comment for cargo fmt
                [0, 0, 1, 0],
                [0, 1, 0, 0],
                [1, 0, 0, 0],
            ],
        );
    }

    fn test_draw_line(from: (f32, f32), to: (f32, f32), expected: Array2<u32>) {
        let mut buffer = Buffer::new(Array2::from_elem(expected.dim(), 0u32));
        buffer.draw_line(
            SNPoint::new(Point2::new(from.0, from.1)),
            SNPoint::new(Point2::new(to.0, to.1)),
            1,
        );
        assert!(
            buffer.array == expected,
            "mismatching arrays:\nGot:\n{}\nExpected:\n{}",
            &buffer.array,
            &expected
        );
    }
}
