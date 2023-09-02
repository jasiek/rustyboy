pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Instruction {
    ADDr(ArithmeticTarget), // add A,r
    ADDi(u8),               // add A,n
    ADCr(ArithmeticTarget), // adc A,r
    ADCi(u8),               // adc A,n
    SUBr(ArithmeticTarget), // sub A,r
    SUBi(u8),               // sub A,i
}
