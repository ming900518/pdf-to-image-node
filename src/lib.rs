/// PDF 轉圖片
use std::{fs::File, path::Path, time::SystemTime};

use cairo::{Context, PdfSurface};
use napi::Error;
use poppler::Document;

#[macro_use]
extern crate napi_derive;

/// PDF 轉圖片
#[napi]
pub struct PdfToImage {
    filename: String,
    document: Document,
    timestamp: u128,
}

#[napi]
impl PdfToImage {
    /// 初始化 PDF 轉圖片流程
    ///
    /// # Parameters
    /// - `pdf_path`：**必要參數**，欲開啓的 PDF 檔案路徑，絕對/相對路徑均可接受
    /// - `password`：選填參數， PDF 檔案密碼
    ///
    /// # Errors
    /// 1. `pdf_path` 參數使用相對路徑時，無法取得標準化的 PDF 檔案路徑
    /// 2. PDF 檔案已加密，提供的密碼無效
    /// 3. 檔案名稱不存在
    /// 4. 無法取得當前系統時間
    #[napi(constructor)]
    pub fn new(pdf_path: String, password: Option<String>) -> Result<Self, Error> {
        let path = Path::new(&pdf_path);
        let absolute = path
            .canonicalize()
            .map_err(|error| Error::from_reason(format!("{error:?}")))?;
        Document::from_file(
            &format!("file://{}", absolute.to_string_lossy()),
            password.as_deref(),
        )
        .map_or_else(
            |error| Err(Error::from_reason(format!("{error:?}"))),
            |document| {
                Ok(Self {
                    filename: path
                        .file_name()
                        .and_then(|filename| filename.to_str())
                        .map_or_else(
                            || Err(Error::from_reason("file name unavailable.")),
                            |filename| Ok(String::from(filename)),
                        )?,
                    document,
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map_err(|error| Error::from_reason(format!("{error:?}")))?
                        .as_millis(),
                })
            },
        )
    }

    /// 將初始化的 PDF 檔案轉換為圖片
    ///
    /// *目前僅支援輸出 PNG 格式*
    ///
    /// # Parameters
    /// - `path`：**必要參數**，轉換圖片的存放目標路徑
    /// - `dpi`：選填參數，每英吋點數，此數值越高，產生出來的圖片品質與檔案大小越大，預設為 `150.0`
    ///
    /// # Errors
    /// 1. 無法取得標準化的存放目標路徑
    /// 2. 無法建立輸出 PNG 檔
    /// 3. 無法建立暫存檔
    /// 4. 無法建立 PDF 表面（PDF surface ，渲染畫面用）
    /// 5. 無法利用產生出的 PDF 表面建立渲染參數
    /// 6. 無法拉伸渲染畫面至指定的 DPI 大小
    /// 7. 無法渲染 PDF 背景色
    /// 8. 無法將渲染出的畫面寫入至 PNG 檔
    #[napi]
    pub fn to_image(&self, path: String, dpi: Option<f64>) -> Result<Vec<String>, Error> {
        let path = Path::new(&path);
        let absolute = path
            .canonicalize()
            .map_err(|error| Error::from_reason(format!("{error:?}")))?;

        let mut result = Vec::new();
        let pages = self.document.n_pages();
        let digits = (pages.ilog10() + 1) as usize;

        for i in 0..pages {
            if let Some(page) = self.document.page(i) {
                let output_filename = format!(
                    "{}/{}-{:0digits$}-{}.png",
                    absolute.to_string_lossy(),
                    self.filename,
                    i + 1,
                    self.timestamp,
                );
                let mut file = File::create(&output_filename)
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                let temp_filename = format!(
                    "{}/{}-{:0digits$}-{}.tmp",
                    absolute.to_string_lossy(),
                    self.filename,
                    i + 1,
                    self.timestamp,
                );
                let temp_file = File::create(&temp_filename)
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                let dpi = dpi.unwrap_or(150.0);
                let (width, height) = page.size();
                let surface =
                    PdfSurface::for_stream(dpi * (width / 72.0), dpi * (height / 72.0), temp_file)
                        .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                let context = Context::new(&surface)
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                context.scale(dpi / 72.0, dpi / 72.0);
                context
                    .save()
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                context.set_source_rgba(1.0, 1.0, 1.0, 1.0);
                context
                    .paint()
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;
                page.render(&context);
                surface
                    .write_to_png(&mut file)
                    .map_err(|error| Error::from_reason(format!("{error:?}")))?;

                std::fs::remove_file(&temp_filename).ok();
                result.push(output_filename);
            }
        }
        Ok(result)
    }
}
