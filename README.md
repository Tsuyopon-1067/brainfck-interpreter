# brainfck-interpreter

Brainf\*ck のインタプリタです．

## 使用方法

- 通常のプログラム実行

```bash
./brainfck-interpreter ./program.bf
```

- 入力を伴うプログラム実行

```bash
./brainfck-interpreter -i ./program.bf
```

- デバッグモード
  - 本来は文字として出力される出力を 10 進数の数値で出力します

```bash
./brainfck-interpreter -d ./program.bf
```

## 仕様

### 命令

- `>`
  - ポインタをインクリメントします．オーバーフローは許容し，255 からインクリメントしたら 0 になります．
- `<`
  - ポインタをデクリメントします．アンダーフローは許容し，0 からデクリメントすると 255 になります．
- `+`
  - ポインタが指す値をインクリメントします．
- `-`
  - ポインタが指す値をデクリメントします．
- `.`
  - ポインタが指す値を出力します．
- `,`
  - 入力から 1 バイト読みこんでポインタが指すさきに代入します．
- `[`
  - ポインタが指す値が 0 なら対応する`]`の直後にジャンプします．
- `]`
  - ポインタが指す値が 0 でなければ対応する`[`の直後にジャンプします．

### メモリ

- 3000Byte
