# tools.nave-wata.net

1. npm をインストールする: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Tailwind CSS CLI をインストールする: https://tailwindcss.com/docs/installation
3. プロジェクトのルートで以下のコマンドを実行して、Tailwind CSS コンパイラを起動する:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

## 開発

プロジェクトのルートで以下のコマンドを実行して、Dioxus 開発サーバーを起動する:

```bash
dx serve --addr 0.0.0.0 --platform web
```

- ブラウザで http://localhost:8080 を開く

## ビルド

リリース用のビルドを行うには、以下のコマンドを実行する:

```bash
dx bundle --platform web --ssg --release
```

このコマンドは静的サイトジェネレーション（SSG）を使用して、Web プラットフォーム向けのリリースビルドを作成します。
