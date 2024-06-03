use anyhow::{Error, Result};
use image::io::Reader as ImageReader;
use jpeg_to_pdf::JpegToPdf;
use regex::Regex;
use std::{
  fs::{self, File},
  io::BufWriter,
  path::Path,
};

mod my_image;
use my_image::{convert_to_jpeg, debug::debug_print, ResizeInfo, ResizeMode};

mod mkpdf_docs;
use mkpdf_docs as docs;

fn main() -> Result<()> {
  let mut resize_info = ResizeInfo::new(ResizeMode::Original, None)?;

  // オプション解析と引数の取得
  let mut args = match option_handling(&mut resize_info) {
    Ok(args) => match args {
      Some(args) => args,
      None => return Ok(()),
    },
    Err(e) => return Err(e),
  };
  debug_print(format!("{args:?}"));
  // debug_print(format!("{:?}", &resize_info));

  // 引数の数が不足していたら終了
  if args.len() < 3 {
    docs::print_usage();
    return Ok(());
  }

  // 出力先ファイルのbasenameを取得
  let output_file_path = if args[1].ends_with(".pdf") {
    Path::new(&args[1])
  } else {
    Path::new({
      args[1].push_str(".pdf");
      &args[1]
    })
  };

  let input_images = &args[2..];

  // 出力先ファイルと同名のファイルがあったら終了
  if output_file_path.exists() {
    return Err(Error::msg(format!(
      "File '{}' is already exists.",
      output_file_path.to_str().unwrap()
    )));
  }

  // PDFを出力
  if let Err(e) = create_pdf(output_file_path, input_images, resize_info) {
    fs::remove_file(output_file_path)?;
    return Err(e);
  }

  Ok(())
}

// 非オプション引数を返す
fn option_handling(resize_info: &mut ResizeInfo) -> Result<Option<Vec<String>>, Error> {
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
      let mode = match arg.as_str() {
        "min" => ResizeMode::Min,
        "max" => ResizeMode::Max,
        _ => {
          if pat_resolution.is_match(&arg) {
            let res: Vec<u32> = arg.split('x').map(|x| x.parse().unwrap()).collect();
            resize_info.set_resoluton(Some((res[0], res[1])))?;
            ResizeMode::Custom
          } else {
            return Err(Error::msg(format!("Invalid resolution: {}", arg)));
          }
        }
      };

      resize_info.set_mode(mode);
      continue;
    }

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
            return Err(Error::msg(format!("Invalid option: -{opt}")));
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
        _ => {
          return Err(Error::msg(format!("Invalid option: {arg}")));
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
      let image = ImageReader::open(input_image)?.decode()?;
      images.push(image);
    }
    images
  };

  // リサイズ
  images = resize_info.resize_all(images);

  // 画像をページとしてPDFオブジェクトに追加
  for image in images {
    pdf = pdf.add_image(convert_to_jpeg(image)?);
    // debug_print(format!("File added: {input_image_path:?}"));
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
