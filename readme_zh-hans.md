# souprune_undertale_preset

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_undertale_preset.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_undertale_preset.svg"/>

**souprune_undertale_preset** 是 SoupRune 维护中的 Undertale 风格可复用预设项目。

| 英语 | 简体中文 |
|------|----------|
| [English](./readme.md) | 简体中文 |

## 简介

本项目为 Undertale 风格游戏提供可复用的 SoupRune 内容。
它包含战斗和大地图视图、FRE 规则、输入和流程配置、对话基础设施、物品、玩家行为，以及 WASM runtime/content guest。

这是一个库 mod，不是独立游戏。如果你想学习一个完整可运行、依赖此预设的示例项目，请使用 `souprune_mad_dummy_example`。

## 使用方法

推荐克隆主 SoupRune 仓库并初始化子模块：

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

主项目会将此仓库挂载到：

```text
projects/undertale_preset
```

项目 mod 可以在自己的 `mod.toml` 中依赖它：

```toml
[dependencies]
undertale_preset = "0.1.0"
```

## Mod 结构

```text
undertale_preset/
├── mod.toml
├── runtime/
├── content/
├── .build/
├── app/
├── battle/
├── overworld/
├── actors/
├── narrative/
├── view/
└── assets/
```
