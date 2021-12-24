use crate::vector::{Vector, N};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};

pub const IDENTITY: Matrix = Matrix([Vector([1, 0, 0]), Vector([0, 1, 0]), Vector([0, 0, 1])]);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Matrix(pub [Vector; N]);

impl Index<usize> for Matrix {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut result: Matrix = Default::default();

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[i][j] += self[i][k] * other[k][j];
                }
            }
        }

        result
    }
}

impl MulAssign<Self> for Matrix {
    fn mul_assign(&mut self, other: Matrix) {
        *self = *self * other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let matrix = Matrix([Vector([1, 2, 3]), Vector([4, 5, 6]), Vector([7, 8, 9])]);

        assert_eq!(
            matrix * matrix,
            Matrix([
                Vector([1 + 2 * 4 + 3 * 7, 2 + 2 * 5 + 3 * 8, 3 + 2 * 6 + 3 * 9]),
                Vector([
                    4 + 5 * 4 + 6 * 7,
                    4 * 2 + 5 * 5 + 6 * 8,
                    4 * 3 + 5 * 6 + 6 * 9
                ]),
                Vector([
                    7 + 8 * 4 + 9 * 7,
                    7 * 2 + 8 * 5 + 9 * 8,
                    7 * 3 + 8 * 6 + 9 * 9
                ]),
            ])
        );
    }
}
