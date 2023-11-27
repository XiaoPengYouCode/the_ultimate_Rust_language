# 项目运行的方式

## rustc 编译 rust 代码

普通编译方式

## cargo创建项目并编译

`cargo new` 会创建一个项目文件夹，是一个 git 仓库，里面有源文件和 toml 配置文件，还有 git 相关配置

我们通过

```shell
cargo new hello
```

创建一个 hello 的项目文件夹

## 运行项目

```shell
cargo run
```

使用该命令进行项目的编译运行，可以拆分为两份操作

```shell
cargo build # 编译项目
./target/debug/hello.exe # 运行编译结果
```

但是这是在 debug 模式下进行的编译，这种模式下编译器优化较少，编译速度较快，对应的，运行速度就会变慢
相反的，我们可以在编译时使用 `--release` 参数进行编译，这样可以得到高性能的 `release` 程序

编译结果在 `./target/release` 下

## 检查项目

项目过大，编译过慢，可以使用 `cargo check` 做最快的检查

## Cargo.toml 和 Cargo.lock

写到项目的时候慢慢看
