use anyhow::{Error, Result};
use jpeg_to_pdf::JpegToPdf;
use std::{
  fs::{self, File},
  io::BufWriter,
};
// Usage: mkpdf <outfile>.pdf <source1>.jpg [<source2>.jpg ...]

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();
  // println!("{args:?}");

  // Quit if no enough args
  if args.len() < 3 {
    return Err(Error::msg(
      "Usage: mkpdf <outfile>.pdf <source1>.jpg [<source2>.jpg ...]",
    ));
  }

  let mut out_file_name = args[1].clone();
  if out_file_name.ends_with(".pdf") {
    // Trim ".pdf" from out file name
    out_file_name = out_file_name.trim_end_matches(".pdf").to_string();
  }

  let src_images: Vec<String> = args[2..].to_vec();

  // Quit if already exist same name file
  let out_file = File::create_new(out_file_name.clone() + ".pdf")?;

  // Create pdf object
  let mut pdf = JpegToPdf::new();

  // Add pages to pdf
  for src_image_path in src_images {
    if !&src_image_path.ends_with(".jpg") {
      return Err(Error::msg("Supports only '.jpg' file."));
    }
    pdf = pdf.add_image(fs::read(src_image_path)?);
  }

  // Export to pdf file
  pdf
    .strip_exif(true)
    .set_document_title(out_file_name)
    .create_pdf(&mut BufWriter::new(out_file))?;

  Ok(())
}
