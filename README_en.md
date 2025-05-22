# mkpdf
  The Command-line tool that can create a PDF file from multiple images

  [README: 日本語](https://github.com/SATA0384/mkpdf/blob/master/README.md)

## Functionality
  Loads image files of various formats in bulk, resize, and outputs.<br>
  Easily desitize your documents, study notes, and more.
  It may be a good idea to incorporate it into your scripts.

## How to Install
  `cargo` and `git`[^1] are required to install. Install these first.
  [cargo: Rust - Official Website](https://www.rust-lang.org/tools/install)

  There are 2 methods to install.

  1. Download installer and execute it(Recommended[^2])<br>
    Download and execute a script file `install.sh` in this repository root.<br>
    ```/bin/sh /path/to/install.sh```

  2. ```cargo install --path /path/to/repo``` after cloning repository.<br>

  [^1]: In the case of method 1
  [^2]: Remove source codes, build objects automatically.

## Usage
  `mkpdf [<--options|-o>] <output_file> <input_image1> [<input_image2>...]`<br>

  Arguments surrounded by '[ ]' are optional.<br>
  You can see details such as available options by executing `mkpdf -h`.
