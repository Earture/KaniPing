<div align="center">
  <img src="src/app.png" alt="プロジェクトアイコン" width="200">
  <h1 align="center">🦀KaniPing - A Ping Tool Written in Rust</h1>
</div>

<div align="center">
<a href="https://github.com/Earture/KaniPing/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Earture/KaniPing?style=for-the-badge&color=blue" alt="MITライセンス"></a>

 <hr>
</div>

<[English](README_en.md) |  [日本語](README_jp.md) | [简体中文](README.md)>

ようこそ！Rustで作られたネットワークの接続チェックPINGバッチモニターツールです！

<div align="center">
  <img src="./assets/Screenshot.png" alt="プロジェクトスクリーンショット" width="500">
</div>

## 対応機能
- 💫 IPまたはドメインの接続チェックを一括で行う
- 💫 Excelファイルを直接インポート可能
- 💫 軽量で高速、システムリソースの消費が少ない
- 💫 Kylin x86/ARMなどのシステムをサポート

## ⚡ クイックスタート

### 最も簡単な方法は [コンパイル済みの実行ファイルをダウンロードすることです](https://github.com/Earture/KaniPing/releases)

- **1. 実行ファイルをダブルクリックで実行**
> [!WARNING]
> プログラムはRustのネイティブライブラリを使用してPINGリクエストを行うため、ターゲットシステムの管理者権限が必要です！
> - Windows `右クリックして「管理者として実行」を選択`
> - Linux\MacOS `sudo ./KaniPing`
> - MacOS `sudo ./KaniPing`,`sudo ./KaniPing.app/Contents/MacOS/KaniPing`
- **2. 左上の`Load Excel`をクリックしてExcelファイルをインポート**
> [!IMPORTANT]
> Excelファイルの最初の3列はIP（ドメイン）、名前、位置である必要があります。プログラムはヘッダー行を自動的に無視します。
> インポートするファイルが正しく`xlsx`形式で保存されていることを確認してください。
- **3. 左上の`Start Monitoring`をクリックすると、動的モニタリングが開始され、5秒ごとに更新されます**
- **4. 左上の`Stop Monitoring`をクリックすると、モニタリングが停止します**

### 自分でコンパイルしたい場合は、Rust環境を設定後、ルートディレクトリで`cargo run`を実行してください。

## 📜 ライセンス

MITライセンスのもとで配布されています。詳細については[`LICENSE`](./LICENSE)をご覧ください。

## 🐈‍⬛Buy Me A Coffee
<img src="./assets/coffee.jpg" alt="プロジェクトアイコン" width="200">
