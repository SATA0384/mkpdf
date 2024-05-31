use anyhow::{Error, Result};
use image::DynamicImage;
use std::io::Cursor;

pub mod debug;
use debug::debug_print;

#[derive(Debug, PartialEq, Eq)]
pub enum ResizeMode {
  Original,
  Custom,
  Min,
  Max,
}

#[derive(Debug)]
pub struct ResizeInfo {
  pub mode: ResizeMode,
  // resolution: Option<(u32, u32)>,
  pub new_resolution: Option<(u32, u32)>,
}

// 画像フォーマットの変換
pub(crate) fn convert_to_jpeg(image: DynamicImage) -> Result<Vec<u8>> {
  debug_print("Converting to jpeg...");
  // jpeg用のバッファを用意
  let mut jpeg_buf = Vec::new();
  let mut seekable_jpeg_buf = &mut Cursor::new(&mut jpeg_buf);

  // バッファにjpeg形式で書き込み
  image
    .into_rgb8()
    .write_to(&mut seekable_jpeg_buf, image::ImageFormat::Jpeg)?;

  Ok(jpeg_buf)
}

// 画像のリサイズ
pub(crate) fn resize_image(image: DynamicImage, r_info: &ResizeInfo) -> Result<DynamicImage> {
  debug_print(format!("{r_info:?}"));

  match r_info.mode {
    // 変換なしの場合
    ResizeMode::Original => return Ok(image),
    ResizeMode::Custom => {
      // 新しい解像度を取得
      let (nwidth, nheight) = r_info.new_resolution.unwrap();

      // リサイズ
      let resized_image = image.resize(nwidth, nheight, image::imageops::FilterType::Triangle);
      return Ok(resized_image);
    }
    ResizeMode::Min => {}
    ResizeMode::Max => {}
  }

  let message = format!(
    "resize_image(): Modes {:?} is not implemented yet.",
    r_info.mode
  );
  Err(Error::msg(message))
}
