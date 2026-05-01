# souprune_undertale_preset

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE) <img src="https://img.shields.io/github/repo-size/Bli-AIk/souprune_undertale_preset.svg"/> <img src="https://img.shields.io/github/last-commit/Bli-AIk/souprune_undertale_preset.svg"/>

**souprune_undertale_preset** is the maintained Undertale-style reusable preset project for SoupRune.

| English | Simplified Chinese |
|---------|--------------------|
| English | [简体中文](./readme_zh-hans.md) |

## Introduction

This project provides reusable SoupRune content for Undertale-style games.
It includes battle and overworld views, FRE rules, input and flow configuration, dialogue infrastructure, items, player behavior, and WASM runtime/content guests.

This is a library mod, not a standalone game. Use `souprune_mad_dummy_example` when you want to study a complete runnable example project that depends on this preset.

## How to Use

The recommended path is to clone the main SoupRune repository and initialize submodules:

```bash
git clone https://github.com/Bli-AIk/souprune.git
cd souprune
git submodule update --init --recursive
```

This repository is mounted by the main project at:

```text
projects/undertale_preset
```

Project mods can depend on it from their `mod.toml`:

```toml
[dependencies]
undertale_preset = "0.1.0"
```

## Mod Structure

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
