#[rustfmt::skip]
fn f() {
    use std::fs;

    ::std::fs::create_dir("a/b");
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `::std::fs::create_dir` instead of `std::fs::create_dir_all`
    std::fs::create_dir("a/b");
 // ^^^^^^^^^^^^^^^^^^^^^^^^^^ Using `std::fs::create_dir` instead of `std::fs::create_dir_all`
    fs::create_dir("a/b");
 // ^^^^^^^^^^^^^^^^^^^^^ Using `fs::create_dir` instead of `std::fs::create_dir_all`
}

#[rustfmt::skip]
mod qualified {
    fn f() {
        use std::fs::create_dir;
        create_dir("a/b");
     // ^^^^^^^^^^^^^^^^^ Using `create_dir` instead of `std::fs::create_dir_all`
    }
    fn g() {
        use std::fs::create_dir as my_create_dir;
        my_create_dir("a/b");
     // ^^^^^^^^^^^^^^^^^^^^ Using `my_create_dir` instead of `std::fs::create_dir_all`
    }
}

mod custom_no_match {
    fn create_dir(s: &str) {}

    fn no_match() {
        create_dir("a/b");
    }
}
