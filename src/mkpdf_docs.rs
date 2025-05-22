pub(crate) fn print_help() {
  print!("mkpdf - ");
  print_version();
  print_usage();

  println!("\nDescription: Create a PDF file from multiple images.\n");

  println!("Supported formats:");
  println!("  AVIF, BMP, GIF, ICO, JPEG, PNG, TIFF, WebP,");
  println!("  DDS, EXR, Farbfeld, HDR, PNM, QOI, TGA\n");

  println!("Options:");
  println!("  --resize <size> | -r <size> ) Resize images. (Keeps aspect ratio)");
  println!("      <size>: <w>x<h> ) Resize to specified size.(<w>: Width, <h>: Height)");
  println!("      <size>: min     ) Resize to fit the smallest one.");
  println!("      <size>: max     ) Resize to fit the leargest one.\n");

  println!("  --resize-filter <mode> ) Specify resize filter");
  println!("      <mode>: nearest  ) Nearest filter. Fast but low quality.");
  println!("      <mode>: linear   ) Linear filter. Mid-fast and good quality. (Default)");
  println!("      <mode>: cubic    ) Cubic filter. Little-slow but better quality.");
  println!("      <mode>: lanczos  ) Lanczos filter (3 windows). Slow but best quality.");
  println!("      <mode>: gaussian ) Gaussian filter. Slow but smooth. (Looks a little blurred)\n");

  println!("  --help    | -h ) Print this help message and exit.");
  println!("  --version | -v ) Print version and exit.");

  println!("GitHub repository: https://github.com/SATA0384/mkpdf");
}

pub(crate) fn print_usage() {
  println!("Usage: mkpdf [<--options|-o>] <output_file> <input_image1> [<input_image2>...]");
  println!("  <--options|-o> : Options. Try `mkpdf -h` to see verbosely.");
  println!("  <output_file>  : if not *.pdf, automatically append \".pdf\"");
  println!("  <input_image>  : You can see supported formats with `mkpdf -h`");
}

const SELF_VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn print_version() {
  println!("{SELF_VERSION}");
}
