# mkpdf
  複数の画像ファイルからPDFファイルを作成するコマンドラインツール<br>
  The Command-line tool that can create a PDF file from multiple images

## 機能 - Function
  幅広い形式の画像ファイルを一括で読み込み、リサイズしてPDF出力することができます。<br>
  写真として保存している文書や学習ノートをPDF化したいときに便利です。<br>
  コマンドラインツールなので、スクリプトに組み込むのもいいかもしれません。<br>

  Loads image files of various formats in bulk, resize, and outputs.<br>
  Easily desitize your documents, study notes, and more.
  It may be a good idea to incorporate it into your scripts.

## 利用方法 - Usage
  `mkpdf [<--options|-o>] <output_file> <input_image1> [<input_image2>...]`<br>
  [ ]で囲まれた引数は任意です。また、利用可能なオプション等の詳細は`mkpdf -h`で確認できます。<br>

  Arguments surrounded by '[ ]' are optional.<br>
  You can see details such as available options by executing `mkpdf -h`.
