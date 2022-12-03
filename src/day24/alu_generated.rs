#[allow(dead_code)] #[allow(unused_mut)] #[allow(unused_assignments)]
#[allow(unused_variables)] pub fn alu_generated_program(input : & [isize]) ->
(isize, isize, isize, isize) {
assert_eq!(input.len(), 14, "invalid number of input digits");let mut w = 0isize ; let mut x = 0isize ; let mut y = 0isize ; let mut z =
0isize ;


let mut w=input[0];
let mut x=z%26;
x+=13;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+6;
y*=x;
z+=y;

let mut w=input[1];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+7;
y*=x;
z+=y;

let mut w=input[2];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;

let mut w=input[3];
let mut x=z%26;
x+=11;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+2;
y*=x;
z+=y;

let mut w=input[4];
let mut x=z%26;
z/=26;
x+=-7;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+15;
y*=x;
z+=y;

let mut w=input[5];
let mut x=z%26;
x+=10;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+8;
y*=x;
z+=y;

let mut w=input[6];
let mut x=z%26;
x+=10;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+1;
y*=x;
z+=y;

let mut w=input[7];
let mut x=z%26;
z/=26;
x+=-5;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;

let mut w=input[8];
let mut x=z%26;
x+=15;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+5;
y*=x;
z+=y;

let mut w=input[9];
let mut x=z%26;
z/=26;
x+=-3;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+3;
y*=x;
z+=y;

let mut w=input[10];
let mut x=z%26;
z/=26;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+5;
y*=x;
z+=y;

let mut w=input[11];
let mut x=z%26;
z/=26;
x+=-5;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+11;
y*=x;
z+=y;

let mut w=input[12];
let mut x=z%26;
z/=26;
x+=-9;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+12;
y*=x;
z+=y;

let mut w=input[13];
let mut x=z%26;
z/=26;
x=if x==w {0}else{1};
let mut y=x*25;
y+=1;
z*=y;
let mut y=w+10;
y*=x;
z+=y;

(w, x, y, z)
}
