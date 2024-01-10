# Rust学习笔记 #
- [Rust学习笔记](#rust学习笔记)
  - [self与Self](#self与self)
  - [模块](#模块)
    - [绝对路径](#绝对路径)
  - [宏](#宏)
    - [声明式宏](#声明式宏)
    - [过程宏](#过程宏)
  - [语句和表达式](#语句和表达式)
    - [语句](#语句)
      - [声明语句](#声明语句)
      - [表达式语句](#表达式语句)
    - [表达式](#表达式)
  - [函数](#函数)
    - [带参数函数](#带参数函数)
    - [有返回值的函数](#有返回值的函数)
  - [杂碎](#杂碎)
    - [与github配合](#与github配合)
    - [本地](#本地)
    - [命名法则](#命名法则)

---
## self与Self ##
self表示调用方法的对象，作为类方法的第一个参数，类似于C++中的this。

Self表示调用者的类型。
```Rust
impl Clone for MyType {
    // 可以直接写具体类型
    fn clone(&self) -> MyType;
    // 也可以用Self代替
    fn clone(&self) -> Self;
}

impl MySuperLongType {
    // 用Self写起来更短
    fn new(a: u32) -> Self { ... }
}
```

Rust中函数参数均需要注明类型，但是self则不需要，这是一个语法糖（syntactic sugar），以下示例中两两等价：
```Rust
impl MyType{
    fn doit(self){}
    fn doit(self: Self){}

    fn doit(&self) {}
    fn doit(self: &Self){}

    fn doit(&mut self) {}
    fn doit(self: &mut Self) {}
}
```
## 模块 ##
### 绝对路径 ###
路径有两种形式：
绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 crate 开头。
相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
当前项目的crate值为当前src的路径。
## 宏 ##
Rust宏很强大，
### 声明式宏 ###
println!后面加！都是声明式宏，目前理解让代码看起来更简单，当然宏的实现者把复杂性给隐藏了
### 过程宏 ###
```Rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

assert_eq!(format!("The origin is: {origin:?}"), "The origin is: Point { x: 0, y: 0 }");
```
#[derive(Debug)]是Rust的派生式宏的用法，给构体Point派生一个Debug方法，相当于下面代码。
```Rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

let origin = Point { x: 0, y: 0 };

assert_eq!(format!("The origin is: {origin:?}"), "The origin is: Point { x: 0, y: 0 }");
```

## 语句和表达式 ##
### 语句 ###
#### 声明语句 ####
声明语句是在它自己封闭的语句块的内部引入一个或多个名称的语句。声明的名称可以表示新变量或新的程序项。

这两种声明语句就是程序项声明语句和 let声明语句。
```Rust
let outer_var = true;
```
let语句，
#### 表达式语句 ####
表达式+分号形成表达式语句。
### 表达式 ###
太复杂，待总结。

## 函数 ##
Rust中的函数定义以 fn 开始，后跟着函数名和一对圆括号。大括号告诉编译器函数体在哪里开始和结束。

可以使用函数名后跟圆括号来调用我们定义过的任意函数。Rust 不关心函数定义于何处，只要定义了就行。

### 带参数函数 ###
```Rust
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```
another_function 的声明中有一个命名为 x 的参数。x 的类型被指定为 i32。当将 5 传给 another_function 时，println! 宏将 5 放入格式化字符串中大括号的位置。

### 有返回值的函数 ###
函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可以从函数中提前返回；但大部分函数隐式返回最后一个表达式。
```Rust
fn five() -> i32 {
    5
}
```
在 five 函数中没有函数调用、宏，甚至没有 let 语句——只有数字 5 本身。这在 Rust 中是一个完全有效的函数。注意，函数返回值的类型也被指定好，即 -> i32

## 杂碎 ##

### 与github配合 ###

创建名为learn_rust的仓库

### 本地 ###

```bash
cargo new learn_rust
git add .
git commit -m "init"
git remote add origin git@github.com:gocpicnic/learn_rust.git
git branch -M main
git push -u origin main
```
---

### 命名法则 ###

Rust使用是蛇形命名法，其中每个单词都是由小写字母组成，且单词之间由下划线连接。例如，在蛇形命名法中，一个名为 "hello_world" 的 crate 名称是合法的，而 "helloWorld" 则是不合法的，会报警告。

