// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<E: Environment> Zero for Affine<E> {
    type Boolean = Boolean<E>;
    type Output = Self::Boolean;

    fn zero() -> Self {
        Self::from(Field::zero(), Field::one())
    }

    fn is_zero(&self) -> Self::Output {
        let is_x_zero = self.x.is_eq(&Field::zero());
        let is_y_one = self.y.is_eq(&Field::one());
        is_x_zero.and(&is_y_one)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Circuit;

    #[test]
    fn test_zero() {
        let zero = <Circuit as Environment>::BaseField::zero();
        let one = <Circuit as Environment>::BaseField::one();

        Circuit::scoped("Zero", |scope| {
            assert_eq!(0, Circuit::num_constants());
            assert_eq!(1, Circuit::num_public());
            assert_eq!(0, Circuit::num_private());
            assert_eq!(0, Circuit::num_constraints());

            assert_eq!(0, scope.num_constants_in_scope());
            assert_eq!(0, scope.num_public_in_scope());
            assert_eq!(0, scope.num_private_in_scope());
            assert_eq!(0, scope.num_constraints_in_scope());

            let candidate = Affine::<Circuit>::zero();
            assert_eq!((zero, one), candidate.to_value());

            assert_eq!(6, Circuit::num_constants());
            assert_eq!(1, Circuit::num_public());
            assert_eq!(1, Circuit::num_private());
            assert_eq!(2, Circuit::num_constraints());

            assert_eq!(6, scope.num_constants_in_scope());
            assert_eq!(0, scope.num_public_in_scope());
            assert_eq!(1, scope.num_private_in_scope());
            assert_eq!(2, scope.num_constraints_in_scope());
        });
    }

    #[test]
    fn test_is_zero() {
        let candidate = Affine::<Circuit>::zero();

        // Should equal 0.
        let candidate_boolean = candidate.is_zero();
        assert_eq!(true, candidate_boolean.to_value());
    }
}