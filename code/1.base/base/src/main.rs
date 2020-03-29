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
/*
fn main() {
    let arr = [1,2,3,4,5,6];
    let sub_arr = &arr[1..3];
    println!("sub_arr is {:?}", sub_arr);
}
*/

/*
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    active: bool
}

fn main() {
    let user = User {
        name: String::from("davis"),
        age: 20,
        active: true
    };
    println!("user is {:#?}", user);
}
*/
/*
struct User {
    name: String,
    age: i32,
    active: bool
}

fn main() {
    let user = User {
        name: String::from("davis"),
        age: 20,
        active: true
    }; // 别忘了分号

    println!("user is {}", user.name); // 访问结构体里的字段
}
*/

/*
struct User {
    name: String,
    age: i32,
    active: bool
}

fn main() {
    let mut user = User {
        name: String::from("davis"),
        age: 20,
        active: true
    }; // 别忘了分号

    user.name = String::from("tina");
    println!("user is {}", user.name); // 访问结构体里的字段
}
*/
/*
struct User {
    name: String,
    age: i32,
    active: bool
}

fn main() {
    let user = build_user(String::from("davis"));
     println!("user is {}", user.name);
}

fn build_user(name: String) -> User{
    User {
        name,
        age: 25,
        active: true
    }
}
*/
/*
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    active: bool
}

fn main() {

    let user1 = User {
        name: String::from("davis"),
        age: 26,
        active: true
    };

    let user2 = User {
        age: 30,
        ..user1  // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
    };

    println!(" user2 is {:#?}", user2 );
}
*/
/*
#[derive(Debug)]
struct Point (i32, i32, i32);

fn main() {
    let p = Point(2,3,4);
    println!("p is {:?}", p);
}
*/

/*
struct User {
    name: String,
    age: i32,
    active: bool
}

// 定义结构体方法
impl User {
    fn set_name(&mut self, name: String) {  
        self.name = name;
    }

    fn get_name(&self) -> &String{
        &self.name
    }
}

fn main() {
    let mut user = User{ name: String::from("davis"), age: 24, active: true } ;
    user.set_name(String::from("tina"));
    let name = user.get_name();
    println!("{}",name);
}
*/

/*
#[derive(Debug)]
struct User {
    name: String,
    age: i32
}

impl User {
    fn build(name:String, age:i32) -> User {
        User{name, age}
    }
}

fn main() {
    let user = User::build(String::from("davis"), 32);
    println!("{:#?}", user);
}
*/
/*
#[derive(Debug)]
enum IP {
    v4,
    v6
}

fn main() {
    let v4 = IP::v4;
    let v6 = IP::v6;

    println!("{:#?}", v4);
    println!("{:#?}", v6);
}
*/
/*
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let m1 = Message::Move{x:30, y:2};
    let m2 = Message::Write(String::from("hello"));
    let m3 = Message::ChangeColor(220,190,100);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
}
*/

/*
// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
*/

/*
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; // 声明类型别名

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add; 
}
*/

/*
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
*/

/*
enum Message {
    Write(String),
    Move { x:i32, y: i32}
}

impl Message {
    fn call(&self) {
        println!("do something");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
*/

/*
enum IP {
    V4,
    V6
}

fn route(ip_type: IP) -> i32 { 
    match ip_type {
        IP::V4 =>{ 
            println!("this is ipv4"); 
            return 1; 
        }
        IP::V6 =>{ 
            println!("this is ipv6"); 
            2
        }
    }
}

fn main() {
    let s = route(IP::V4);
    println!("s is {}", s);
}
*/

/*
fn plus_one(x: Option<i32>) -> i32 {
    match x {
        Some(k) => k + 1,
        None => 0
    }
}

fn main() {
    let x = plus_one(Some(5));
    let y = plus_one(None);
    println!("x is {}", x);
    println!("y is {}", y);
}
*/

/*
fn plus_one(x: Option<i32>) -> i32 {
    match x {
        Some(k) => k + 1,
        // None => 0  去掉 None 成员的分支
    }
}

fn main() {
    let x = plus_one(Some(5));
    let y = plus_one(None);
    println!("x is {}", x);
    println!("y is {}", y);
}
*/
/*
fn plus_one(x: Option<i32>) -> i32 {
    /*
    match x {
        Some(k) => k + 1,
        None => 0
    }
    */
    if let Some(k) = x { //  要模式在前，实参在后，不能这样写： x = Some(k) 
        k + 1
    } else {
        0
    }
}

fn main() {
    let x = plus_one(Some(5));
    let y = plus_one(None);
    println!("x is {}", x);
    println!("y is {}", y);
}
*/

/*
fn make_pair<T,U>(a: T, b:U) -> (T,U) {
    (a,b)
}
  
fn main()
{
  let couple1 = make_pair("man", "female");
  println!("couple1 = {:?}", couple1);

  let couple2 = make_pair(99i32, 109f64);
  println!("couple2 = {:?}", couple2);
}
*/
/*
struct Point<T, U> {
    x: T, 
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
*/
/*
enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
} 

fn main() {

}
*/

/*
struct Point<T> {
    x: T,
    y: T
}

impl Point<i32> {
    fn plus(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let p = Point{x:10, y:20};
    let area = p.plus();
    println!("area is {}", area);
}
*/

/*
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl Point<i32> {
    fn plus(&self) -> i32 {
        self.x + self.y
    }

    fn build(&self) -> Point<i32> {
        Point{ x: self.x, y: self.y }
    }
}

impl<T> Point<T> {
    fn new(&self) -> &T {
        &self.x 
    }
}

fn main() {
    let p = Point{x:10, y:20};
    let area = p.plus();
    println!("area is {}", area);

    let p1 = p.build();
    println!("{:?}", p1);

    let x = p.new();
    println!("{}", x);

}
*/

/*
trait Car {

    fn lanuch() {           // 声明默认的方法，并实现
        println!("car start lanuch");
    }

    fn desc(&self) ;        // 声明接口方法，不实现

    fn query_miles(&self) -> i32 ;  // 声明接口方法和返回值，不实现
}

struct MVP {
    name: String,
    color: String,
    power: i32,
    miles: i32
}
 
impl Car for MVP {
    fn desc(&self) {
        println!("{} , color is {}, and power is {}", self.name, self.color, self.power);
    }

    fn query_miles(&self) -> i32 {
        self.miles
    }
}

fn main() {
    MVP::lanuch();  // 调用默认接口方法
    let m = MVP{name: String::from("mvp car"), color: String::from("red"), power: 320, miles: 10000};
    m.desc();
    let mile = m.query_miles();
    println!("miles is {}", mile);
}
*/

/*
trait Car {

    fn lanuch() {           // 声明默认的方法，并实现
        println!("car start lanuch");
    }

    fn desc(&self) ;        // 声明接口方法，不实现

    fn query_miles(&self) -> i32 ;  // 声明接口方法和返回值，不实现

    fn add_oil(&self, num:i32);

}

struct MVP {
    name: String,
    color: String,
    power: i32,
    miles: i32
}
 
impl Car for MVP {
    fn desc(&self) {
        println!("{} , color is {}, and power is {}", self.name, self.color, self.power);
    }

    fn query_miles(&self) -> i32 {
        self.miles
    }

    fn add_oil(&self, num: i32) {
        println!("add oil is {}", num);
    }
}

fn main() {
    MVP::lanuch();  // 调用默认接口方法
    let m = MVP{name: String::from("mvp car"), color: String::from("red"), power: 320, miles: 10000};
    m.desc();
    let mile = m.query_miles();
    println!("miles is {}", mile);

    add_oil(m);
    //add_oil2(m);
   
}

// trait 作为函数参数
fn add_oil(car: impl Car) {
    car.add_oil(100);
}

// trait bounds
fn add_oil2<T: Car>(car: T) {
    car.add_oil(200);
}
*/

/*
fn main() {
    let mut v = vec![2,3,4,5,6];
    v.push(9); // 追加新元素到最后

    let b = &v[0];  // 读取元素
    let b2 = v.get(3);  // 读取元素，返回 Option 枚举类型
    println!("b is {}", b);
    println!("b2 is {:?}", b2);

    v.pop();  // 移除最后一个元素

    // 遍历 Vector 
    for ele in &v {
        println!("value is {} ",ele);
    }
}
*/

/*
use std::collections::HashMap;
fn main() {
    // 创建一个 hashmap 
    let mut hm = HashMap::new();

    // 插入元素，分别为 key 和 value 
    hm.insert("blue", 100);

    // 根据 key 读取 元素，返回 Option 枚举类型
    let b = hm.get("blue");
    println!("{:?}", b);

    // 如果不存在指定 key，就插入值 
    hm.entry("yel").or_insert(50);

    // 遍历 hashmap 
    for (key, val) in &hm {
        println!("key={}, val={}" , key, val);
    }
}
*/

/*
use std::fs::File;
fn main() {
    let f = File::open("t.txt"); // f 是 Result 枚举类型

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file not found error : {:?}", error); // panic! 输出错误信息并停止程序执行
        },
    };
}
*/

/*
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("t.txt"); // f 是 Result 枚举类型

    let f = match f {
        Ok(file) => file,
        Err(error) =>  match error.kind() {  // 匹配错误
            ErrorKind::NotFound => match File::create("t.txt"){  // 创建文件
                Ok(new_file) => new_file,
                Err(e) => panic!("can not create file"),
            },
            other_error => panic!("other error"),
        },
    };
    println!("{:?}", f);
}
*/

/*
use std::fs::File;

fn main() {
    let f = File::open("t1.txt").unwrap();
    //let f = File::open("t1.txt").expect("file not found");
    println!("{:?}", f);
}
*/

/*
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let f = open_file();
    println!("{:?}", f);
}

fn open_file() -> Result<String, io::Error> {
    let f = File::open("t1.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
*/

/*
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let f = open_file();
    println!("{:?}", f);
}

fn open_file() -> Result<String, io::Error> {
    let mut f = File::open("t1.txt")? ;
    let mut s = String::new();
    f.read_to_string(&mut s)? ;
    Ok(s)
}
*/

/*
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let f = open_file();
    println!("{:?}", f);
}

fn open_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("t1.txt") ?.read_to_string(&mut s) ?  ;  // 返回错误的链式写法 ?. 
    Ok(s)
}
*/


#[cfg(test)]
mod test {

    #[test]  // 表明这是一个测试函数
    fn it_eq(){
        assert_eq!(4,4);  // 断言
    } 

    #[test]
    #[should_panic] // 表明这个测试函数会出现 panic 报错 ，出现了则测试通过，否则测试失败
    fn another() {
        panic!("Make this test fail");
    }
}

fn main() {

}