//use std::mem;
fn main() {
    print!("hello world !")
    //operators();
    //core_data_type();

}

/*
fn operators(){
    // arithmetic
    let mut a = 2 * 3 + 1 ;
    println!("{}", a);
    a = a + 1 ;
    a *= 2 ; // -=, +=, *=, /=, %=
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    // x:i32^3
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5 ;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise // |=OR, &=AND, ^=XOR, !=NOR
                            // [0|1 = 1], [1|1 = 1], [1|0 = 1], [0|0= 1]
    let c = 1 | 2 ;  // 01 | 10 = 11(3)
    let c_c_1 = 2 | 2 ; // 10 | 10 = 10(2)
    let c_c_2 = 1 | 3 ; // 01 | 11 = 11(3)
    println!("1|2={} , 2|2={}, 1|3={}", c, c_c_1, c_c_2);
                       // &     // [1&1 = 1], [0&1 = 0], [0&0 = 0], [1&0= 0]
    let c_d_1 = 1 & 2;// 01 & 10 = 00(0)
    let c_d_2 = 2 & 2; // 10 & 10 = 10(2)
    let c_d_3 = 2 & 3; // 10 & 11 = 10(2)
    let c_d_4 = 3 & 1; // 11 & 01 = 01(1)
    println!("1&2={} , 2&2={}, 2&3={}, 3&1={}", c_d_1, c_d_2, c_d_3,c_d_4);

                       // ^    // [0^0 = 0],[1^1=0], [1^0=1], [0^1=1]
    let c_3 = 1 ^ 2;// 01 ^ 10 = 11(3)
    let c_4 = 1 ^ 3; // 01 ^ 11 = 10(2)
    let c_4 = 3 ^ 3; // 11 ^ 11 = 00(0)
    println!("1^2={}, 1^2={}, 3^3={}", c_3, c_4, c_4);

    // << >> << >>
    let two_to_10 = 1 << 10 ; // >>
    println!("2^10 = {}", two_to_10);

    let pi_less_4 = std::f64::consts::PI <= 4.0; //true
    // > >= <= < ==
    let x = 5;
    let y = 10/2;

    println!("x equals y ? {}" , x==y);
}

fn core_data_type() {
    // unsigned u8: 0..255  //8bits
    let a:u8 = 255;
    println!("a is :{}", a);

    // unsigned u16: 0.. 255 * 255
    let b:u16 = 255;
    println!("b :{}", b);

    let mut c:u8 = 21; //mutable
    println!("c is {}", c);

    c = 43;
    println!("c is {}", c);

    let mut d = 123456789; // 32-bits signed i32
    println!("d is = {}", d);
    d = 99999;
    println!("d is {}", d);

    // 1 bytes = 8-bits =0.. 2^8 = [0..255](256) // u8
    // 2 bytes = 16-bits = 0.. 2^16 = 256 * 256 = [0..=65535](65536) // u16
    // 4 bytes = 32-bits = 0.. 2^32 = 65536 * 65536 = [0..=4294967295](4294967296) // u32 default
    // 8 bytes = 64-bits = 0.. 2^64 = 4294967296 * 4294967296  // u64
    // 16 bytes = 128-bits = 0.. 2^128  // u28

    // 1 bytes = 8-bits = -2^(8-1).. +2^(8-1)-1 = [-128..=127](256) // i8
    // 2 bytes = 16-bits = -2^(16-1).. +2^(16-1)-1 = [-32768..=32767](65536) // i16
    // .... N bytes = N * 8-bits = -2^(N-1).. +2&(N-1)-1 = |X| + Y  - 1
    let i1:i8 = 127;
    println!("i1 is {}, size = {} bytes", i1, mem::size_of_val(&i1));
    // 2 bytes = 16-bits = 2^16 = 256 * 256 = [0..=65535](65536) // u16
    // 4 bytes = 32-bits = 2^32 = 65536 * 65536 = [0..=4294967295](4294967296) // u32 default
    // 8 bytes = 64-bits = 2^64 = 4294967296 * 4294967296  // u64
    // 16 bytes = 128-bits = 2^128  // u28

    let f:u16 = 65535;
    println!("f is {}, size = {} bytes", f, mem::size_of_val(&f));

    // usize && isize
    let z:isize = 123; //isize/usize based on Current System
    let z_size = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bits OS",
             z, z_size, z_size * 8);

    let z2:usize = 333;
    let z2_size = mem::size_of_val(&z2);
    println!("z2 = {}, takes up {} bytes, {}-bits OS",
             z2, z2_size, z2_size * 8);

    // char
    let d = 'P'; //char default 4 bytes 32-bits
    println!("d is {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.665; //8bytes or 64-bits
    println!("e is {}, size = {} bytes", e, mem::size_of_val(&e));

    let e2:f32 = 2.44444; // 4 bytes or 32-bits
    println!("e2 is {}, size = {} bytes", e2, mem::size_of_val(&e2));

    // boolean
    let h = true; // 1 bytes, 8-bits
    println!("h is {}, size = {} bytes", h, mem::size_of_val(&h));

    let v2 = 4>1; //true
    println!("v2 is {}, size = {} bytes", v2, mem::size_of_val(&v2));
}

*/