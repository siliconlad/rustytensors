use crate::vector::traits::VectorOps;
use crate::vector::vector_impl::Vector;
use crate::vector::vector_view::VectorView;
use crate::vector::vector_view_mut::VectorViewMut;
use funty::Numeric;
use std::ops::Add;
use std::borrow::Borrow;

fn v_add<T: Numeric, const N: usize>(
    lhs: &dyn VectorOps<T, N>,
    rhs: &dyn VectorOps<T, N>,
) -> Vector<T, N> {
    Vector::<T, N>::new(std::array::from_fn(|i| lhs[i] + rhs[i]))
}

fn v_add_scalar<T: Numeric, const N: usize>(v: &dyn VectorOps<T, N>, scalar: T) -> Vector<T, N> {
    Vector::<T, N>::new(std::array::from_fn(|i| v[i] + scalar))
}

//////////////
//  Vector  //
//////////////

impl<T: Numeric, const N: usize> Add<Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<&Vector<T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<T: Numeric, const N: usize> Add<Vector<T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<&Vector<T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<T: Numeric, const N: usize> Add<T> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        v_add_scalar(&self, scalar)
    }
}

impl<T: Numeric, const N: usize> Add<T> for &Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        v_add_scalar(self, scalar)
    }
}

//////////////////
//  VectorView  //
//////////////////

impl<'a, T: Numeric, const N: usize> Add<Vector<T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<Vector<T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&Vector<T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&Vector<T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + other[i]))
    }
}

impl<'a, T: Numeric, const N: usize> Add<T> for VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + scalar))
    }
}

impl<'a, T: Numeric, const N: usize> Add<T> for &VectorView<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        Vector::<T, N>::new(std::array::from_fn(|i| self[i] + scalar))
    }
}

/////////////////////
//  VectorViewMut  //
/////////////////////

impl<'a, T: Numeric, const N: usize> Add<Vector<T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&Vector<T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<Vector<T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: Vector<T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&Vector<T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &Vector<T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorView<'_, T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorView<'_, T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorView<'_, T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorView<'_, T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(&self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(&self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<VectorViewMut<'_, T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(self, &other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<&VectorViewMut<'_, T, N>> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, other: &VectorViewMut<'_, T, N>) -> Self::Output {
        v_add(self, other)
    }
}

impl<'a, T: Numeric, const N: usize> Add<T> for VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        v_add_scalar(&self, scalar)
    }
}

impl<'a, T: Numeric, const N: usize> Add<T> for &VectorViewMut<'a, T, N> {
    type Output = Vector<T, N>;

    fn add(self, scalar: T) -> Self::Output {
        v_add_scalar(self, scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        // Vector + Vector
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1 + v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Reference addition for f64
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1 + &v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Mixed reference and non-reference addition for f64
        let result = v1 + &v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let result = &v1 + v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Vector + VectorView
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1 + v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &Vector + VectorView
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1 + v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Vector + &VectorView
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1 + &v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &Vector + &VectorView
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1 + &v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Vector + VectorViewMut
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1 + v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &Vector + VectorViewMut
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1 + v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Vector + &VectorViewMut
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1 + &v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &Vector + &VectorViewMut
        let v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1 + &v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Reference addition for f64 scalar
        let v = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let result = &v + 0.5;
        assert!((result[0] - 1.5).abs() < f64::EPSILON);
        assert!((result[1] - 2.5).abs() < f64::EPSILON);
        assert!((result[2] - 3.5).abs() < f64::EPSILON);

        let result = v + 0.5;
        assert!((result[0] - 1.5).abs() < f64::EPSILON);
        assert!((result[1] - 2.5).abs() < f64::EPSILON);
        assert!((result[2] - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_vector_view_add() {
        let v1 = Vector::<i32, 5>::new([1, 2, 3, 4, 5]);
        let v2 = Vector::<i32, 5>::new([5, 4, 3, 2, 1]);
        let view1 = v1.view::<3>(1).unwrap();
        let view2 = v2.view::<3>(1).unwrap();

        // Test VectorView + VectorView
        let result = &view1 + &view2;
        assert_eq!(result, Vector::<i32, 3>::new([6, 6, 6]));

        // Test VectorView + &VectorView
        let result = view1 + &view2;
        assert_eq!(result, Vector::<i32, 3>::new([6, 6, 6]));

        // Test &VectorView + VectorView
        let view1 = v1.view::<3>(1).unwrap();
        let result = &view1 + view2;
        assert_eq!(result, Vector::<i32, 3>::new([6, 6, 6]));

        // Test VectorView + VectorView
        let view1 = v1.view::<3>(1).unwrap();
        let view2 = v2.view::<3>(1).unwrap();
        let result = view1 + view2;
        assert_eq!(result, Vector::<i32, 3>::new([6, 6, 6]));

        // Test VectorView + Vector
        let view1 = v1.view::<3>(1).unwrap();
        let result = view1 + Vector::<i32, 3>::new([1, 1, 1]);
        assert_eq!(result, Vector::<i32, 3>::new([3, 4, 5]));

        // Test VectorView + &Vector
        let vec = Vector::<i32, 3>::new([1, 1, 1]);
        let view1 = v1.view::<3>(1).unwrap();
        let result = view1 + &vec;
        assert_eq!(result, Vector::<i32, 3>::new([3, 4, 5]));

        // Test &VectorView + Vector
        let view1 = v1.view::<3>(1).unwrap();
        let result = &view1 + Vector::<i32, 3>::new([1, 1, 1]);
        assert_eq!(result, Vector::<i32, 3>::new([3, 4, 5]));

        // Test &VectorView + &Vector
        let view1 = v1.view::<3>(1).unwrap();
        let vec = Vector::<i32, 3>::new([1, 1, 1]);
        let result = &view1 + &vec;
        assert_eq!(result, Vector::<i32, 3>::new([3, 4, 5]));

        // Test VectorView + VectorViewMut
        let view1 = v1.view::<3>(1).unwrap();
        let mut v3 = Vector::<i32, 5>::new([10, 20, 30, 40, 50]);
        let view_mut = v3.view_mut::<3>(1).unwrap();
        let result = view1 + view_mut;
        assert_eq!(result, Vector::<i32, 3>::new([22, 33, 44]));

        // Test VectorView + &VectorViewMut
        let view1 = v1.view::<3>(1).unwrap();
        let view_mut = v3.view_mut::<3>(1).unwrap();
        let result = view1 + &view_mut;
        assert_eq!(result, Vector::<i32, 3>::new([22, 33, 44]));

        // Test &VectorView + VectorViewMut
        let view1 = v1.view::<3>(1).unwrap();
        let mut v3 = Vector::<i32, 5>::new([10, 20, 30, 40, 50]);
        let view_mut = v3.view_mut::<3>(1).unwrap();
        let result = &view1 + view_mut;
        assert_eq!(result, Vector::<i32, 3>::new([22, 33, 44]));

        // Test &VectorView + &VectorViewMut
        let view1 = v1.view::<3>(1).unwrap();
        let mut v4 = Vector::<i32, 5>::new([10, 20, 30, 40, 50]);
        let view_mut = v4.view_mut::<3>(1).unwrap();
        let result = &view1 + &view_mut;
        assert_eq!(result, Vector::<i32, 3>::new([22, 33, 44]));

        // Test scalar addition
        let view1 = v1.view::<3>(1).unwrap();
        let result = view1 + 10;
        assert_eq!(result, Vector::<i32, 3>::new([12, 13, 14]));

        let view1 = v1.view::<3>(1).unwrap();
        let result = &view1 + 10;
        assert_eq!(result, Vector::<i32, 3>::new([12, 13, 14]));
    }

    #[test]
    fn test_vector_view_mut_add() {
        // VectorViewMut + VectorViewMut
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Reference addition for f64
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + &v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // Mixed reference and non-reference addition for f64
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + &v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let mut v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + v2.view_mut::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // VectorViewMut + VectorView
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &VectorViewMut + VectorView
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // VectorViewMut + &VectorView
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + &v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &VectorViewMut + &VectorView
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + &v2.view::<3>(0).unwrap();
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // VectorViewMut + Vector
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &VectorViewMut + Vector
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // VectorViewMut + &Vector
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = v1.view_mut::<3>(0).unwrap() + &v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // &VectorViewMut + &Vector
        let mut v1 = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let v2 = Vector::<f64, 3>::new([0.1, 0.2, 0.3]);
        let result = &v1.view_mut::<3>(0).unwrap() + &v2;
        assert!((result[0] - 1.1).abs() < f64::EPSILON);
        assert!((result[1] - 2.2).abs() < f64::EPSILON);
        assert!((result[2] - 3.3).abs() < f64::EPSILON);

        // VectorViewMut + Scalar
        let mut v = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let result = v.view_mut::<3>(0).unwrap() + 0.5;
        assert!((result[0] - 1.5).abs() < f64::EPSILON);
        assert!((result[1] - 2.5).abs() < f64::EPSILON);
        assert!((result[2] - 3.5).abs() < f64::EPSILON);

        // &VectorViewMut + Scalar
        let mut v = Vector::<f64, 3>::new([1.0, 2.0, 3.0]);
        let result = &v.view_mut::<3>(0).unwrap() + 0.5;
        assert!((result[0] - 1.5).abs() < f64::EPSILON);
        assert!((result[1] - 2.5).abs() < f64::EPSILON);
        assert!((result[2] - 3.5).abs() < f64::EPSILON);
    }
}
