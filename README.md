# `pdf-to-image-node` - 不依賴 `canvas` 的 PDF 轉圖片 Node.js Library

> [!IMPORTANT]  
> 安裝本套件前，需要先在作業系統中安裝 `poppler` 與 `cairo` 兩個動態連結庫
>
> -   `poppler`：[https://poppler.freedesktop.org](https://poppler.freedesktop.org)
> -   `cairo` ：[https://www.cairographics.org/download/](https://www.cairographics.org/download/)

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
