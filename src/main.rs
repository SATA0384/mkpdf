use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use debug_print::{
  debug_eprint as deprint, debug_eprintln as deprintln, debug_print as dprint,
  debug_println as dprintln,
};
use image::{imageops::FilterType, ImageReader};
use jpeg_to_pdf::JpegToPdf;
use regex::Regex;
use std::{
  fs::{self, File},
  io::BufWriter,
  path::Path,
};

mod mkpdf_image;
use mkpdf_image::{convert_to_jpeg, ResizeInfo, ResizeMode};

mod mkpdf_docs;
use mkpdf_docs as docs;

fn main() -> Result<()> {
  let mut resize_info = ResizeInfo::new(ResizeMode::Original, None, None)?;

  // オプション解析と引数の取得
  let mut args = match option_handling(&mut resize_info)? {
    Some(args) => args,
    None => return Ok(()),
  };
  dprintln!("args = {:?}", args);
  dprintln!("resize_info = {:#?}", &resize_info);

  // 引数の数が不足していたら終了
  if args.len() < 3 {
    docs::print_usage();
    return Ok(());
  }

  // 出力先ファイルパスを取得
  let output_file_path = Path::new({
    // .pdf拡張子がついていなければ付加
    if !args[1].ends_with(".pdf") {
      args[1].push_str(".pdf");
    }
    &args[1]
  });

  let input_images = &args[2..];

  // 出力先ファイルと同名のファイルがあったら終了
  if output_file_path.exists() {
    return Err(anyhow!(
      "File '{}' is already exists.",
      output_file_path.to_str().unwrap()
    ));
  }

  // PDFを出力
  if let Err(e) = create_pdf(output_file_path, input_images, resize_info) {
    fs::remove_file(output_file_path)?;
    return Err(e);
  }

  Ok(())
}

// オプションを解析し、非オプション引数を返す
fn option_handling(resize_info: &mut ResizeInfo) -> Result<Option<Vec<String>>> {
  // オプション含むシェル引数を取得
  let raw_args: Vec<String> = std::env::args().collect();

  // オプション解析
  let pat_short_opt = Regex::new(r"^-[0-9|a-z|A-Z]").unwrap();
  let pat_long_opt = Regex::new(r"^--[0-9|a-z|A-Z]").unwrap();
  let mut non_option_args = Vec::new();
  let mut is_resize_mode = false;
  let mut is_resize_filter_mode = false;

  for arg in raw_args {
    /* オプション引数を取得 */
    if is_resize_mode {
      // -r,--resize: リサイズモード

      let pat_resolution = Regex::new("^[1-9][0-9]{2}[0-9]?x[1-9][0-9]{2}[0-9]?$").unwrap();
      let mode = match arg.as_str() {
        "min" => ResizeMode::Min,
        "max" => ResizeMode::Max,
        _ => {
          // <w>x<h>形式 - 解像度の指定
          if pat_resolution.is_match(&arg) {
            let res: Vec<u32> = arg.split('x').map(|xy| xy.parse().unwrap()).collect();
            resize_info.set_resoluton(Some((res[0], res[1])))?;
            ResizeMode::Custom
          } else {
            return Err(anyhow!("Invalid resolution: {}", arg));
          }
        }
      };

      resize_info.set_mode(mode);
      is_resize_mode = false;
      continue;
    } else if is_resize_filter_mode {
      // --resize_filter_mode: リサイズ時のフィルターモード

      let filter = match arg.as_str() {
        "nearest" | "low" | "fast" => FilterType::Nearest,
        "linear" | "good" | "triangle" => FilterType::Triangle,
        "lanczos" | "best" | "slow" => FilterType::Lanczos3,
        "cubic" | "better" | "catmullrom" => FilterType::CatmullRom,
        "gaussian" | "blur" => FilterType::Gaussian,
        _ => {
          return Err(anyhow!("Invalid resize filter: {}", arg));
        }
      };

      resize_info.set_filter(Some(filter));
      is_resize_filter_mode = false;
      continue;
    }

    /* オプションを取得 */
    if pat_short_opt.is_match(&arg) {
      // ショートオプションなら
      let opts = arg[1..].chars();

      // 一文字ずつ解析(e.g. -ab -> -a -b)
      for opt in opts {
        match opt {
          'h' => {
            docs::print_help();
            return Ok(None);
          }
          'v' => {
            docs::print_version();
            return Ok(None);
          }
          'r' => {
            is_resize_mode = true;
            continue;
          }
          _ => {
            return Err(anyhow!("Invalid option: -{opt}"));
          }
        }
      }
    } else if pat_long_opt.is_match(&arg) {
      // ロングオプションなら
      match arg.as_str() {
        "--help" => {
          docs::print_help();
          return Ok(None);
        }
        "--version" => {
          docs::print_version();
          return Ok(None);
        }
        "--resize" => {
          is_resize_mode = true;
          continue;
        }
        "--resize-filter" => {
          is_resize_filter_mode = true;
          continue;
        }
        _ => {
          return Err(anyhow!("Invalid option: {arg}"));
        }
      }
    } else {
      // オプションまたはその引数でない引数(純粋な引数)を抽出
      non_option_args.push(arg);
    }
  }

  // 純粋な引数を引数として扱う
  Ok(Some(non_option_args))
}

fn create_pdf(
  output_file_path: &Path,
  input_images: &[String],
  resize_info: ResizeInfo,
) -> Result<()> {
  // 出力先ファイルの作成 - 同名ファイルが存在する場合は終了
  let output_file = File::create_new(output_file_path)?;

  // PDFオブジェクトの作成
  let mut pdf = JpegToPdf::new();

  // 画像を一括で読み込み
  let mut images = {
    let mut images = Vec::new();
    for input_image in input_images {
      dprint!("Opening image: {input_image}");
      let image = ImageReader::open(input_image)?.decode()?;
      dprintln!(" - Done.");
      images.push(image);
    }
    images
  };

  // リサイズ
  images = resize_info.resize_all(images);

  // 画像をページとしてPDFオブジェクトに追加
  for image in images {
    pdf = pdf.add_image(convert_to_jpeg(image)?);
  }

  // 文書タイトルをPDFのファイル名にセット
  let title = match output_file_path.file_name() {
    Some(basename) => basename.to_str().unwrap().trim_end_matches(".pdf"),
    None => "Document",
  };

  // PDFオブジェクトをファイルに出力
  pdf
    .strip_exif(true)
    .set_document_title(title)
    .create_pdf(&mut BufWriter::new(output_file))?;

  Ok(())
}
