use nalgebra::OVector;
use nalgebra::base::allocator::Allocator;
use nalgebra::base::{DefaultAllocator, DimName};
use num_traits::NumAssignOps;
use num_traits::float::FloatCore;
use num_traits::identities::One;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

// pub trait NumAssign: Num + NumAssignOps {}

// pub trait NumOps<Rhs = Self, Output = Self>:
//     Add<Rhs, Output = Output>
//     + Sub<Rhs, Output = Output>
//     + Mul<Rhs, Output = Output>
//     + Div<Rhs, Output = Output>
//     + Rem<Rhs, Output = Output>

// Num: PartialEq + Zero + One + NumOps
// FloatCore: Num + NumCast + Neg<Output = Self> + PartialOrd + Copy

/// A scalar type.
///
/// Scalars are used for things like knot locations, weights, parameter values,
/// and the scalar components of vector types.
// pub trait ScalarT:
//     Copy
//     + PartialOrd
//     + Debug
//     + Add<Output = Self>
//     + AddAssign
//     + Mul<Output = Self>
//     + MulAssign
//     + Sub<Output = Self>
//     + Div<Output = Self>
//     + One
// {
// }

pub trait ScalarT:
    FloatCore + NumAssignOps + Debug
{
}

impl<T> ScalarT for T where
    T: FloatCore + NumAssignOps + Debug
{
}

/// A vector type.
///
/// Vectors are used for 3D locations like control points and points on curves
/// or surfaces.
pub trait VectorT:
    Clone + Debug + Add<Output = Self> + Mul<<Self as VectorT>::Field, Output = Self>
{
    type Field: ScalarT;
}

impl<N, D> VectorT for OVector<N, D>
where
    N: 'static + ScalarT,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    type Field = N;
}
