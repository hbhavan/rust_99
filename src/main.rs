use crate::problems::p1::P1;
use crate::problems::p2::P2;
use crate::problems::p3::P3;
use crate::problems::p4::P4;
use crate::problems::p5::P5;
use crate::problems::p6::P6;
use crate::problems::p7::P7;
use crate::problems::p8::P8;
use crate::problems::p9::P9;
use crate::problems::test;

mod problems;
mod util;

fn main() {
    let mut results = Vec::new();

    results.extend(test::<P1>());
    results.extend(test::<P2>());
    results.extend(test::<P3>());
    results.extend(test::<P4>());
    results.extend(test::<P5>());
    results.extend(test::<P6>());
    results.extend(test::<P7>());
    results.extend(test::<P8>());
    results.extend(test::<P9>());

    for result in results {
        println!("{}", result)
    }
}
