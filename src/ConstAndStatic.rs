
/**
 * constant (常量):
 * - 普通变量默认不可变，但可以通过mut关键字修改为可变变量，const常量无法被修改为可变变量。
 * - 命名按惯例使用大写字母加下划线间隔 (Screaming snake case)
 * - 必须显示指定类型
 * - 可以是包括全局作用域在内的任意作用域
 * - 会在编译时确定，必须使用字面值（区分运行时值）
 * - 直接内联进代码，不占用内存
 */
const MAX_PLAYERS: u8 = 10; 

/**
 * static variables （静态变量）:
 * - 命名按惯例使用大写字母加下划线间隔 (Screaming snake case)
 * - 必须显示指定类型
 * - 可以是包括全局作用域在内的任意作用域
 * - 可添加 mut 关键字变为可变变量 (访问mut静态变量存在线程安全，需做临界区管控)
 * - 分配在静态内存区
*/ 
static CASINO_NAME: &str = "Rusty Castino";
static mut CASINO_NAME_MUT: &str = "Rusty Castino";

fn main() {
    
    // 在编译时，代码中的MAX_PLAYERS会被直接替换为10
    let a: u8 = MAX_PLAYERS;
    let b: u8 = MAX_PLAYERS;

    let c: &str = CASINO_NAME;
    // let d: &str = CASINO_NAME_MUT; // use of mutable static is unsafe and requires unsafe block
}
