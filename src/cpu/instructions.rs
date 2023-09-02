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
    SUBi(u8),               // sub A,n
    SBCr(ArithmeticTarget), // sbc A,r
    SBCi(u8),               // sbc A,n
    ANDr(ArithmeticTarget), // and A,r
    ANDi(u8),               // and A,n
    XORr(ArithmeticTarget), // xor A,r
    XORi(u8),               // xor A,n
    ORr(ArithmeticTarget),  // or  A,r
    ORi(u8),                // or  A,n
}
