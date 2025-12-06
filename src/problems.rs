pub mod p01;
pub mod p02;
pub mod p03;
pub mod p04;
pub mod p05;
pub mod p06;
pub mod p07;
pub mod p08;
pub mod p09;
pub mod p10;
pub mod p11;
pub mod p12;
//pub mod p13;
//pub mod p14;
//pub mod p15;
//pub mod p16;
//pub mod p17;
//pub mod p18;
//pub mod p19;
//pub mod p20;
//pub mod p21;
//pub mod p22;
//pub mod p23;
//pub mod p24;
//pub mod p25;
//pub mod p26;
//pub mod p27;
//pub mod p28;
//pub mod p29;
//pub mod p30;
//pub mod p31;
//pub mod p32;
//pub mod p33;
//pub mod p34;
//pub mod p35;
//pub mod p36;
//pub mod p37;
//pub mod p38;
//pub mod p39;
//pub mod p40;
//pub mod p41;
//pub mod p42;
//pub mod p43;
//pub mod p44;
//pub mod p45;
//pub mod p46;
//pub mod p47;
//pub mod p48;
//pub mod p49;
//pub mod p50;
//pub mod p51;
//pub mod p52;
//pub mod p53;
//pub mod p54;
//pub mod p55;
//pub mod p56;
//pub mod p57;
//pub mod p58;
//pub mod p59;
//pub mod p60;
//pub mod p61;
//pub mod p62;
//pub mod p63;
//pub mod p64;
//pub mod p65;
//pub mod p66;
//pub mod p67;
//pub mod p68;
//pub mod p69;
//pub mod p70;
//pub mod p71;
//pub mod p72;
//pub mod p73;
//pub mod p74;
//pub mod p75;
//pub mod p76;
//pub mod p77;
//pub mod p78;
//pub mod p79;
//pub mod p80;
//pub mod p81;
//pub mod p82;
//pub mod p83;
//pub mod p84;
//pub mod p85;
//pub mod p86;
//pub mod p87;
//pub mod p88;
//pub mod p89;
//pub mod p90;
//pub mod p91;
//pub mod p92;
//pub mod p93;
//pub mod p94;
//pub mod p95;
//pub mod p96;
//pub mod p97;
//pub mod p98;
//pub mod p99;

pub trait Solution {
    type Input;
    type Output;

    fn test_name() -> String {
        String::from("Test")
    }

    fn get_inputs() -> [Self::Input; 3];

    fn get_outputs() -> [Self::Output; 3];

    fn execute(input: &Self::Input) -> Self::Output;

    fn assert(expected: Self::Output, result: Self::Output) -> bool;
}

pub fn test<S: Solution>() -> Vec<String> {
    let inputs = S::get_inputs();
    let outputs = S::get_outputs();

    let result = inputs
        .iter()
        .zip(outputs)
        .enumerate()
        .map(|(index, (input, expected))| {
            let result = S::execute(input);
            let test_case = format!("{name} - {i}", name = S::test_name(), i = index + 1);

            match S::assert(expected, result) {
                true => format!("{test_case} PASSED"),
                false => format!("{test_case} FAILED"),
            }
        })
        .collect();

    result
}
