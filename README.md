<div align="center">
  <img src="src/app.png" alt="项目图标" width="200">
  <h1 align="center">🦀KaniPing - A  Ping Tool Written in Rust.</h1>
</div>

<div align="center">
<a href="https://github.com/Earture/KaniPing/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Earture/KaniPing?style=for-the-badge&color=blue" alt="MIT License"></a>

 <hr>
</div>

欢迎使用!一个由Rust构建的网络通断PING批量监测工具！

<div align="center">
  <img src="./assets/Screenshot.png" alt="项目截图" width="500">
</div>

## ⚡ 快速开始

最简单的方法是 [直接下载编译好的可执行文件](https://docs.all-hands.dev/modules/usage/runtimes#connecting-to-your-filesystem)

- 1.双击运行文件
> [!WARNING]
> 因为程序使用了Rust原生库进行PING请求，所以需要目标系统的管理员权限！
> - Windows `右键选择以管理员身份运行`
> - Linux\MacOS `sudo ./KaniPing`
- 2.点击左上角`Load Excel`导入excel文件，excel文件
> [!IMPORTANT]
> Excel文件前三列必须为IP（域名）、名称、位置，同时程序会自动忽略作为表头的第一行
- 点击左上角`Start Monitoring` 即可开始动态监测，每`5秒`刷新一次
- 点击左上角`Stop Monitoring` 即可停止监测刷新


## 📜 License

Distributed under the MIT License. See [`LICENSE`](./LICENSE) for more information.