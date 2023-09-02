pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum ArithmeticTarget16 {
    BC,
    DE,
    HL,
    SP,
}

pub enum Instruction {
    /* 8-bit Load instructions */
    LDrr(ArithmeticTarget, ArithmeticTarget), // LD r,r'
    LDri(ArithmeticTarget, u8),               // LD r,i

    /* 8-bit Arithmetic/Logic instructions */
    ADDr(ArithmeticTarget), // add A,r
    ADDi(u8),               // add A,i
    ADCr(ArithmeticTarget), // adc A,r
    ADCi(u8),               // adc A,i
    SUBr(ArithmeticTarget), // sub A,r
    SUBi(u8),               // sub A,i
    SBCr(ArithmeticTarget), // sbc A,r
    SBCi(u8),               // sbc A,i
    ANDr(ArithmeticTarget), // and A,r
    ANDi(u8),               // and A,i
    XORr(ArithmeticTarget), // xor A,r
    XORi(u8),               // xor A,i
    ORr(ArithmeticTarget),  // or  A,r
    ORi(u8),                // or  A,i
    CPr(ArithmeticTarget),  // cp  A,r
    CPi(u8),                // cp  A,i
    INCr(ArithmeticTarget), // inc r
    DECr(ArithmeticTarget), // dec r
    CPL,                    // cpl

    /* 16-bit Arithmetic/Logic instructions */
    ADDHLRR(ArithmeticTarget16), // add HL, rr

    /* CPU Control instructions */
    SCF,  // scf
    CCF,  // ccf
    NOP,  // nop
    HALT, // halt
}
