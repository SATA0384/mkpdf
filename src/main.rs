use anyhow::{Error, Result};
use image::{io::Reader as ImageReader, DynamicImage};
use jpeg_to_pdf::JpegToPdf;
use regex::Regex;
use std::{
  ffi::OsStr,
  fmt::Display,
  fs::{self, File},
  io::{BufWriter, Cursor},
  path::Path,
};

const SELF_VERSION: &str = env!("CARGO_PKG_VERSION");
const IS_DEBUG: bool = false;

fn print_help() {
  print!("mkpdf - ");
  print_version();
  print_usage();

  println!("\nDescription: Create PDF file from multiple images.\n");

  println!("Options:");
  println!("  --help    | -h ) Print this help message and exit.");
  println!("  --resize <mode> ) Resize images and add to PDF.");
  println!("        -r <mode> ) <mode>: <width>x<height> ) Resize to specified size.");
  // println!("                          : min ) Resize to fit the smallest one.");
  // println!("                          : max ) Resize to fit the leargest one.");
  println!("  --version | -v ) Print version and exit.");
}

fn print_usage() {
  println!("Usage: mkpdf <output_file> <input_image1> [<input_image2>...]");
  println!("  <output_file> : if not *.pdf, automatically append '.pdf'");
  println!("  <input_image> : Supported formats are [.jpg(.jpeg)|.png|.bmp]");
}

#[derive(Debug, PartialEq, Eq)]
enum ResizeMode {
  Original,
  Custom,
  Min,
  Max,
}
#[derive(Debug)]
struct ResizeInfo {
  mode: ResizeMode,
  // resolution: Option<(u32, u32)>,
  new_resolution: Option<(u32, u32)>,
}

fn main() -> Result<()> {
  let mut resize_info = ResizeInfo {
    mode: ResizeMode::Original,
    // resolution: None,
    new_resolution: None,
  };

  // 引数の取得
  let args = {
    // オプション含むシェル引数を取得
    let raw_args: Vec<String> = std::env::args().collect();

    // オプション解析
    let pat_short_opt = Regex::new(r"^-[0-9|a-z|A-Z]").unwrap();
    let pat_long_opt = Regex::new(r"^--[0-9|a-z|A-Z]").unwrap();
    let mut non_option_args = Vec::new();
    let mut is_resize_mode = false;

    for arg in raw_args {
      if is_resize_mode {
        let pat_resolution = Regex::new("^[1-9][0-9]{2}[0-9]?x[1-9][0-9]{2}[0-9]?$").unwrap();
        resize_info.mode = match arg.as_str() {
          "min" => ResizeMode::Min,
          "max" => ResizeMode::Max,
          _ => {
            if pat_resolution.is_match(&arg) {
              let res: Vec<u32> = arg.split('x').map(|x| x.parse().unwrap()).collect();
              resize_info.new_resolution = Some((res[0], res[1]));
              ResizeMode::Custom
            } else {
              return Err(Error::msg(format!("Invalid resolution: {}", arg)));
            }
          }
        };
        continue;
      }

      if pat_short_opt.is_match(&arg) {
        // ショートオプションなら
        let opts = arg[1..].chars();
        // println!("{opts:?}");

        // 一文字ずつ解析(e.g. -ab -> -a -b)
        for opt in opts {
          match opt {
            'h' => {
              print_help();
              return Ok(());
            }
            'v' => {
              print_version();
              return Ok(());
            }
            'r' => {
              is_resize_mode = true;
              continue;
            }
            _ => println!("Invalid option: -{opt}"),
          }
        }
      } else if pat_long_opt.is_match(&arg) {
        // ロングオプションなら
        match arg.as_str() {
          "--help" => {
            print_help();
            return Ok(());
          }
          "--version" => {
            print_version();
            return Ok(());
          }
          "--resize" => {
            is_resize_mode = true;
            continue;
          }
          _ => println!("Invalid option: {arg}"),
        }
      } else {
        // オプションまたはその引数でない引数(純粋な引数)を抽出
        non_option_args.push(arg);
      }
    }

    // 純粋な引数を引数として扱う
    non_option_args
  };
  debug_print(format!("{args:?}"));
  // debug_print(format!("{:?}", &resize_info));

  // 引数の数が不足していたら終了
  if args.len() < 3 {
    print_usage();
    return Ok(());
  }

  // 出力先ファイルのbasenameを取得
  let output_file_name = if args[1].ends_with(".pdf") {
    // Trim ".pdf" from out file name
    args[1].trim_end_matches(".pdf").to_string()
  } else {
    args[1].clone()
  };

  let input_images = &args[2..];

  // 出力先ファイルの作成 - 同名ファイルが存在する場合は終了
  let output_file = File::create_new(output_file_name.clone() + ".pdf")?;

  // PDFオブジェクトの作成
  let mut pdf = JpegToPdf::new();

  // jpegフォーマットか判定するクロージャ
  let is_jpeg = |path: &Path| {
    path.extension() == Some(OsStr::new("jpg")) || path.extension() == Some(OsStr::new("jpeg"))
  };

  // PDFオブジェクトにページ(画像)を追加
  for input_image_path in input_images {
    // 入力画像のパスをStringからPathに変換
    let input_image_path = Path::new(&input_image_path);

    // TODO: リサイズしないならjpgはfs::read -> Vec<u8>で読み込んだほうが遥かに高速なので分岐したい
    let jpeg_image = if resize_info.mode.eq(&ResizeMode::Original) {
      // リサイズなし

      if is_jpeg(&input_image_path) {
        // 入力画像がjpegファイル
        fs::read(input_image_path)?
      } else {
        // 入力画像がjpeg以外のファイル(要変換)
        let image = ImageReader::open(input_image_path)?.decode()?;
        convert_to_jpeg(image)?
      }
    } else {
      // リサイズあり

      // 入力画像をDynamicImage形式で読み込み
      let mut image = ImageReader::open(input_image_path)?.decode()?;

      // 画像をリサイズ
      image = resize_image(image, &resize_info)?;

      // 画像をjpegフォーマットに変換
      convert_to_jpeg(image)?
    };

    // 画像をページとしてPDFオブジェクトに追加
    pdf = pdf.add_image(jpeg_image);
    debug_print(format!("File added: {input_image_path:?}"));
  }

  // PDFオブジェクトをファイルに出力
  match pdf
    .strip_exif(true)
    .set_document_title(output_file_name)
    .create_pdf(&mut BufWriter::new(output_file))
  {
    Ok(_) => {}
    Err(e) => eprintln!("Failed to create PDF: {e}"),
  }

  Ok(())
}

fn print_version() {
  println!("{SELF_VERSION}");
}

fn debug_print(msg: impl Display) {
  if IS_DEBUG {
    eprintln!("{msg}");
  }
}

// 画像フォーマットの変換
fn convert_to_jpeg(image: DynamicImage) -> Result<Vec<u8>> {
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
fn resize_image(image: DynamicImage, r_info: &ResizeInfo) -> Result<DynamicImage> {
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

  let message = format!("resize_image(): Modes {:?} not implemented.", r_info.mode);
  Err(Error::msg(message))
}
