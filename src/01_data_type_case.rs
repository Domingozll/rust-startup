fn main() {
    //标量数据类型
    //bool
    let b1: bool = true;

    // unsigned integers 无符号整数
    let i1: u8 = 1;
    let i2: u16 = 2;
    let i3: u32 = 3;
    let i4: u64 = 4;
    let i5: u128 = 5;

    // signed integers 有符号整数
    let i1: i8 = 1;
    let i2: i16 = -2;
    let i3: i32 = 3;
    let i4: i64 = -4;
    let i5: i128 = 5;

    println!("i4 is {i4}"); // i4 is -4

    //floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1; // usize：指针大小的整数，用于存储内存地址或数组索引等
    let p2: isize = 1; // isize：指针大小的整数，用于存储内存地址或数组索引等

    // characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    //复合数据类型
    //arrays （只能存储通类型数据）
    let a1: [i32; 5] = [1,2,3,4,5];
    let i1: i32 = a1[2];
    println!("i1 is:{i1}"); // i1 is:3

    // tuples (元组，可存储不同类型数据)
    let t1: (i32, i32, i32) = (1,2,3);
    let t1: (i32, f64, &str) = (5,5.0,"05");
    let s1: &str = t1.2;
    println!("s1 is:{s1}");// s1 is:05
    let (i1, f1, s1) = t1;
    println!("i1 is:{i1},f1 is:{f1},s1 is:{s1}");//i1 is:5,f1 is:5,s1 is:05

    // empty tuples,所有函数默认的返回类型
    let uint:() = ();

    // type aliasing. a new name for a existing type（类型别名,可增强代码可读性）
    type age = u8;
    let a1: age = 57;
    println!("a1 is {}", a1);

}
