fn main() {
    let y: i32 = my_function(1,2);
    println!("my function returned: {}",y);
}

fn my_function(x: i32,y: i64) -> i32 { // 命名规则：小写加下划线(snake case)
    // return 5; //提前return，return后面的代码不会执行
    println!("my function called with:{},{}",x,y);
    let z: i32 = 10;
    z
}
