fn main() {
    let y = 2;
    let x @ _ = --(*&y);
    let x @ _ = --(*&y);
}
