# rspico_sw2key

rspico_sw2key は [RustyKeys](https://github.com/KOBA789/rusty-keys) を参考にし作成した自作キーボードのファームウェアです。

## 必要なもの (ハードウェア)
- Raspberry Pi Pico (main基板[ピンヘッダ実装済みのほうが楽])
- Raspberry Pi Pico (debug probe用[ピンヘッダ欲しい])
- タクトスイッチ (お好きなもの)
- ジャンパ線 (無限)
- ファストン端子 (無限)
- 熱収縮チューブ (無限)

## 環境開発 (Win)
下記と同じ方法で作成できる！！！<br>

https://rusty-keys.koba789.com/

## 入ってるもの
- hello.rs (Hello Worldします。)
- lchika.rs (Lチカします)
- serial_text.rs (シリアル通信でコンソールに文字列を出します) 
- __rp_key.rs (コレ、キーボード本体です)__

## 実行

``` console
$ cargo run --release --bin rp_key
```

# 参考
とても参考になりました！ありがとうございます。<br>
- https://github.com/KOBA789/rusty-keys