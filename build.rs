
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/doubler.c")
        .compile("libdoubler.a");
    cc::Build::new()
        .file("src/quicksort.c")
        .compile("libquicksort.a");
}