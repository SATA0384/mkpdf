use anyhow::{Error, Ok, Result};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct ResizeInfo {
  mode: ResizeMode,
  new_resolution: Option<(u32, u32)>,
}

#[allow(dead_code)]
impl ResizeInfo {
  /* Constructor */
  pub fn new(mode: ResizeMode, new_resolution: Option<(u32, u32)>) -> Result<Self> {
    let mut ret = ResizeInfo {
      mode: ResizeMode::Original,
      new_resolution: None,
    };

    ret.set_mode(mode).set_resoluton(new_resolution)?;
    Ok(ret)
  }

  /* Accessors */
  pub(crate) fn mode(&self) -> &ResizeMode {
    &self.mode
  }
  pub(crate) fn set_mode(&mut self, mode: ResizeMode) -> &mut Self {
    self.mode = mode;
    self
  }
  pub(crate) fn resolution(&self) -> &Option<(u32, u32)> {
    &self.new_resolution
  }
  pub(crate) fn set_resoluton(&mut self, new_resolution: Option<(u32, u32)>) -> Result<&mut Self> {
    if self.mode == ResizeMode::Custom && new_resolution == None {
      return Err(Error::msg(
        "Resolution must be Some((u32, u32)) when mode 'Custom'.",
      ));
    }

    self.new_resolution = new_resolution;
    Ok(self)
  }

  /* Methods */
  pub(crate) fn resize_all(&self, images: Vec<DynamicImage>) -> Vec<DynamicImage> {
    match self.mode {
      // リサイズなし
      ResizeMode::Original => images,

      // カスタム解像度(アス比保持)
      ResizeMode::Custom => {
        // 新しい解像度を取得
        debug_assert_ne!(self.new_resolution, None);
        let (nwidth, nheight) = self.new_resolution.unwrap();

        let resized_images: Vec<DynamicImage> = images
          .into_iter()
          .map(|image| image.resize(nwidth, nheight, FilterType::Triangle))
          .collect();

        resized_images
      }

      // 引数の配列のうち最小のものに合わせる
      ResizeMode::Min => {
        let (mut w, mut h) = &images[0].dimensions();

        for image in &images[1..] {
          w = w.min(image.dimensions().0);
          h = h.min(image.dimensions().1);
        }

        ResizeInfo::new(ResizeMode::Custom, Some((w, h)))
          .unwrap()
          .resize_all(images)
      }

      // 引数の配列のうち最大のものに合わせる
      ResizeMode::Max => {
        let (mut w, mut h) = &images[0].dimensions();

        for image in &images[1..] {
          w = w.max(image.dimensions().0);
          h = h.max(image.dimensions().1);
        }

        ResizeInfo::new(ResizeMode::Custom, Some((w, h)))
          .unwrap()
          .resize_all(images)
      }
    }
  }
}

// Jpegフォーマットへの変換
#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) fn resize_image(image: DynamicImage, r_info: &ResizeInfo) -> Result<DynamicImage> {
  debug_print(format!("{r_info:?}"));

  match r_info.mode {
    // 変換なしの場合
    ResizeMode::Original => return Ok(image),
    ResizeMode::Custom => {
      // 新しい解像度を取得
      let (nwidth, nheight) = r_info.new_resolution.unwrap();

      // リサイズ
      let resized_image = image.resize(nwidth, nheight, FilterType::Triangle);
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
