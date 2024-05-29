use anyhow::{Error, Ok, Result};
use image::{io::Reader as ImageReader, DynamicImage};
use jpeg_to_pdf::JpegToPdf;
use std::{
  fs::{self, File},
  io::{BufWriter, Cursor},
  path::Path,
};

// Usage: mkpdf <outfile>.pdf <source1>.jpg [<source2>.jpg ...]

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();

  // Quit if no enough args
  if args.len() < 3 {
    eprintln!("Usage: mkpdf <outfile> <source1> [<source2>...]");
    eprintln!("       <outfile> : if not *.pdf, automatically append '.pdf'");
    eprintln!("       <sources> : Supported formats are [.jpg|.png|.bmp]");
    return Ok(());
  }

  let mut out_file_name = args[1].clone();
  if out_file_name.ends_with(".pdf") {
    // Trim ".pdf" from out file name
    out_file_name = out_file_name.trim_end_matches(".pdf").to_string();
  }

  let src_images: Vec<String> = args[2..].to_vec();

  // Create file - Quit if already exist same name file
  let out_file = File::create_new(out_file_name.clone() + ".pdf")?;

  // Create pdf object
  let mut pdf = JpegToPdf::new();

  // Add images to pdf
  for src_image_path in src_images {
    let src_image_path = Path::new(&src_image_path);

    let jpg_image;
    if &*src_image_path.extension().unwrap() == "jpg" {
      // Load image from jpeg file
      jpg_image = fs::read(src_image_path)?;
    } else {
      // Load image from other than jpeg file
      let image = ImageReader::open(src_image_path)?.decode()?;
      jpg_image = convert_to_jpeg(image)?;
    }
    pdf = pdf.add_image(jpg_image);
  }

  // Export to pdf file
  pdf
    .strip_exif(true)
    .set_document_title(out_file_name)
    .create_pdf(&mut BufWriter::new(out_file))?;

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
