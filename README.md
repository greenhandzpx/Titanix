# Titanix
<img src="./docs/fig/hitsz_logo.jpg" style="zoom: 43.7%;" />

### 项目描述

使用Rust编写的，基于riscv64的支持多核的宏内核操作系统。

### 完成情况
初赛满分
<img src="./docs/fig/preliminary.png" style="zoom: 43.7%;" />


### OopS内核介绍
- 进程管理：实现基本的进程与线程管理功能，能够通过比赛测试
- 内存管理：实现基本的内存管理功能。实现mmap，munmap 系统调用。使用 懒分配 和 Copy_on_write 优化策略。
- 文件系统：完成虚拟文件系统。实现块缓存，支持dev文件系统。移植了FAT32文件系统，使用多种缓存机制提高文件系统速度。
- 信号机制：完成基础的信号机制，能够通过比赛测试。
- 用户程序：目前正在适配busybox。


<!-- ### 文档简介

本项目文档位于“/设计文档.pdf”。介绍了OopS的总体架构，以及进程管理、内存管理、文件系统、信号机制4个内核模块，以及总结。 -->

### 代码架构简介
- bootloader: SBI
- os/src: 内核代码
  -  /boards: 开发板配置参数
  -  /config：内核的各个模块的相关配置参数
  -  /driver: 驱动
  -  /fs：文件系统
  -  /mm: 内存管理
  <!-- -  /net：网络 -->
  -  /process: 进程管理
  -  /processor: 多核心管理
  -  /syscall: 系统调用处理函数
  -  /trap: 异常处理
  -  /sync：同步机制
  -  /utils：工具数据结构
  -  main.rs: 主程序
  -  sbi.rs：sbi调用
  -  console.rs: 负责字符输入输出
  -  entry.asm：起始代码
- user: 用户程序

<!-- ### os.bin编译

在根目录下输入

```jsx
make all BOARD=k210
```

可以生成可以在官方K210测试平台上运行的操作系统内核镜像文件 -->

### 运行：


在/os目录下输入

```jsx
make run-fat32
```

<!-- 可以在k210上运行OopS内核 -->


### 项目人员：

哈尔滨工业大学（深圳）：

- 曾培鑫(893522573@qq.com)：进程管理，内存管理，VFS设计，多核支持。
- 陈佳豪(Straho@163.com)：VFS设计，文件系统相关系统调用实现，设备驱动管理。
- 任秦江()：FAT32文件系统设计与实现。
- 指导老师：夏文，仇洁婷

### 参考：
- [rCore-Tutorial v3](https://github.com/rcore-os/rCore-Tutorial-Book-v3)
- [FTL OS](https://gitlab.eduxiji.net/DarkAngelEX/oskernel2022-ftlos/-/tree/master/)
- [jkxs-OS](https://gitlab.eduxiji.net/dh2zz/oskernel2022/-/tree/main)


### 感谢与声明：
本项目使用了洛佳等开发者的RustSBI，以及吴一凡等开发者的rCoreTutorial-v3。

同时感谢我们的学长叶自立，张艺枫，陈林锟，夏文老师和仇洁婷老师，以及同校参赛队伍的帮助。

本项目使用GPL3.0协议