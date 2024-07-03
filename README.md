# `pdf-to-image-node` - 不依賴 `canvas` 的 PDF 轉圖片 Node.js Library

> [!IMPORTANT]  
> 安裝本套件前，需要先在作業系統中安裝 Rust 程式語言、 `poppler` 與 `cairo` 動態連結庫、以及編譯所需的工具
>
> -   `Rust`：[https://rust-lang.org/](https://rust-lang.org/)
> -   `poppler`：[https://poppler.freedesktop.org/](https://poppler.freedesktop.org/)
> -   `cairo` ：[https://www.cairographics.org/download/](https://www.cairographics.org/download/)
>
> 請參閱下方「安裝必要依賴」段落配置環境

## 使用範例

```javascript
import { PdfToImage } from "pdf-to-image-node";

// 指定輸出 PNG 檔案時，以多少 DPI 渲染 PDF 檔，預設為 `150`，不修改可填 null 或 undefined
const targetDpi = 100;
try {
    new PdfToImage("*PDF 檔案路徑*", "*PDF 檔案密碼，如無密碼可留空或填 null*").toImage(
        "*輸出檔案目錄路徑*",
        targetDpi
    );
} catch (error) {
    /// TODO: 可能的錯誤場景，請參閱每個 method 的 TypeScript Type Definition
}
```

## 安裝必要依賴

1. 安裝 Rust 程式語言

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. 安裝編譯所需工具及動態連結庫

    - macOS

        ```
        xcode-select --install
        brew install pkg-config glib cairo poppler
        ```

    - Debian/Ubuntu

        ```
        sudo apt install build-essentials pkg-config libglib2.0-dev libcairo2-dev libpoppler-dev libpoppler-glib-dev
        ```
