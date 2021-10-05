//! An example of using the `BasicMulti` noise function

extern crate noise;

use noise::{utils::*, BasicMulti, MultiFractal};

fn main() {
    PlaneMapBuilder::new(BasicMulti::default())
        .build()
        .write_to_file("basicmulti.png");
}
