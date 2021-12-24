use crate::matrix::Matrix;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::{Index, IndexMut};

pub const N: usize = 3;

pub type Value = isize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector(pub [Value; N]);

impl Vector {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

impl Index<usize> for Vector {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Value>> for Vector {
    fn from(values: Vec<Value>) -> Self {
        assert_eq!(values.len(), N);

        let mut v: Self = Default::default();

        for i in 0..N {
            v[i] = values[i];
        }

        v
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result: Self = Default::default();

        for i in 0..N {
            result[i] = self[i] + other[i];
        }

        result
    }
}

impl AddAssign<Self> for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result: Self = Default::default();

        for i in 0..N {
            result[i] = self[i] - other[i];
        }

        result
    }
}

impl SubAssign<Self> for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Mul<Matrix> for Vector {
    type Output = Self;

    fn mul(self, other: Matrix) -> Self::Output {
        let mut result: Vector = Default::default();

        for i in 0..N {
            for j in 0..N {
                result[i] += self[j] * other[j][i];
            }
        }

        result
    }
}

impl MulAssign<Matrix> for Vector {
    fn mul_assign(&mut self, other: Matrix) {
        *self = *self * other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vector() {
        assert_eq!(Vector([1, 2, 3]) + Vector([4, 5, 6]), Vector([5, 7, 9]));
    }

    #[test]
    fn test_sub_vector() {
        assert_eq!(Vector([6, 8, 10]) - Vector([5, 6, 7]), Vector([1, 2, 3]));
    }

    #[test]
    fn test_mul_matrix() {
        assert_eq!(
            Vector([1, 2, 3]) * Matrix([Vector([1, 2, 3]), Vector([4, 5, 6]), Vector([7, 8, 9]),]),
            Vector([1 + 2 * 4 + 3 * 7, 2 + 2 * 5 + 3 * 8, 3 + 2 * 6 + 3 * 9])
        );
    }
}
