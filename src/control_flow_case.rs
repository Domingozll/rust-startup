fn main() {
    // if/else
    let a: i32 = 3;
    if a > 5 { // if条件值必须为bool值（同Java）
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }

    let b = if a > 3 { 1 } else { -1 };//类似java中三元表达式,所有分支条件中的类型必须相同
    println!("b is:{}", b);

    //loop

    // loop {
    //     println!("loop forever");
    // }

    loop {
        println!("loop before break;");
        break;
    }

    loop {
        loop {
            println!("inner loop before break;");
            break; // 只结束内层循环
        }
        println!("outer loop before break;");
        break;
    }

    'outer : loop {
        loop {
            println!("inner loop before break;");
            break 'outer; // 指定结束某一层循环
        }
        println!("outer loop before break;");
        break;
    }

    //带返回值loop
    let x: i32 = loop {
        break 5;
    };

    let mut c: i32 = 0;
    while c < 5 {
        println!("c is :{}",c);
        c = c + 1
    }

    let d: [i32; 5] = [1,2,3,4,5];
    for i in d {
        println!("i is:{}",i);
    }
}
