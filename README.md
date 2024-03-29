# CSV の指定列を指定した値で上書きする

## 注意

Rust 勉強中の自習課題です。

クローン、使用は自己責任でお願い致します。

## 仕様

まずは素朴な実装から考える。

- コマンドライン引数は全部条件指定に使ってしまう。
- 入力は標準入力からのみ
- 出力は標準出力のみ

sample : tests/members.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
15-0,Bob,18
n-09,Jen,48
```

引数にてカンマ区切りで左辺がフィールド名、右辺が上書きする値を指定できる
```sh
<tests/members.csv cargo run -- "age,-" >tests/expect-ageless.csv
```
このコマンドラインの出力は下記のようになる。

sample : tests/expect-ageless.csv
```csv
id,name,age
10-1,John,-
10-2,Ken,-
15-0,Bob,-
n-09,Jen,-
```

## 背景的コメント

CSV 内のコントロールカラムなどの値を一括で特定のフラグ("u"とか"n"とか)に書き換えるのに使えればよい。
