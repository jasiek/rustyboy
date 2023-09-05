#[derive(Copy, Clone)]
pub enum ArithmeticTarget8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum ArithmeticTarget16 {
    BC,
    DE,
    HL,
    SP,
}

pub enum Instruction {
    /* 8-bit Load instructions */
    LDrr(ArithmeticTarget8, ArithmeticTarget8), // LD r,r'
    LDri(ArithmeticTarget8, u8),                // LD r,i

    /* 8-bit Arithmetic/Logic instructions */
    ADDr(ArithmeticTarget8), // add A,r
    ADDi(u8),                // add A,i
    ADCr(ArithmeticTarget8), // adc A,r
    ADCi(u8),                // adc A,i
    SUBr(ArithmeticTarget8), // sub A,r
    SUBi(u8),                // sub A,i
    SBCr(ArithmeticTarget8), // sbc A,r
    SBCi(u8),                // sbc A,i
    ANDr(ArithmeticTarget8), // and A,r
    ANDi(u8),                // and A,i
    XORr(ArithmeticTarget8), // xor A,r
    XORi(u8),                // xor A,i
    ORr(ArithmeticTarget8),  // or  A,r
    ORi(u8),                 // or  A,i
    CPr(ArithmeticTarget8),  // cp  A,r
    CPi(u8),                 // cp  A,i
    INCr(ArithmeticTarget8), // inc r
    DECr(ArithmeticTarget8), // dec r
    CPL,                     // cpl

    /* 16-bit Arithmetic/Logic instructions */
    ADDHLRR(ArithmeticTarget16), // add HL, rr
    INCRR(ArithmeticTarget16),   // inc rr
    DECRR(ArithmeticTarget16),   // dec rr

    /* Rotate and Shift instructions */
    RLCA,                     // rotate A left
    RLA,                      // rotate A left with carry
    RRCA,                     // rotate A right
    RRA,                      // rotate A right with carry
    RLCr(ArithmeticTarget8),  // rotate left
    RRCr(ArithmeticTarget8),  // rotate right
    RLr(ArithmeticTarget8),   // rotate left with carry
    RRr(ArithmeticTarget8),   // rotate right with carry
    SLAr(ArithmeticTarget8),  // shift left arithmetic
    SRAr(ArithmeticTarget8),  // shift right arithmetic
    SRLr(ArithmeticTarget8),  // shift right logical
    SWAPr(ArithmeticTarget8), // exchange low/hi nibble

    /* Single bit operations */
    BITnr(u8, ArithmeticTarget8), // test bit n of r
    SETnr(u8, ArithmeticTarget8), // set bit n of r
    RESnr(u8, ArithmeticTarget8), // unset bit n of r

    /* 16-bit load instructions */
    LDrrnn(ArithmeticTarget16, u16), // LD rr,nn
    LDSPHL(),                        // LD SP, HL
    PUSH(ArithmeticTarget16),        // PUSH rr
    POP(ArithmeticTarget16),         // POP rr

    /* CPU Control instructions */
    SCF,  // scf
    CCF,  // ccf
    NOP,  // nop
    HALT, // halt
}
