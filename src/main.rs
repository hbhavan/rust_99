use crate::problems::p01::P01;
use crate::problems::p02::P02;
use crate::problems::p03::P03;
use crate::problems::p04::P04;
use crate::problems::p05::P05;
use crate::problems::p06::P06;
use crate::problems::p07::P07;
use crate::problems::p08::P08;
use crate::problems::p09::P09;
use crate::problems::p10::P10;
use crate::problems::p11::P11;
use crate::problems::p12::P12;
use crate::problems::test;

mod problems;
mod util;

fn main() {
    let mut results = Vec::new();

    results.extend(test::<P01>());
    results.extend(test::<P02>());
    results.extend(test::<P03>());
    results.extend(test::<P04>());
    results.extend(test::<P05>());
    results.extend(test::<P06>());
    results.extend(test::<P07>());
    results.extend(test::<P08>());
    results.extend(test::<P09>());
    results.extend(test::<P10>());
    results.extend(test::<P11>());
    results.extend(test::<P12>());

    for result in results {
        println!("{}", result)
    }
}
