// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cast::transmute;
use std::cmp::ApproxEq;
use std::num::{Zero, One};

use super::Vec3;

#[deriving(Eq)]
pub struct Vec2<T> { x: T, y: T }

impl<T> Vec2<T> {
    #[inline]
    pub fn index<'a>(&'a self, i: uint) -> &'a T {
        &'a self.as_slice()[i]
    }

    #[inline]
    pub fn index_mut<'a>(&'a mut self, i: uint) -> &'a mut T {
        &'a mut self.as_mut_slice()[i]
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T,..2] {
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T,..2] {
        unsafe { transmute(self) }
    }
}

impl<T:Copy> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T ) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }

    #[inline]
    pub fn from_value(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }

    #[inline]
    pub fn swap(&mut self, a: uint, b: uint) {
        let tmp = *self.index(a);
        *self.index_mut(a) = *self.index(b);
        *self.index_mut(b) = tmp;
    }

    #[inline(always)]
    pub fn map(&self, f: &fn(&T) -> T) -> Vec2<T> {
        Vec2::new(f(self.index(0)),
                  f(self.index(1)))
    }
}

impl<T:Copy + Num> Vec2<T> {
    #[inline]
    pub fn identity() -> Vec2<T> {
        Vec2::new(One::one::<T>(), One::one::<T>())
    }

    #[inline]
    pub fn zero() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn unit_x() -> Vec2<T> {
        Vec2::new(One::one::<T>(), Zero::zero::<T>())
    }

    #[inline]
    pub fn unit_y() -> Vec2<T> {
        Vec2::new(Zero::zero::<T>(), One::one::<T>())
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        *self.index(0) == Zero::zero() &&
        *self.index(1) == Zero::zero()
    }

    #[inline]
    pub fn add_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) + value,
                  *self.index(1) + value)
    }

    #[inline]
    pub fn sub_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) - value,
                  *self.index(1) - value)
    }

    #[inline]
    pub fn mul_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) * value,
                  *self.index(1) * value)
    }

    #[inline]
    pub fn div_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) / value,
                  *self.index(1) / value)
    }

    #[inline]
    pub fn rem_t(&self, value: T) -> Vec2<T> {
        Vec2::new(*self.index(0) % value,
                  *self.index(1) % value)
    }

    #[inline]
    pub fn add_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) + *other.index(0),
                  *self.index(1) + *other.index(1))
    }

    #[inline]
    pub fn sub_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) - *other.index(0),
                  *self.index(1) - *other.index(1))
    }

    #[inline]
    pub fn mul_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) * *other.index(0),
                  *self.index(1) * *other.index(1))
    }

    #[inline]
    pub fn div_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) / *other.index(0),
                  *self.index(1) / *other.index(1))
    }

    #[inline]
    pub fn rem_v(&self, other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(*self.index(0) % *other.index(0),
                  *self.index(1) % *other.index(1))
    }

    #[inline]
    pub fn neg_self(&mut self) {
        *self.index_mut(0) = -*self.index(0);
        *self.index_mut(1) = -*self.index(1);
    }

    #[inline]
    pub fn add_self_t(&mut self, value: T) {
        *self.index_mut(0) += value;
        *self.index_mut(1) += value;
    }

    #[inline]
    pub fn sub_self_t(&mut self, value: T) {
        *self.index_mut(0) -= value;
        *self.index_mut(1) -= value;
    }

    #[inline]
    pub fn mul_self_t(&mut self, value: T) {
        *self.index_mut(0) *= value;
        *self.index_mut(1) *= value;
    }

    #[inline]
    pub fn div_self_t(&mut self, value: T) {
        *self.index_mut(0) /= value;
        *self.index_mut(1) /= value;
    }

    #[inline]
    pub fn rem_self_t(&mut self, value: T) {
        *self.index_mut(0) %= value;
        *self.index_mut(1) %= value;
    }

    #[inline]
    pub fn add_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) += *other.index(0);
        *self.index_mut(1) += *other.index(1);
    }

    #[inline]
    pub fn sub_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) -= *other.index(0);
        *self.index_mut(1) -= *other.index(1);
    }

    #[inline]
    pub fn mul_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) *= *other.index(0);
        *self.index_mut(1) *= *other.index(1);
    }

    #[inline]
    pub fn div_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline]
    pub fn rem_self_v(&mut self, other: &Vec2<T>) {
        *self.index_mut(0) /= *other.index(0);
        *self.index_mut(1) /= *other.index(1);
    }

    #[inline]
    pub fn dot(&self, other: &Vec2<T>) -> T {
        *self.index(0) * *other.index(0) +
        *self.index(1) * *other.index(1)
    }

    #[inline]
    pub fn perp_dot(&self, other: &Vec2<T>) -> T {
        (*self.index(0) * *other.index(1)) -
        (*self.index(1) * *other.index(0))
    }

    #[inline]
    pub fn to_homogeneous(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, Zero::zero())
    }
}

impl<T:Copy + Num> Neg<Vec2<T>> for Vec2<T> {
    #[inline]
    pub fn neg(&self) -> Vec2<T> {
        Vec2::new(-self.index(0), -self.index(1))
    }
}

impl<T:Copy + Real> Vec2<T> {
    #[inline]
    pub fn length2(&self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length2().sqrt()
    }

    #[inline]
    pub fn distance2(&self, other: &Vec2<T>) -> T {
        other.sub_v(self).length2()
    }

    #[inline]
    pub fn distance(&self, other: &Vec2<T>) -> T {
        other.distance2(self).sqrt()
    }

    #[inline]
    pub fn angle(&self, other: &Vec2<T>) -> T {
        self.perp_dot(other).atan2(self.dot(other))
    }

    #[inline]
    pub fn normalize(&self) -> Vec2<T> {
        self.mul_t(One::one::<T>()/self.length())
    }

    #[inline]
    pub fn normalize_to(&self, length: T) -> Vec2<T> {
        self.mul_t(length / self.length())
    }

    #[inline]
    pub fn lerp(&self, other: &Vec2<T>, amount: T) -> Vec2<T> {
        self.add_v(&other.sub_v(self).mul_t(amount))
    }

    #[inline]
    pub fn normalize_self(&mut self) {
        let n = One::one::<T>() / self.length();
        self.mul_self_t(n);
    }

    #[inline]
    pub fn normalize_self_to(&mut self, length: T) {
        let n = length / self.length();
        self.mul_self_t(n);
    }

    pub fn lerp_self(&mut self, other: &Vec2<T>, amount: T) {
        let v = other.sub_v(self).mul_t(amount);
        self.add_self_v(&v);
    }
}

impl<T:Copy + Eq + ApproxEq<T>> ApproxEq<T> for Vec2<T> {
    #[inline]
    pub fn approx_epsilon() -> T {
        ApproxEq::approx_epsilon::<T,T>()
    }

    #[inline]
    pub fn approx_eq(&self, other: &Vec2<T>) -> bool {
        self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<T,T>())
    }

    #[inline]
    pub fn approx_eq_eps(&self, other: &Vec2<T>, epsilon: &T) -> bool {
        self.index(0).approx_eq_eps(other.index(0), epsilon) &&
        self.index(1).approx_eq_eps(other.index(1), epsilon)
    }
}

impl<T:Copy + Ord> Vec2<T> {
    #[inline]
    pub fn lt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) < value,
                  *self.index(1) < value)
    }

    #[inline]
    pub fn le_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= value,
                  *self.index(1) <= value)
    }

    #[inline]
    pub fn ge_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= value,
                  *self.index(1) >= value)
    }

    #[inline]
    pub fn gt_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) > value,
                  *self.index(1) > value)
    }

    #[inline]
    pub fn lt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) < *other.index(0),
                  *self.index(1) < *other.index(1))
    }

    #[inline]
    pub fn le_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) <= *other.index(0),
                  *self.index(1) <= *other.index(1))
    }

    #[inline]
    pub fn ge_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) >= *other.index(0),
                  *self.index(1) >= *other.index(1))
    }

    #[inline]
    pub fn gt_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) > *other.index(0),
                  *self.index(1) > *other.index(1))
    }
}

impl<T:Copy + Eq> Vec2<T> {
    #[inline]
    pub fn eq_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) == value,
                  *self.index(1) == value)
    }

    #[inline]
    pub fn ne_t(&self, value: T) -> Vec2<bool> {
        Vec2::new(*self.index(0) != value,
                  *self.index(1) != value)
    }

    #[inline]
    pub fn eq_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) == *other.index(0),
                  *self.index(1) == *other.index(1))
    }

    #[inline]
    pub fn ne_v(&self, other: &Vec2<T>) -> Vec2<bool> {
        Vec2::new(*self.index(0) != *other.index(0),
                  *self.index(1) != *other.index(1))
    }
}

impl Vec2<bool> {
    #[inline]
    pub fn any(&self) -> bool {
        *self.index(0) || *self.index(1)
    }

    #[inline]
    pub fn all(&self) -> bool {
        *self.index(0) && *self.index(1)
    }

    #[inline]
    pub fn not(&self) -> Vec2<bool> {
        Vec2::new(!*self.index(0), !*self.index(1))
    }
}
