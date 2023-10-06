# Network Speed Test Project in Rust

## 簡介

這個 Rust 專案是一個簡單的網路速度測試工具。使用 `reqwest` 來進行 HTTP 下載，並計算下載速度（以 Mbps 為單位）。同時，這個專案還展示了如何使用 `tokio` 的非同步來顯示一個簡單的測試中提示（loading dots）。

## 功能

- 測試 HTTP 下載速度。
- 使用非同步來顯示測試中提示。

## 使用步驟

1. 確保環境已經安裝了 Rust。如果還沒有安裝，請參考 [Rust 官網](https://www.rust-lang.org/) 進行安裝。

2. clone 這個專案。
3. 在專案目錄下，執行 `cargo run`。