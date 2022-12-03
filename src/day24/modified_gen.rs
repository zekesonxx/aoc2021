
fn digit(output: &mut isize, input: isize, a: isize, b: isize, dodiv: bool) {
    let w=input;
    let x = *output%26;
    *output /= 26;
    if w != x+a {
        *output *= 26;
        *output += w+b;
    }
}

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn alu_generated_program(input : & [isize]) -> (isize, isize, isize, isize) {
assert_eq!(input.len(), 14, "invalid number of input digits");
let mut output = 0isize ;


// let mut w=input[0];
// if w != (output%26)+13 {
//     output *= 26;
//     output += w+6;
// }
digit(&mut output, input[0], 13, 6, false);
//output = input[0]+6;

let mut w=input[1];
if w != (output%26)+15 {
    output *= 26;
    output += w+7;
}

let mut w=input[2];
if w != (output%26)+15 {
    output *= 26;
    output += w+10;
}

let mut w=input[3];
if w != (output%26)+11 {
    output *= 26;
    output += w+2;
}

let mut w=input[4];
let x = output%26;
output /= 26;
if w != x-7 {
    output *= 26;
    output += w+15;
}

let mut w=input[5];
if w != (output%26)+10 {
    output *= 26;
    output += w+8;
}

let mut w=input[6];
if w != (output%26)+10 {
    output *= 26;
    output += w+1;
}

let mut w=input[7];
let x = output%26;
output /= 26;
if w != x-5 {
    output *= 26;
    output += w+10;
}

let mut w=input[8];
if w != (output%26)+15 {
    output *= 26;
    output += w+5;
}

let mut w=input[9];
let x = output%26;
output /= 26;
if w != x-3 {
    output *= 26;
    output += w+3;
}

let mut w=input[10];
let x = output%26;
output /= 26;
if w != x {
    output *= 26;
    output += w+5;
}

let mut w=input[11];
let x = output%26;
output /= 26;
if w != x-5 {
    output *= 26;
    output += w+11;
}

let mut w=input[12];
let x = output%26;
output /= 26;
if w != x-9 {
    output *= 26;
    output += w+12;
}

let mut w=input[13];
let x = output%26;
output /= 26;
if w != x {
    output *= 26;
    output += w+10;
}

(0, 0, 0, output)
}
