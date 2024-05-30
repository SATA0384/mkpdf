use anyhow::Result;
use image::{io::Reader as ImageReader, DynamicImage};
use jpeg_to_pdf::JpegToPdf;
use std::{
  ffi::OsStr,
  fs::{self, File},
  io::{BufWriter, Cursor},
  path::Path,
};

fn print_usage() {
  eprintln!("Usage: mkpdf <output_file> <input_image1> [<input_image2>...]");
  eprintln!("  <output_file> : if not *.pdf, automatically append '.pdf'");
  eprintln!("  <input_image> : Supported formats are [.jpg|.png|.bmp]");
}

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();

  // Quit if no enough args
  if args.len() < 3 {
    print_usage();
    return Ok(());
  }

  // Get output file basename
  let output_file_name = if args[1].ends_with(".pdf") {
    // Trim ".pdf" from out file name
    args[1].trim_end_matches(".pdf").to_string()
  } else {
    args[1].clone()
  };

  let input_images = &args[2..];

  // Create file - Quit if already exist same name file
  let output_file = File::create_new(output_file_name.clone() + ".pdf")?;

  // Create pdf object
  let mut pdf = JpegToPdf::new();

  // Add images to pdf
  for input_image_path in input_images {
    // Convert image path from String to Path
    let input_image_path = Path::new(&input_image_path);

    let jpg_image = if &input_image_path.extension() == &Some(OsStr::new("jpg")) {
      // Load image from jpeg file
      fs::read(input_image_path)?
    } else {
      // Load image from other than jpeg file
      let image = ImageReader::open(input_image_path)?.decode()?;
      convert_to_jpeg(image)?
    };

    pdf = pdf.add_image(jpg_image);
  }

  // Export to pdf file
  pdf
    .strip_exif(true)
    .set_document_title(output_file_name)
    .create_pdf(&mut BufWriter::new(output_file))?;

  Ok(())
}

fn convert_to_jpeg(image: DynamicImage) -> Result<Vec<u8>> {
  // Prepair buffer for jpeg image
  let mut jpg_buf = Vec::new();
  let mut seekable_jpg_buf = &mut Cursor::new(&mut jpg_buf);

  // Write to buffer
  image
    .into_rgb8()
    .write_to(&mut seekable_jpg_buf, image::ImageFormat::Jpeg)?;

  Ok(jpg_buf)
}
