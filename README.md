# mkpdf
  複数の画像ファイルからPDFファイルを作成するコマンドラインツール<br>

  [README: English](https://github.com/SATA0384/mkpdf/edit/master/README_en.md)

## 機能
  幅広い形式の画像ファイルを一括で読み込み、リサイズしてPDF出力することができます。<br>
  写真として保存している文書や学習ノートをPDF化したいときに便利です。<br>
  コマンドラインツールなので、スクリプトに組み込むのもいいかもしれません。<br>

## インストール方法
  インストールには`cargo`と`git`[^1]が必要です。事前にインストールしておいてください。
  [cargo: Rust - 公式ページ](https://www.rust-lang.org/ja/tools/install)

  インストール方法は2種類あります。
  1. インストールスクリプトをダウンロードし、実行する方法(推奨[^2])<br>
    本リポジトリのルートにある`install.sh`スクリプト単体をダウンロードし、実行してください。<br>
    ```/bin/sh /path/to/install.sh```

  2. リポジトリをクローンし、```cargo install --path /path/to/repo```でインストールする方法<br>

  [^1]: 方法1の場合
  [^2]: ソースコードやビルド時の中間ファイル等が自動的に削除されます。

## 使用方法
  `mkpdf [<--options|-o>] <output_file> <input_image1> [<input_image2>...]`<br>
  [ ]で囲まれた引数は任意です。また、利用可能なオプション等の詳細は`mkpdf -h`で確認できます。<br>
