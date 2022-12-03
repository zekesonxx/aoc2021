use super::optimizedop::*;
use super::primitives::*;

pub fn generate_rust(program: &[Op], optimize: bool) -> String {
    let mut output = String::new();
    output += stringify! {
        #[allow(dead_code)]
        #[allow(unused_mut)]
        #[allow(unused_assignments)]
        #[allow(unused_variables)]
        pub fn alu_generated_program(input: &[isize]) -> (isize, isize, isize, isize)
    };
    output += " {\n";

    // calculate how long the input should be
    let inputs = program.iter().filter(|x| (**x).is_inp()).count();
    output += &format!("assert_eq!(input.len(), {}, \"invalid number of input digits\");", inputs);
    output += stringify!{
        let mut w = 0isize;
        let mut x = 0isize;
        let mut y = 0isize;
        let mut z = 0isize;
    };
    output += "\n\n";

    let mut program: Vec<OptimizedOp> = program.iter().map(|op| (*op).into()).collect();
    let before = program.len();
    if optimize {
        for _ in 1..15 { program.optimize(); }
    }
    let after = program.len();
    println!("{}->{}", before, after);

    let mut i = 0usize;
    for (_pos, op) in program.iter().enumerate() {
        if let OptimizedOp::Inp(_) = op {
            output += "\n";
        }
        output += &op.to_rust_src(&mut i);
        output += "\n";
    }

    output += "\n";
    output += stringify! {
        (w,x,y,z)
    };
    output += "\n}\n";
    output
}
