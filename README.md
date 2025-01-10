<div align="center">
  <img src="src/app.png" alt="项目图标" width="200">
</div>

# KaniPing - A  Ping Tool Written  in Rust.

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Status](https://img.shields.io/badge/status-active-success.svg) ![Contributors](https://img.shields.io/github/contributors/earture/kaniping.svg)

> 一个用纯Rust实现的批量网络通断Ping实时监测程序

---
## 背景介绍


---

## 功能特性

项目的主要功能点。

- ✅ 支持直接从excel表格导入需要监测的数据
- ✅ 支持IP和域名
- ✅ 使用Rust原生PING库，减少延迟，低系统资源占用
- ✅ 支持Windows、Linux、MacOS系统

---

## 安装与使用

### 前置条件



### 安装步骤

如您使用Windows系统，可直接下载编译好的可执行文件，运行后点击 Load Excel 导入excel文件，请确保导入的excel文件前三列分别为IP（域名）、名称、位置，并且程序会自动忽略第一列。