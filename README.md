# echobard

A typing game where you echo poems, verses, and prose back — scored on accuracy and timing.

お題の詩歌・散文を入力して再現するタイピングゲーム。一致度とタイムでスコアを競う。

## コンセプト

- 入力方式(物理キーボード、IME、ソフトウェアキーボード、音声入力など)は問わない。
  ゲームが判定するのは **入力確定後の文字列** であり、そこに至る過程は問わない。
- お題は詩歌を中心に据えるが、原理的には文字種・言語・文体を限定しない
  (和歌・俳句・漢詩・英詩・ハングル・散文なども将来的に対象になりうる)。
- 「速く正確に打つ」競技性よりも、「お題の言葉をなぞり、味わう」体験を重視する。

## 遊び方(現時点の想定)

1. お題(詩歌の1行/一首/一句)が表示される。
2. プレイヤーが入力欄に文字を入力し、確定する。
3. 確定した文字列とお題文字列を比較し、一致度(編集距離ベース)を算出する。
4. 入力開始から確定までの時間を計測し、時間スコアを算出する。
5. 一致度と時間スコアを合成し、その行のスコアとする。
6. 詩歌の全行を終えると、合計スコアが表示される。

## 技術スタック

- ゲームエンジン: [Godot](https://godotengine.org/)
- コアロジック: [Rust](https://www.rust-lang.org/) ([gdext](https://github.com/godot-rust/gdext) 経由で Godot に統合)

## リポジトリ構成(予定)

クレートの配置は `slide-roguelike` の構成方針を踏襲する一方、Godot プロジェクトのディレクトリ名は役割が伝わるよう変更している(経緯は [ADR-0002](docs/adr/0002-directory-layout.md) を参照)。

```
Cargo.toml       # ワークスペース定義
echobard-core/   # ゲームロジック本体。Godot に依存しない
echobard-ext/    # gdext を用いた Godot 向けの薄いブリッジ
godot/           # Godot プロジェクト本体(project.godot はここ)
  assets/
    poems/       # お題データ(詩歌)。ライセンス・出典は別途管理
  scenes/
  echobard.gdextension
target/          # cargo build の出力(gitignore対象)。.gdextension から相対参照する
docs/
  requirements.md
  adr/           # Architecture Decision Records
```

デスクトップ向けのビルド成果物は `target/debug/` を `.gdextension` から `res://../target/debug/...` の形で相対参照する(Android は例外的に Godot プロジェクト内 `libs/android/` にライブラリを同梱する)。

## ステータス

構想・設計段階。詳細な要件は [docs/requirements.md](docs/requirements.md)、設計判断の経緯は [docs/adr/](docs/adr/)(運用方針は [docs/adr/README.md](docs/adr/README.md))を参照。

## ライセンス

コードは [MIT License](LICENSE) の下で公開する。

お題データ(詩歌)については、コードとは別にライセンス・出典を管理する方針(パブリックドメイン作品を中心に収録予定)。詳細は今後 `assets/poems/` 配下に追記する。

## 名前の由来

`echobard` は「お題の言葉をこだま(echo)のように打ち返す」体験と、「詩歌を詠む者(bard)」のイメージを組み合わせた造語。
