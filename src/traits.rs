/// Enables retrieval and mapping over the first element of a pair
pub trait First<A, B, A2> {
    /// Return a reference to the first element of a pair
    fn fst(&self) -> A;

    /// Returns a new pair with the a function applied to the first element
    fn first<F>(&self, f: F) -> (A2, B)
    where
        F: FnOnce(A) -> A2;
}

/// Enables retrieval and mapping over the second element of a pair
pub trait Second<A, B, B2>: Sized {
    /// Return the second element of a pair
    fn snd(&self) -> B;

    /// Returns a new pair with the a function applied to the second element
    fn second<F>(&self, f: F) -> (A, B2)
    where
        F: FnOnce(B) -> B2;
}

/// Enables mapping over both elements of a pair at once
pub trait Bimap<A, A2, B, B2>: First<A, B, A2> + Second<A, B, B2> {
    fn bimap<F, G>(&self, f: F, g: G) -> (A2, B2)
    where
        F: FnOnce(A) -> A2,
        G: FnOnce(B) -> B2,
    {
        (f(self.fst()), g(self.snd()))
    }
}
