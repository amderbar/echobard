# ADR-0002: ディレクトリ構成(Godot プロジェクトと Rust クレートの配置)

- Status: Accepted
- Date: 2026-08-11

## Context

echobard は `slide-roguelike` と同様、Godot プロジェクトと Rust クレート(gdext 経由で
統合するコアロジック)が同居するモノレポ構成を取る。ディレクトリ構成の検討にあたり、
まず参考のため `slide-roguelike` の実際の構成を確認した。

```
slide-roguelike/                  # リポジトリルート
  Cargo.toml                      # ワークスペース定義。members にクレートをフラット列挙
  slide-roguelike-core/           # コアロジック(Godot 非依存)
  slide-roguelike-ext/            # gdext ブリッジ
  gdext-logger/                   # ロギング用の補助クレート
  slide-roguelike/                # Godot プロジェクト本体(project.godot はここ)
    assets/
    scenes/
    slide_roguelike_ext.gdextension
  target/                         # cargo build の出力(デフォルト位置のまま)
```

主な特徴は以下の通り。

- クレートはワークスペース直下に、リポジトリ名を接頭辞にしたフルネームで
  フラットに配置されている(`crates/` のようなサブディレクトリはない)。
- Godot プロジェクトは、**リポジトリ名と同名のディレクトリ**
  (`slide-roguelike/slide-roguelike/`)に置かれている。
- ビルド成果物はデフォルトの `target/` をそのまま使い、`.gdextension` から
  `res://../target/debug/...` の形で相対参照している(`CARGO_TARGET_DIR` の変更なし)。
- assets は Godot プロジェクトディレクトリの内側に置かれている。

echobard は `slide-roguelike` の派生・後継ではなく別プロジェクトであるため、
これらの構成をそのまま踏襲する必然性はない。それぞれの選択が「Godot/Cargo の制約で
そうせざるを得ないもの」か「単なる前例」かを切り分けた上で、echobard としての構成を
改めて決定する。

## Decision

### 踏襲するもの

- **クレートのフラット配置**: `echobard-core` / `echobard-ext` をワークスペース直下に
  フラット配置する。クレート数が現時点で 2〜3 程度(将来的に増える可能性はあるが、
  `slide-roguelike` も 3 クレートで収まっている)であり、`crates/` サブディレクトリでまとめる
  恩恵(ルートの見通しの良さ)よりも、1 階層減るシンプルさを優先した。
  クレート数が大きく増えてルート直下が煩雑になった場合は、改めて `crates/` への移行を
  検討する。
- **assets を Godot プロジェクト内に置くこと**: これは前例に合わせたというより、
  Godot の制約に基づく。Godot はインポート設定(`.import` ファイル・UID キャッシュなど)を
  プロジェクト内(`res://` で参照できる範囲)のリソースに対して管理するため、
  asset 本体をプロジェクトディレクトリの外に置くとエディタのインポートパイプラインが
  正しく機能しない。
- **ビルド成果物をデフォルトの `target/` に出力し、`.gdextension` から相対参照すること**:
  `CARGO_TARGET_DIR` を変更する積極的な理由がないため、Cargo の標準動作に従う。

### 変更するもの

- **Godot プロジェクトのディレクトリ名**: `slide-roguelike` はリポジトリ名と同名のディレクトリ
  (`slide-roguelike/slide-roguelike/`)を Godot プロジェクトに使っているが、この方式は
  `cd` した際にディレクトリ名が重複して紛らわしい。

  ゲームの表示名(タイトルバーやエクスポート時の名前)は `project.godot` の `config/name`
  設定で決まり、ディレクトリ名とは独立しているため、「フォルダ名をゲーム名・プロジェクト名と
  揃えなければならない」という技術的制約はない。単一言語の Godot プロジェクト単体の
  リポジトリであれば「リポジトリ名 = プロジェクト名 = フォルダ名」を揃える方が素直だが、
  echobard のように Rust クレートと同居するモノレポでは、フォルダの役割(Godot 側の実体で
  あること)が伝わる名前を優先し、**`godot/`** を採用する。

## Consequences

- `slide-roguelike` の構成に慣れている場合、Godot プロジェクトの場所が
  `<repo>/<repo-name>/` ではなく `<repo>/godot/` になる点に注意が必要。
- クレート数が将来大きく増えた場合、`crates/` サブディレクトリへの移行を再検討する
  余地を残す(そのときは改めて ADR を起こす)。
- assets の配置(Godot プロジェクト内)は Godot の制約に基づく決定であり、
  他のゲームエンジンを検討する場合はこの前提が崩れる可能性がある
  (現時点では Godot 採用が前提のため考慮しない)。
