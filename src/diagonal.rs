use ndarray::*;

use super::convert::*;
use super::operator::*;

pub struct Diagonal<S: Data> {
    diag: ArrayBase<S, Ix1>,
}

pub trait IntoDiagonal<S: Data> {
    fn into_diagonal(self) -> Diagonal<S>;
}

pub trait AsDiagonal<'a, A> {
    fn as_diagonal(&self) -> Diagonal<ViewRepr<&'a A>>;
}

impl<S: Data> IntoDiagonal<S> for ArrayBase<S, Ix1> {
    fn into_diagonal(self) -> Diagonal<S> {
        Diagonal { diag: self }
    }
}

impl<'a, A, S: Data<Elem = A>> AsDiagonal<'a, A> for &'a ArrayBase<S, Ix1> {
    fn as_diagonal(&self) -> Diagonal<ViewRepr<&'a A>> {
        Diagonal { diag: self.view() }
    }
}

impl<'a, A, S, Sr> Operator<&'a mut ArrayBase<Sr, Ix1>, &'a mut ArrayBase<Sr, Ix1>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Sr: DataMut<Elem = A>,
{
    fn op(&self, mut a: &'a mut ArrayBase<Sr, Ix1>) -> &'a mut ArrayBase<Sr, Ix1> {
        for (val, d) in a.iter_mut().zip(self.diag.iter()) {
            *val = *val * *d;
        }
        a
    }
}

impl<'a, A, S, Si, So> Operator<&'a ArrayBase<Si, Ix1>, ArrayBase<So, Ix1>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Si: Data<Elem = A>,
    So: DataOwned<Elem = A> + DataMut,
{
    fn op(&self, a: &'a ArrayBase<Si, Ix1>) -> ArrayBase<So, Ix1> {
        let mut a = replicate(a);
        self.op(&mut a);
        a
    }
}

impl<A, S, Sr> Operator<ArrayBase<Sr, Ix1>, ArrayBase<Sr, Ix1>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Sr: DataOwned<Elem = A> + DataMut,
{
    fn op(&self, mut a: ArrayBase<Sr, Ix1>) -> ArrayBase<Sr, Ix1> {
        self.op(&mut a);
        a
    }
}

impl<'a, A, S, Sr> Operator<&'a mut ArrayBase<Sr, Ix2>, &'a mut ArrayBase<Sr, Ix2>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Sr: DataMut<Elem = A>,
{
    fn op(&self, mut a: &'a mut ArrayBase<Sr, Ix2>) -> &'a mut ArrayBase<Sr, Ix2> {
        let ref d = self.diag;
        for ((i, _), val) in a.indexed_iter_mut() {
            *val = *val * d[i];
        }
        a
    }
}

impl<'a, A, S, Si, So> Operator<&'a ArrayBase<Si, Ix2>, ArrayBase<So, Ix2>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Si: Data<Elem = A>,
    So: DataOwned<Elem = A> + DataMut,
{
    fn op(&self, a: &'a ArrayBase<Si, Ix2>) -> ArrayBase<So, Ix2> {
        let mut a = replicate(a);
        self.op(&mut a);
        a
    }
}

impl<A, S, Sr> Operator<ArrayBase<Sr, Ix2>, ArrayBase<Sr, Ix2>> for Diagonal<S>
where
    A: LinalgScalar,
    S: Data<Elem = A>,
    Sr: DataOwned<Elem = A> + DataMut,
{
    fn op(&self, mut a: ArrayBase<Sr, Ix2>) -> ArrayBase<Sr, Ix2> {
        self.op(&mut a);
        a
    }
}
