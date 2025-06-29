# WebTools プロジェクトガイドライン

このガイドラインは、WebTools プロジェクトの概要と開発ガイドラインを提供します。詳細な情報は `/web/docs` ディレクトリの各ドキュメントを参照してください。

## プロジェクト概要

WebTools は、様々なウェブツールを提供するRustベースのウェブアプリケーションです。Dioxusフレームワークを使用し、Tailwind CSSでスタイリングされています。

## プロジェクト構造

```
/web
├── src/                  # ソースコードディレクトリ
│   ├── components/       # 再利用可能なUIコンポーネント
│   ├── constants/        # 定数定義
│   ├── libs/             # ユーティリティライブラリ
│   ├── routes/           # ルーティング定義とページコンポーネント
│   ├── components.rs     # コンポーネントのエクスポート
│   ├── constants.rs      # 定数のエクスポート
│   ├── libs.rs           # ライブラリのエクスポート
│   ├── main.rs           # アプリケーションのエントリーポイント
│   └── routes.rs         # ルーティング設定
├── assets/               # 静的アセット（画像、フォントなど）
├── resources/            # リソースファイル
└── docs/                 # ドキュメント
```

詳細な構造については `/web/docs/project_structure.md` を参照してください。

## コーディング規約

### 一般原則

1. **明確さを優先**: 短いコードよりも読みやすいコードを優先します
2. **コメント**: 複雑なロジックには適切なコメントを付けます
3. **単一責任**: 各関数・コンポーネントは単一の責任を持つようにします
4. **DRY (Don't Repeat Yourself)**: コードの重複を避けます

### Rust コーディング規約

- `cargo fmt` を使用して一貫したフォーマットを維持します
- 命名規則:
  - **型名** (構造体、列挙型、トレイト): パスカルケース (`SimpleCalculator`)
  - **変数と関数**: スネークケース (`calculate_result`)
  - **定数**: 大文字のスネークケース (`MAX_LENGTH`)

### Dioxus コンポーネント規約

コンポーネントは以下の構造に従います:

```rust
pub(crate) fn ComponentName(cx: Scope) -> Element {
    // 状態の初期化
    let state = use_state(cx, || initial_value);

    // イベントハンドラ
    let handle_event = move |_| {
        // イベント処理
    };

    // UI レンダリング
    cx.render(rsx! {
        div {
            class: "style-classes",
            // コンポーネントの内容
        }
    })
}
```

詳細なコーディング規約については `/web/docs/coding_guidelines.md` を参照してください。

## 新機能の追加方法

新しいツールを追加するには、以下の手順に従います:

1. ルートファイルの作成（ツールの実装）
2. ルーティング設定の追加
3. カードコンポーネントの作成
4. トップページへのカード追加
5. アイコンとOGP画像の追加
6. 必要に応じたグリッドの追加
7. 使い方コンポーネントの追加

すべてのツールには使い方説明を追加する必要があります。`Usage` コンポーネントを使用して、一貫したスタイルと構造で使い方を表示してください。

詳細な手順については `/web/docs/adding_new_routes.md` を参照してください。

## ビルドとテスト

### 開発環境

```bash
# 開発サーバーの起動
docker compose up -d

# 開発サーバーを再起動する場合
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web
```

### コードフォーマットとリンター

```bash
# Rustのコードフォーマット
docker compose exec web cargo fmt

# リンターの実行
docker compose exec web cargo clippy
```

### テスト

```bash
# テストの実行
docker compose exec web cargo test
```

コード変更後は必ずテストを実行して、機能が正しく動作することを確認してください。また、新機能の追加や既存機能の変更を行う際には、適切なテストコードも作成してください。テストコードは対応するモジュールと同じディレクトリに配置し、機能の正常動作と異常系の処理を検証するようにしてください。

### 開発ワークフロー

コード変更後は、以下の手順に従って品質を確保してください：

1. **コードフォーマッターの実行**:
   ```bash
   docker compose exec web cargo fmt
   ```
   - フォーマッターによって自動的に修正されるスタイルの問題があります
   - フォーマッターが出力するすべての警告を解決してください

2. **リンターの実行**:
   ```bash
   docker compose exec web cargo clippy
   ```
   - リンターが検出した警告やエラーをすべて解決してください
   - 警告を無視せず、コードの品質向上のためにすべての警告に対処してください

3. **テストの実行**:
   ```bash
   docker compose exec web cargo test
   ```
   - すべてのテストが成功することを確認してください

4. **ビルドの確認**:
   ```bash
   docker compose exec web ./bundle.sh
   ```
   - ビルドが正常に完了することを確認してください
   - ビルド時に出力されるすべての警告を解決してください
   - 警告を放置すると、将来的にエラーになる可能性があります

これらのステップを順番に実行し、各ステップで検出された問題をすべて解決してから次のステップに進むことで、コードの品質と安定性を確保できます。

### 本番ビルド

```bash
# 本番用にビルド
docker compose exec web ./bundle.sh
```

## ドキュメント

各ツールの仕様書は `/web/docs/pages/` ディレクトリに配置されています。新しいツールを追加する場合は、対応する仕様書も作成してください。また、既存の機能を変更した場合は、関連するドキュメントも必ず更新してください。ドキュメントの更新は、コードの変更と同じプルリクエストに含めるようにし、コードとドキュメントの一貫性を維持してください。

## CDK (AWS Cloud Development Kit) ガイドライン

WebTools プロジェクトでは、AWS CDKを使用してクラウドインフラストラクチャをコードとして管理しています。CDKコードは `/cdk` ディレクトリに配置され、TypeScriptで記述されています。

### CDK プロジェクト構造

```
/cdk
├── bin/                  # CDKアプリケーションのエントリーポイント
│   └── app.ts           # メインアプリケーション定義
├── lib/                 # CDK構成要素の実装
│   ├── constructs/      # 再利用可能なCDK構成要素
│   │   ├── cdn/         # CloudFront関連の構成要素
│   │   ├── domain/      # ドメイン・証明書関連の構成要素
│   │   ├── shared/      # 共通の構成要素
│   │   └── storage/     # ストレージ関連の構成要素
│   ├── interfaces/      # TypeScript型定義
│   └── stacks/          # CDKスタック定義
├── test/                # テストファイル
├── utils/               # ユーティリティ関数
├── assets/              # デプロイ用アセット
├── .env                 # 環境変数設定
└── docs/                # 詳細ドキュメント
```

### CDK アーキテクチャ原則

1. **スタック分離**: 機能ごとにスタックを分離し、依存関係を明確にします
   - `DomainStack`: Route53ホストゾーンとSSL証明書
   - `StorageStack`: S3バケットによる静的サイトストレージ
   - `CdnStack`: CloudFrontディストリビューションとDNSレコード

2. **構成要素の再利用**: 共通機能は `constructs/` ディレクトリで再利用可能な構成要素として実装します

3. **型安全性**: TypeScriptの型システムを活用し、`interfaces/` ディレクトリで型定義を管理します

4. **環境分離**: 環境変数を使用して設定を外部化し、環境ごとの差異を管理します

### CDK コーディング規約

#### 命名規則

- **スタック名**: パスカルケース + "Stack" サフィックス (`DomainStack`)
- **構成要素名**: パスカルケース + "Construct" サフィックス (`SecurityHeadersConstruct`)
- **変数・関数**: キャメルケース (`domainName`, `requireEnv`)
- **定数**: 大文字のスネークケース (`STACK_PREFIX`)

#### ファイル構成

- **スタックファイル**: `/lib/stacks/` に配置し、単一責任の原則に従います
- **構成要素ファイル**: `/lib/constructs/` にドメインごとに分類して配置します
- **インターフェース**: `/lib/interfaces/` に対応するドメイン名で配置します

#### コメント規約

```typescript
/**
 * 構成要素やスタックの説明
 * 
 * より詳細な説明や使用方法、注意点などを記載
 * 複数行にわたる場合は適切に改行します
 */
export class ExampleConstruct extends Construct {
  /** プロパティの説明 */
  public readonly exampleProperty: string;

  /**
   * コンストラクタの説明
   * 
   * @param scope 親構成要素
   * @param id 構成要素ID
   * @param props 構成要素のプロパティ
   */
  constructor(scope: Construct, id: string, props: ExampleProps) {
    super(scope, id);
    // 実装
  }
}
```

### 開発ワークフロー

#### 環境設定

```bash
# CDKディレクトリに移動
cd cdk

# 依存関係のインストール
npm install

# 環境変数の設定
cp .env.example .env
# .envファイルを編集して必要な値を設定
```

#### 開発・テスト

```bash
# TypeScriptのコンパイル
npm run build

# テストの実行
npm test

# CDK構文チェック
npx cdk synth

# 差分確認
npx cdk diff
```

#### デプロイ

```bash
# 全スタックのデプロイ
npx cdk deploy --all

# 特定スタックのデプロイ
npx cdk deploy ToolsNaveWataNet-DomainStack
```

### 新機能追加ガイドライン

新しいAWSリソースや機能を追加する場合は、以下の手順に従います：

1. **要件定義**: 必要なAWSサービスと構成を明確にします
2. **インターフェース定義**: `/lib/interfaces/` に型定義を作成します
3. **構成要素実装**: `/lib/constructs/` に再利用可能な構成要素を実装します
4. **スタック統合**: 既存スタックに統合するか、新しいスタックを作成します
5. **テスト作成**: `/test/` ディレクトリにユニットテストを作成します
6. **ドキュメント更新**: `/cdk/docs/` に詳細ドキュメントを作成・更新します

### セキュリティ考慮事項

- **最小権限の原則**: IAMロールとポリシーは必要最小限の権限のみを付与します
- **暗号化**: データの暗号化を適切に設定します
- **アクセス制御**: S3バケットポリシーやCloudFrontの設定で適切なアクセス制御を行います
- **環境変数**: 機密情報は環境変数で管理し、コードに直接記述しません

詳細な実装ガイドやベストプラクティスについては、`/cdk/docs/` ディレクトリの各ドキュメントを参照してください。

## トラブルシューティング

開発中に発生する可能性のある一般的な問題とその解決策については、`/web/docs/troubleshooting.md` を参照してください。
