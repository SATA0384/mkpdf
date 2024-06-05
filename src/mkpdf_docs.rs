pub(crate) fn print_help() {
  print!("mkpdf - ");
  print_version();
  print_usage();

  println!("\nDescription: Create PDF file from multiple images.\n");

  println!("Options:");
  println!("  --help    | -h ) Print this help message and exit.");
  println!("  --resize <mode> ) Resize images. (Keeps aspect ratio)");
  println!("        -r <mode> ) <mode>: <width>x<height> ) Resize to specified size.");
  println!("                          : min ) Resize to fit the smallest one.");
  println!("                          : max ) Resize to fit the leargest one.");
  println!("  --version | -v ) Print version and exit.");
}

pub(crate) fn print_usage() {
  println!("Usage: mkpdf [<--options|-o>] <output_file> <input_image1> [<input_image2>...]");
  println!("  <--options|-o> : Options. Try 'mkpdf -h' to see verbosely.");
  println!("  <output_file>  : if not *.pdf, automatically append '.pdf'");
  println!("  <input_image>  : Supported formats are [JPEG|PNG|BMP|WEBP]");
}

const SELF_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn print_version() {
  println!("{SELF_VERSION}");
}
