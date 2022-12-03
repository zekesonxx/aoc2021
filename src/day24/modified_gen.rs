
fn digit(output: &mut isize, input: isize, a: isize, b: isize, dodiv: bool) {
    let w=input;
    let x = *output%26;
    println!();
    println!("output={}", output);
    println!("numbers that wouldn't block: {:?}", (1..=9).filter(|w| *w==x+a).collect::<Vec<isize>>());

    if dodiv {
        *output /= 26;
        println!("output/26={}", output);
    }
    if w != x+a {
        println!("in block ({}in != {} ({}mod+{}const))", w, x+a, x, a);
        println!("output = {}*26", output);
        println!("output += {}", w);
        println!("output += {}", b);
        *output *= 26;
        *output += w+b; 
    }
    println!("output={}", output);
}

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn alu_generated_program(input : & [isize]) -> (isize, isize, isize, isize) {
assert_eq!(input.len(), 14, "invalid number of input digits");
let mut output = 0isize ;

digit(&mut output, input[0],  13,  6, false);
digit(&mut output, input[1],  15,  7, false);
digit(&mut output, input[2],  15, 10, false);
digit(&mut output, input[3],  11,  2, false);
digit(&mut output, input[4],  -7, 15, true);
digit(&mut output, input[5],  10,  8, false);
digit(&mut output, input[6],  10,  1, false);
digit(&mut output, input[7],  -5, 10, true);
digit(&mut output, input[8],  15,  5, false);
digit(&mut output, input[9],  -3,  3, true);
digit(&mut output, input[10] , 0,  5, true);
digit(&mut output, input[11], -5, 11, true);
digit(&mut output, input[12], -9, 12, true);
digit(&mut output, input[13],  0, 10, true);

(0, 0, 0, output)
}
