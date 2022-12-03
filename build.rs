use std::fs;
mod day24 {
    #![allow(dead_code)]
    #![allow(unused_mut)]
    #![allow(unused_assignments)]
    pub mod primitives {
        include!("./src/day24/primitives.rs");
    }
    pub mod optimizedop {
        include!("./src/day24/optimizedop.rs");
    }
    pub mod rustgen {
        include!("./src/day24/rustgen.rs");
    }
}

const MONAD_SOURCE: &str = include_str!("./input/2021/day24.txt");

fn main() {
    let program = day24::primitives::parse_alu_program(MONAD_SOURCE);
    fs::write("./src/day24/alu_generated.rs", day24::rustgen::generate_rust(&program, true)).unwrap();
    fs::write("./src/day24/alu_generated_unoptimized.rs", day24::rustgen::generate_rust(&program, false)).unwrap();

}