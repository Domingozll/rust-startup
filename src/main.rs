fn main() {
    // println!("Hello, world!");
    
    //creation
    // 32位整型 i32
    // 16位整型 i16
    // let a: i32  = 5;
    let a: i16 = 5;
    // let a: i16 = 5.0 //加了类型后编译器会做类型校验
    let c = 5.0; //没加类型编译器会自动判断类型
    
    //mutability
    // a = 10; //rust中的变量默认都是常量，需要加 mut关键字才能修改
    let mut b = 10;
    b = 20;

    // shadowing
    let c: i32 = 10;
    let c: i64 = 20; //c变量名会覆盖前面同名变量的值
    println!("c is: {c}");//c is: 20

    /*
    * mutability是对同一个变量多次赋值
    * shadowing是新变量覆盖旧变量的值
    */

    //scope 作用域
    {
        let d = 30;
        println!("d is {d}");
    }
    // println!("d is {d}");// d不可见

    //数据类型
    //bool
    let b1: bool = true;

    // unsigned integers 无符号整数
    let i1: u8 = 1;
    let i2: u16 = 2;
    let i3: u32 = 3;
    let i4: u64 = 4;
    let i5: u128 = 5;
}
