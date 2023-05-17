# 这是什么

把比赛 `libc-test` 分支中的测例单独拿出来编译，而不是必须通过 `runtest.exe` 和 `entry-static.ext/entry-dynamic.exe` 运行。然后可用 qemu-riscv64 在用户态运行测试，也可生成镜像。

这样可以单独测试一个测例，**避开上层的 `runtest.exe` 中一些复杂的 syscall**，方便OS开发。

## 可能需要的安装

qemu，本例是 6.1.1

> 可能需要先安装 Ninja 库。使用 `apt install ninja-build`

在包内
```bash
mkdir build
cd build
../configure --target-list=riscv64-softmmu,riscv64-linux-user
sudo make install
```
最终会安装到`usr/local/bin`下。前者是`qemu-system-riscv64`，用于运行OS；后者是`qemu-riscv64`，用于运行用户程序

## 运行

用户态执行：

```bash
qemu-riscv64 ./src/a.out
```

实际执行：

```bash
make
```
会在当前目录下的 `build/` 中生成所有可单独执行的测例文件。

将生成的文件放到FAT镜像中即可作为 OS 的输入。

另外，可以在生成文件后执行

```bash
make str_array_in_rust
```
这样可以把所有测例名输出成一个字符串数组(`&[&str]`)格式的`rust`变量，方便在代码中使用。

## 进度

可编译执行静态测例

## 其他/tips

检查文件格式
`file ./a.out`

记得musl编译的时候选一下静态

## 文件引用说明

- 测例对应 `testsuits-for-oskernel/libc-test` 目录下的测例，不一定和原版 libc 相同。
- 要求的静态/动态测例列表 `dynamic_testcases.txt / static_testcases.txt` 修改自库 `testsuits-for-oskernel`。
- `Makefile` 借鉴了 `libc-test` 和 `testsuits-for-oskernel` 的写法，但也是这个模块主要做的大幅修改的部分。否则直接用提供的 `Makefile` 就可以测试了，不需要这个模块
