/*
fn main() {
    let x = 'c';        // 字符char型，只能用单引号
    let y = "string";   // 字符串，必须用双引号
    let z = 25;

    println!("x={}, y={}, z={}", x, y, z);
}
*/

/*
fn main() {
    let x = 6;
    x = 10;
    println!("x is {}", x);
}
*/

/*
fn main() {
    let mut x = 6; // 用mut来显式声明为可变的变量
    println!("x is {}", x); // // 打印 x，至少使用一次，否则 x=6 的赋值是没有意义的。
    x = 10;
    println!("x is {}", x);
}
*/

/*
fn main() {
    let x = 5;
    let x = 't';
    let x = 99.99;
    println!("x is {}", x);
}
*/

/*
fn main() {
    const max_value:i32  = 100_000;
}
*/

/*
fn main(){
    let x = 5;         // default i32
    let b = 99.9;      // default f64
    let y: i8 = -128;  // i8 范围 -128 ~ 127 
    let z :u8 = 255;   // u8 范围 0 ~ 255 

    println!("x is {}", x);
    println!("b is {}", b);
    println!("y is {}", y);
    println!("z is {}", z);
}
*/
/*
fn main() {
    let y: i8 = -129;  // i8 范围 -128 ~ 127 

    println!("y is {}", y);
}
*/
/*
fn main() {
    let x:f32 = -99.9911123;  //单精度，有效位7位，小数点最多6位
    let y :f64 = 9.99999999999911123; // 双精度浮点型，有效位16位，小数点最多15位
    println!("x is {}", x);
    println!("y is {}", y);
}
*/
/*
fn main(){
    let t = true;
    let k:bool = false;

    println!("t is {}", t);
    println!("k is {}", k);
}
*/
/*
fn main() {
    let c = 'z'; // 字母
    let z = '中'; // 中文字符
    let cat = '😻'; // emoji 

    println!("c is {}", c);
    println!("z is {}", z);
    println!("cat is {}", cat);
}
*/
/*
fn main(){
   let sum = 5 + 10;             // 加法
   let difference = 95.5 - 4.3;  // 减法
   let product = 4 * 30;         // 乘法
   let quotient = 56.7 / 32.2;   // 除法
   let remainder = 43 % 5;       // 取余

   println!("sum is {}", sum);
   println!("difference is {}", difference);
   println!("product is {}", product);
   println!("quotient is {}", quotient);
   println!("remainder is {}", remainder);
   println!("format output {}", format!(" {:.*}", 3, 10.11122233)); // 格式化输出
}
*/
/*
fn main(){
    let t:(i32, bool, char) = (10, false, 'a');
    println!("t is {}", t);
}
*/
/*
fn main(){
    let t:(i32, bool, char) = (10, false, 'a');
    println!("t is {:?}", t);
}
*/
/*
fn main(){
    let t:(i32, bool, char) = (10, false, 'a');
    println!("a is {}", t.0);
}
*/
/*
fn main() {
    let tup = (10, false, 'a');  // 省略了变量的类型声明
    println!("a is {}", tup.0);
}
*/
/*
fn main() {
    let tup = (10, false, 'a');
    let (x, y, z) = tup;
    println!("a is {}", x);

    let (x, y, z) = (10, false, 'a');
    println!("a is {}", x);
}
*/
/*
fn main() {
    let tup = (10, false, 'a');
    let (mut x, y, z) = tup;
    x = 5;
    println!("a is {}", x);
}
*/

/*
fn main(){
    let arr = [1,2,3,4];
    println!("arr[0] is {:?}", arr);
}
*/
/*
fn main(){
    let arr:[i32; 4] = [1,2,3,4];
    println!("arr[0] is {}", arr[2]);
}
*/
/*
fn main() {
    let arr: [i32; 4] = [1,2,3,4]; // 类型声明为 i32 ，长度为4 的数组
    arr[0] = 99;
    println!("arr[0] is {}", arr[0]);
}
*/

/*
fn main() {
    let mut arr: [i32; 4] = [1,2,3,4]; // 类型声明为 i32 ，长度为4 的数组
    arr[0] = 99;
    println!("arr[0] is {}", arr[0]);
}
*/

/*
fn main() {
    let x = 6;
    if x < 5 {
        println!("{} less then 5", x );
    } else {
        println!("{} greater then 5", x);
    }
}
*/
/*
fn main() {
    let num = 3;
    if num {
        println!("do something");
    }
}
*/
/*
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

/*
fn main() {
    let is_ok = true;
    let num = if is_ok {
        5
    } else {
        6
    }; // 因为是给变量赋值，别忘了这里的分号
    println!("num is {}", num);
}
*/
/*
fn main() {
    let is_ok = true;
    let num = if is_ok {
        5
    } else {
        99.11
    }; // 因为是给变量赋值，别忘了这里的分号
    println!("num is {}", num);
}
*/

/*
fn main() {
    let mut i = 0;
    loop {
        i = i+1;
        if i == 5 {
            break;
        } else {
            println!("i is {}", i);
        }
    }
}
*/
/*
fn main() {
    let mut i = 0; 
    let num = loop {
        i = i + 1;
        if i == 5 {
            break i + 1;
        } else {
            println!("i is {}", i);
        }
    }; // 因为是给变量赋值语句，别忘了分号

    println!("num is {}", num);
}*/

/*
fn main() {
    let mut i = 5;
    while i != 0 {
        println!("i is {}", i);
        i = i - 1;
    }
}
*/

/*
fn main() {
    let arr:[i32; 5] = [2,3,4,5,6];
    let mut i = 0;
    while i < arr.len() {
        println!("arr[{}] is {}", i, arr[i]);
        i = i + 1;
    }
}
*/
/*
fn main() {
    let arr:[i8; 6] = [9,8,7,6,5,4];
    for i in arr.iter() {
        println!("ele is {}", i);
    }
}
*/
/*
fn main() {
    let x = 1;
    {
        let y = 1;
        println!("x is {}", x); // 可以访问到 x 变量
    }
    println!("y is {}", y);    // 不能访问到 y 变量，y 变量出了作用域就会被 drop 销毁掉
}
*/
/*
fn main() {
    // 行注释

    /*
       块注释
    */

    // 多行注释
    //
    //
}
*/

/*
/// 将给定的数字加一
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
*/

/*
fn main() {
    // do something

    another_function();
}

fn another_function() {
    // do something
}
*/

/*
fn main() {
    let x = 6;
    
    get_number(x);
}

fn get_number(num:f32) {
    println!("num is {}", num);
}
*/

/*
fn main() {
    let tup:(i32, char, f32) = (1, 'a', 9.9);
    get_number(tup);
}

fn get_number(p:(i32,char,f32)) {
    println!("p is {}", p.0);
}
*/
/*
fn main() {
    let x = 10;
    let y = get_number(x);
    println!("y is {}", y);
}

fn get_number(num:i32) -> i32 {
    num
}
*/

/*
fn main() {
    let y = |num: i32|{
        num + 1
    };
}
*/
/*
fn main() {
    let y = |num: i32|{  //定义了一个闭包，只有一个参数的匿名函数
        num + 1
    };  // 语句不要忘记了分号

    println!("y is {}", y);
}
*/

/*
fn main() {
    let y = |num: i32|{  //定义了一个闭包，只有一个参数的匿名函数
        num + 1
    };  // 语句不要忘记了分号

    println!("y is {}", y(5));
}
*/

/*
fn main() {
    let x = 10;

    let y = |num| num + x ;
    
    println!("new num is {}", y(x));
}
*/
/*
fn main() {
    let x = 10;

    fn y(num:i32) -> i32{
        num + x 
    }
    
    println!("new num is {}", y(x));
}
*/

/*
fn main() {
    let x = 10;
    let y = move |num: i32|{  // 把 x 变量移动到闭包里
        num + x 
    } ; 
    println!("x is {}", x);
}
*/
/*
fn main() {
    let x = vec![1,2,3];
    let y = |num|{ 
        num == x 
    } ; 
    println!("x is {:?}", y(x) );
}
*/
/*
fn main() {
    let x = vec![1,2,3];
    let y = |num|{ 
        num == &x 
    } ; 
    println!("x is {:?}", y(&x) );
}
*/
/*
fn main() {
    let x = 10; // x 是 10的所有者，拥有10的所有权
    let y = x;  // y 是 10的所有者，由于x是整型变量，数据存放在栈上，把10拷贝了一份给了y ，x 依然存在
    println!("x is {}", x);

    let s = String::from("hello"); // s 是 "hello" 的所有者
    let k = s;  // k 是 "hello" 的所有者，由于s是字符串类型，存放在堆上，把数据转移给了k，同时 s 销毁
    //println!("s is {}", s);
    println!("k is {}", k);
}
*/

/*
fn main() {
    let x = 10 ;

    {
        let y = 9.9;
    }
    println!("y is {}", y);
}
*/

/*
fn main() {
    let name = String::from("davis");

    set_name(name);
    println!("name = {}", name); // name 作为参数传递给了函数 `set_name`，同时所有权也移交给了函数的形参，这里 name 已经不存在了，会报编译错误，跟其他语言完全不一样
}

fn set_name(name:String) {
    println!("name = {}", name);
}
*/
/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 把 s1 的数据克隆一份给了 s2 , s1 依然存在

    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
}
*/
/*
fn main() {
    let name = String::from("davis");

    set_name(&name);
    println!("name = {}", name); // 实参是变量的引用，并不是值的所有权，name依然存在
}
fn set_name(name: &String) {
    println!("name = {}", name);
}
*/
/*
fn main() {
    let name = String::from("davis");

    let new_name = set_name(&name);           // &name 指向 name 的引用，并不拥有值的所有权 
    // println!("name = {}", new_name); 
}
fn set_name(name: &String) {   // name 是对 String 的引用
    name.push_str(" cai")
}
*/
/*
fn main() {
    let mut name = String::from("davis");  // 变量要声明为可变

    set_name(&mut name);    // 引用也要传递 可变引用
}
fn set_name(name: &mut String){   // 参数声明为 可变引用
    name.push_str(",cai");
}
*/
/*
fn main(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
*/
/*
fn main(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}
*/
/*
fn main(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
*/
/*
fn main() {
    let s = String::from("hello");
    let s2 = &s[0..3]; // 从0开始，取3个元素
    println!("s2 is {}", s2);
}
*/
/*
fn main() {
    let s = String::from("hello");
    let s2 = &s[0..=3]; // 从下标0开始，到下标等于3为止
    println!("s2 is {}", s2);
}
*/
/*
fn main() {
    let s = String::from("hello");
    let s2 = &s[..=3]; // 从下标0开始，省略开头的值， 到下标等于3为止
    let s3 = &s[2..];  // 一直到结尾，省略结尾的值
    let s4 = &s[..];  // 取整个字符串

    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    println!("s4 is {}", s4);
}
*/

fn main() {
    let arr = [1,2,3,4,5,6];
    let sub_arr = &arr[1..3];
    println!("sub_arr is {:?}", sub_arr);
}