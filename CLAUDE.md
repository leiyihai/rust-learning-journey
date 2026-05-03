# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目说明

这是一个 Rust 编程入门学习项目，用户是编程教练模式下的学习者。项目包含 20 个循序渐进的练习，分为三个阶段：基础入门（1-10）、核心进阶（11-16）、实战应用（17-20）。

## 常用命令

```bash
cargo check                              # 检查所有练习是否编译通过（快速）
cargo build                              # 编译所有练习
cargo run --bin ex01_hello               # 运行单课练习
cargo run --bin ex03_types --release     # release 模式运行
cargo test --bin ex19_testing            # 运行单课测试
```

## 项目架构

- `src/bin/ex01_hello.rs` ... `ex20_todo_app.rs` — 20 个独立的可执行练习文件，每个 `[[bin]]` 在 Cargo.toml 中声明
- `lessons/` — 每课配套讲义（Markdown），用户先读讲义再做练习
- `PROGRESS.md` — 进度追踪表，`[ ]` 改 `[x]` 标记完成
- `NOTES.md` — 用户个人踩坑日志，记录学习中的错误和心得
- `Cargo.toml` — 无外部依赖，纯标准库教学

## 教练工作流程

当用户说"做完 exN_xxx 了"：

1. 读取 `src/bin/exN_xxx.rs` 和 `lessons/NN_xxx.md` 检查完成情况
2. 运行 `cargo run --bin exN_xxx` 验证输出
3. 逐一审查每个任务的正确性，给出反馈
4. 如果正确，提醒用户更新 `PROGRESS.md` 并 git commit

## 练习代码约定

- 已完成的任务：TODO 注释保留，但代码已取消注释并补全
- 未完成的任务：代码被注释掉，或函数体为 `todo!()`
- 用户应优先使用 `todo!()` 让代码可编译，逐步替换为真实实现

## 用户偏好

- 交流使用中文，仅代码块保留英文原文
- 用户有 C# 背景，可用 C# 概念作为类比
