# 期末作業

## JPEG 解碼器
此為引用 [這裡](https://github.com/MROS/jpeg_tutorial) 的 JPEG 解碼器專案，原作者以JPEG轉換為PPM為例。我對於 JPEG 解碼的過程已大部分理解，而將其轉換為 PPM 檔的過程稍微理解。這次打算製作將其轉換為 PNG 檔的程式，基本上是以原作者的 [ppm.rs](https://github.com/akito1129/sp111b/blob/main/Finalterm/src/ppm.rs) 作為主體修改，並參考了ChatGPT的建議。

先來說說 PPM 跟 PNG 的差異在哪裡，PPM 是以ASCII文本格式寫入，而 PNG 則是以二進制，所以我在 [Cargo.toml](https://github.com/akito1129/sp111b/blob/main/Finalterm/Cargo.toml) 中將 png 加入依賴項來導入我要的 png 相關函數及結構。主要用來輸出成 PNG 格式的程式為 [png.rs](https://github.com/akito1129/sp111b/blob/main/Finalterm/src/png.rs) ，為確保能夠運行，我在 [main.rs](https://github.com/akito1129/sp111b/blob/main/Finalterm/src/main.rs) 中做了些許修改，其餘的都不需要進一步的更改，但保留優化空間。

我用了一些網路上抓的 JPEG 資料測試，如需要實測結果請參考 [01.md](https://github.com/akito1129/sp111b/blob/main/Finalterm/01.md)、[02.md](https://github.com/akito1129/sp111b/blob/main/Finalterm/02.md)、[03.md](https://github.com/akito1129/sp111b/blob/main/Finalterm/03.md)、[04.md](https://github.com/akito1129/sp111b/blob/main/Finalterm/04.md)。