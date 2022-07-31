---
theme: condensed-night-purple
---

> 源码仓库： https://github.com/ericlee33/rust-courses
>
> 作者: https://github.com/ericlee33
>
> 如果文章有帮助的话，欢迎 ⭐️ 点赞 ⭐️，关注我

# 前言

本文从一个掌握 `JavaScript` 语言开发者的视角，带你从 `JS` 的视角理解 `Rust` 语法，快速上手 `Rust` 语言

# 安装 Rust 环境

https://www.rust-lang.org/learn/get-started

# 生成 Rust 项目

我们用类似`JS`**monorepo**的方式组织我们的`Rust`项目，在`./crates`目录下

我们用 `cargo new tutorials_1`命令创建一个项目

[Rust Workspace 教程](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## 启动 Rust 项目

在项目根目录运行`cargo run`，即可编译并运行项目

如果只想编译，请运行`cargo build`，会生成二进制可执行文件

# 教程项目目录结构

![image.png](https://p9-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/dd9d6207b16740a49ea1968c10b8b4d4~tplv-k3u1fbpfcp-watermark.image?)

通过`cargo`生成的模板项目，只有`main.rs`和`Cargo.toml`。

为了研究`Rust`模块系统我们额外创建了`lib.rs`和`func.rs`文件

# 入口文件`main.rs`

> 在`NodeJs`中，为了执行一个程序，需要一个`index.js`文件，只不过在`Rust`中是将`main.rs`定义到了规范中。

当运行`Rust`项目的时候，会以`main.rs`的`main`函数为入口，开始执行程序。

# `lib.rs`用途

> 在`JS`中，我们通过在`package.json`中指定`"main"`来决定打包发布后，项目的入口文件。

在`Rust`中，制定了一套规范，打包后的`crate`入口为`src/lib.rs`文件，所以，如果我们的包需要发布，项目中一定需要存在一个`lib.rs`做入口。

> crate: 类似于`JS`中`package`的概念，并且`Rust`也存在一套类似`npm`的机制，可见 https://crates.io/

# 语法

## 变量

### 绑定变量

在`Rust`中，声明一个变量，只要像`js`一样声明即可

```rs
let x = "hello world";
```

`js`支持直接给一个声明后的变量重新赋值，`Rust`默认是不支持的，如果需要支持，需要如下写法

```rs
let mut x = 5;
```

其中`mut`关键字代表使该变量可变

### 变量解构

类似于`ES6`，`Rust`也支持变量解构绑定

```rs
fn main() {
    let (a, mut b): (bool, bool) = (true, false);

    println!("a: {}, b: {}", a, b);

    b = true;

    println!("a: {}, b: {}", a, b);

}
```

`println!`是一个宏，类似于`js`的`console.log()`，通过 {} 匹配对应位置的变量，第二个参数为 `...args: any[]`类型`

## 单模块导出

大部分`Rust`教程不会在一开始教模块语法，都是在`JS`中，我们经常会用到模块导入导出，在`ES6`中，我们使用`import`进行导出，

### pub、mod

> `pub`类似于`es6`的`export`，`mod`类似于`import`

mod 用途：声明一个模块。用`TS`的话来说，类似于声明一个`namespace`的感觉，让我们看看用法

```rs
// lib.rs
pub mod same_level_utils;
```

在`lib.rs`文件中。

我们通过`mod`关键字，引用了`same_level_utils`为一个模块。

添加`pub`前缀，表面这是一个公开模块，这样我们就将**模块导出**了。

> 这里相当于做了类似于`es6`的 `export same_level_utils from 'same_level_utils'`的操作

`Rust`默认有 2 种导入寻找方式：

1. 优先寻找文件是否存在，如果有，则导入文件
2. 如果该名称文件不存在，寻找是否存在`./same_level_utils`的文件夹中的`mod.rs`文件

> 这里类似于`js`的`import utils from './utils'`，优先寻找`./utils.js`文件，如果不存在的话，会寻找`./utils/index.js`文件

我们这里先介绍的是第一种，我们这里的`same_level_utils.rs`与`lib.rs`在相同层级

> 在`TS`中，我们在`class`中也有使用`public`、`private`前缀表示属性/方法，是否可供外部访问，和这里是类似的。

```rs
// same_level_utils.rs
pub fn print_abc_word() {
    println!("abc")
}
```

让我们看看`same_level_utils.rs`文件的内容，这里`fn`关键字表示是一个函数，类似于`JS`中的`function`关键词，表示将`print_abc_word`函数暴露出去。

### use

```rs
// main.rs
use tutorials_1::same_level_utils;

fn main() {
    same_level_utils::print_abc_word();
}
```

> 在`JS`中，我们使用`import`关键字导入暴露出的模块

这里我们使用绝对路径的方式引入，注意到我们包的名字为`tutorials_1`，类似于`JS`中`package.json`的`name`，那么模块命名空间就为`tutorials_1`

调用或使用嵌套模块的方法也很简单，只要使用两个冒号 (`::`) 从左到右拼接从外到内的模块即可

`print_abc_word`其实属于`same_level_utils`这个命名空间的方法，这里我们通过`::`的方式引入，执行函数的方式与`JS`是相同的，使用`()`执行函数。

让我们看看执行结果：

![image.png](https://p9-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/8d5026b3d6af443fa23855ed91dbae6f~tplv-k3u1fbpfcp-watermark.image?)

## 如何组织目录，更好的模块导出方式

在`JS`中，我们有时会将工具`模块`放到`utils`目录中，那么，在`rust`中怎么组织呢？

```rs
// lib.rs
pub mod utils;
```

我们在`lib.rs`中导出`utils`模块，由于我们的文件夹没有`util.rs`，他会去寻找`utils`目录，这种情况下，入口文件为`utils/mod.rs`，`Rust`会去识别`mod.rs`文件，这是一个约定。

```rs
// mod.rs
pub mod date;
```

在`mod.rs`中，我们导出`date`模块

```rs
// date.rs
use std::time::SystemTime;

pub fn get_current_system_time() -> SystemTime {
    SystemTime::now()
}
```

在`date`模块中，我们导出`get_current_system_time`函数，

使用方式

```rs
// main.rs
use tutorials_1::same_level_utils;

fn main() {
    same_level_utils::print_abc_word();
}
```

![image.png](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/e0f3185c183d41869ec8456d83af8d0d~tplv-k3u1fbpfcp-watermark.image?)

# 总结

至此，我们熟悉了`Rust`的变量与模块导入，可以快速创建起来一个项目，并且层级分明。

# 后言

如果文章对你有帮助，给文章和源码仓库**点个赞**吧。如果有不懂的地方可以看我资料加微信进行交流。

> 源码仓库： https://github.com/ericlee33/rust-courses
