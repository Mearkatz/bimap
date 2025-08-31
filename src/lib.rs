use traits::Bimap;

pub mod traits;

impl<A, A2, B, B2, F, G> Bimap<A, A2, B, B2, F, G> for (A, B)
where
    F: FnOnce(A) -> A2,
    G: FnOnce(B) -> B2,
{
    fn first(self, f: F) -> (A2, B) {
        let (a, b) = self;
        (f(a), b)
    }

    fn second(self, g: G) -> (A, B2) {
        let (a, b) = self;
        (a, g(b))
    }

    fn bimap(self, f: F, g: G) -> (A2, B2) {
        let (a, b) = self;
        (f(a), g(b))
    }
}
