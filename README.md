# Rcli Demo
>
> cargo generate https://github.com/tyr-rust-bootcamp/template

## features
- [x] v1.1
> cargo run --  "Hello World" -c " config list " -dd csv -i ../assets/juventus.csv -o output.json
>
## lib.rs和mod.rs
>lib.rs：这是Rust项目的库文件，每个Rust项目都会有一个lib.rs文件作为项目的根库文件。它用于定义项目中的主要功能和公共接口，可以被其他模块或者外部的项目引用。通常在这个文件中会看到pub关键字来声明公共的函数、结构体或者枚举等。
>
>mod.rs：mod.rs是模块文件，用于组织代码进入更小的单元，即模块(module)。当代码量增加时，可以将相关的函数和类型放入同一个文件夹内，并创建一个mod.rs文件来定义这个模块的内容。这样可以使代码更加模块化，便于管理和重用。mod.rs文件可以包含私有的实现细节，也可以使用pub关键字暴露出公共接口。
>
>总的来说，lib.rs是一个项目的根库文件，用于暴露整个项目的公共接口；而mod.rs是模块文件，用于组织和管理项目内部的代码结构，它可以被lib.rs或其他mod.rs文件引用。