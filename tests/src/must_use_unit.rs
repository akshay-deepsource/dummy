#[rustfmt::skip]
mod must_use_unit {
    type T = ();

    #[must_use] fn a() {}
 // ^^^^^^^^^^^^^^^^^^^^^ Function that is annotated with `#[must_use] returns `()`

    #[must_use] pub fn b() {}
 // ^^^^^^^^^^^^^^^^^^^^^^^^^ Function that is annotated with `#[must_use] returns `()`

    #[must_use] pub fn c() -> T {}
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Function that is annotated with `#[must_use] returns `()`

    #[must_use] #[cfg(feature = "publish")] fn d() -> () {}
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Function that is annotated with `#[must_use] returns `()`

    #[must_use = "With note"] pub fn must_use_with_note() {}
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Function that is annotated with `#[must_use] returns `()`

    #[must_use] pub async fn yield_now() {}
}
