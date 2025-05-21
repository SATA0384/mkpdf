use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use debug_print::{
  debug_eprint as deprint, debug_eprintln as deprintln, debug_print as dprint,
  debug_println as dprintln,
};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::io::Cursor;

#[allow(dead_code)]
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
  filter: FilterType,
}

// Default filter (Suggested)
pub const DEFAULT_FILTER: FilterType = FilterType::Triangle;

#[allow(dead_code)]
impl ResizeInfo {
  /* * * * * * * * * * * * * * * * * * * * * * */
  /*                Constructor                */
  /* * * * * * * * * * * * * * * * * * * * * * */
  pub fn new(
    mode: ResizeMode,
    new_resolution: Option<(u32, u32)>,
    filter: Option<FilterType>,
  ) -> Result<Self> {
    let mut ret = ResizeInfo {
      mode: ResizeMode::Original,
      new_resolution: None,
      filter: DEFAULT_FILTER,
    };

    ret
      .set_resoluton(new_resolution)?
      .set_mode(mode)
      .set_filter(filter);
    Ok(ret)
  }

  /* * * * * * * * * * * * * * * * * * * * * * */
  /*                 Accessors                 */
  /* * * * * * * * * * * * * * * * * * * * * * */

  /// Return resize mode.
  pub fn mode(&self) -> &ResizeMode {
    &self.mode
  }

  /// Set resize mode.
  pub fn set_mode(&mut self, mode: ResizeMode) -> &mut Self {
    dprintln!("[my_image::set_mode] mode: {:?} <= {:?}", self.mode, mode);
    self.mode = mode;
    self
  }

  /// Return [`Option`] value containing resolution used with `ResizeMode::Custom`
  pub fn resolution(&self) -> &Option<(u32, u32)> {
    &self.new_resolution
  }
  pub fn set_resoluton(&mut self, new_resolution: Option<(u32, u32)>) -> Result<&mut Self> {
    dprint!("[my_image::set_resolution()] {self:?} <= {new_resolution:?}");
    if self.mode == ResizeMode::Custom && new_resolution == None {
      return Err(anyhow!(
        "Resolution must be Some((u32, u32)) when mode 'Custom'.",
      ));
    }

    self.new_resolution = new_resolution;
    dprintln!(" - Done.");
    Ok(self)
  }

  /// Return filter type that used at resizing.
  pub fn filter(&self) -> &FilterType {
    &self.filter
  }

  /// Set filter type that used at resizing.
  /// If value is [`None`], use `my_image::DEFAULT_FILTER`
  pub fn set_filter(&mut self, filter: Option<FilterType>) -> &mut Self {
    dprint!("[my_image::set_filter()] {self:?} <= {filter:?}");

    self.filter = filter.unwrap_or(DEFAULT_FILTER);
    dprintln!(" - Done.");
    self
  }

  /* * * * * * * * * * * * * * * * * * * * * * */
  /*                  Methods                  */
  /* * * * * * * * * * * * * * * * * * * * * * */

  pub fn resize_all(&self, images: Vec<DynamicImage>) -> Vec<DynamicImage> {
    dprintln!("[my_image::resize_all()] {self:?}");

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
          .map(|image| image.resize(nwidth, nheight, self.filter))
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

        ResizeInfo::new(ResizeMode::Custom, Some((w, h)), Some(self.filter))
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

        ResizeInfo::new(ResizeMode::Custom, Some((w, h)), Some(self.filter))
          .unwrap()
          .resize_all(images)
      }
    }
  }
}

// Jpegフォーマットへの変換
#[allow(dead_code)]
pub fn convert_to_jpeg(image: DynamicImage) -> Result<Vec<u8>> {
  dprint!("Converting to jpeg");

  // jpeg用のバッファを用意
  let mut jpeg_buf = Vec::new();
  let mut seekable_jpeg_buf = &mut Cursor::new(&mut jpeg_buf);

  // バッファにjpeg形式で書き込み
  image
    .into_rgb8()
    .write_to(&mut seekable_jpeg_buf, image::ImageFormat::Jpeg)?;

  dprintln!(" - Done.");
  Ok(jpeg_buf)
}

// 画像のリサイズ
#[allow(dead_code)]
pub(crate) fn resize_image(image: DynamicImage, r_info: &ResizeInfo) -> Result<DynamicImage> {
  dprintln!("{r_info:?}");

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
  Err(anyhow!(message))
}
