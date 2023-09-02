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
}
