#[rustfmt::skip]
fn f() {
    let v = [1, 2, 3];
    v.iter().filter(|&x| *x == 0).next();
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Called `.filter(..).next()` when `.find(..)` would suffice

    let i = IteratorFalsePositive { x: 0u32 };
    i.filter(42).next();
}

struct IteratorFalsePositive {
    x: u32,
}

impl IteratorFalsePositive {
    fn filter(&self, filter_fn: u32) -> &Self {
        self
    }
    fn next(&self) -> &Self {
        self
    }
}
