# clip-translator

Windows clipboard daemon that auto-translates EN⇄JA via remote Ollama and shows results in native notifications.

## Features

- クリップボード監視 (テキストをコピーするだけで自動翻訳)
- LLM (Ollama) による言語自動判定 + 英⇄日翻訳
- OS ネイティブ通知で結果表示 (アクションセンターに残る)
- システムトレイ常駐 + 右クリックメニュー (Pause / Resume / Quit)
- グローバルホットキー `Ctrl+Alt+P` で Pause/Resume トグル

## Requirements

- Windows 10/11
- Rust 1.85+ (edition 2024 を使用)
- リモート PC で動作中の Ollama サーバー (LAN 内アクセス可能)
- Ollama にロード済みの翻訳対応モデル (例: `llama3.1`, `gpt-oss:20b` など)

## Build

```bash
git clone https://github.com/<user>/clip-translator.git
cd clip-translator
cargo build --release
```

## Usage

```bash
clip-translator.exe --endpoint http://192.168.0.50:11434 --model gpt-oss:20b
```

| 引数 | 必須 | 説明 |
|---|---|---|
| `--endpoint` | ✓ | Ollama サーバーのエンドポイント |
| `--model` | ✓ | 使用する Ollama モデル名 |

### 操作

- **テキストをコピー** → 数秒で翻訳結果が通知に出る
- **トレイアイコン右クリック** → Pause / Resume / Quit
- **`Ctrl+Alt+P`** → Pause/Resume トグル

### 自動起動

スタートアップフォルダ (`shell:startup`) に `clip-translator.exe` のショートカットを配置。
ショートカットの「リンク先」に引数を含める:
```
"C:\path\to\clip-translator.exe" --endpoint http://192.168.0.50:11434 --model gpt-oss:20b
```

## License

MIT
