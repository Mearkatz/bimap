/// Enables mapping over both elements of a pair at once
pub trait Bimap<A, A2, B, B2, F, G>
where
    F: FnOnce(A) -> A2,
    G: FnOnce(B) -> B2,
{
    /// Returns `self` with `f` applied to its first element.
    fn first(self, f: F) -> (A2, B);

    /// Returns `self` with `g` applied to its second element.
    fn second(self, g: G) -> (A, B2);

    /// Returns `self` with `f` applied to its first element and `g` applied to its second element.
    fn bimap(self, f: F, g: G) -> (A2, B2);
}
