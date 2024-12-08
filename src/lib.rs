use std::ops::Mul;

use traits::{Bimap, First, Second};

pub mod traits;

impl<A, B, A2> First<A, B, A2> for (A, B)
where
    A: Clone,
    B: Clone,
{
    fn fst(&self) -> A {
        self.0.clone()
    }

    fn first<F>(&self, f: F) -> (A2, B)
    where
        F: FnOnce(A) -> A2,
    {
        (f(self.0.clone()), self.1.clone())
    }
}

impl<A, B, B2> Second<A, B, B2> for (A, B)
where
    A: Clone,
    B: Clone,
{
    fn snd(&self) -> B {
        self.1.clone()
    }

    fn second<F>(&self, f: F) -> (A, B2)
    where
        F: FnOnce(B) -> B2,
    {
        (self.0.clone(), f(self.1.clone()))
    }
}

impl<A, A2, B, B2> Bimap<A, A2, B, B2> for (A, B) where Self: First<A, B, A2> + Second<A, B, B2> {}

fn square_then_multiply<T>(t: &(T, T)) -> T
where
    T: Clone + Mul<Output = T>,
{
    let square = |x: T| -> T { x.clone() * x };
    let (a, b) = t.bimap(square, square);
    a * b
}
