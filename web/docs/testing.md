# テスト実装ガイドライン

このドキュメントでは、WebToolsプロジェクトにおけるテスト実装のガイドラインを説明します。

## 目次

1. [テストの種類](#テストの種類)
2. [テストの実装方法](#テストの実装方法)
3. [テストの実行方法](#テストの実行方法)
4. [テストカバレッジ](#テストカバレッジ)
5. [テストのベストプラクティス](#テストのベストプラクティス)

## テストの種類

WebToolsプロジェクトでは、以下の種類のテストを実装することを推奨します：

### ユニットテスト

個々の関数やメソッドの動作を検証するテスト。特に、計算ロジックやデータ変換ロジックなど、ビジネスロジックに関わる部分には必ずユニットテストを実装してください。

### コンポーネントテスト

Dioxusコンポーネントの動作を検証するテスト。コンポーネントのレンダリング結果や状態変化を検証します。

### 統合テスト

複数のコンポーネントや機能が連携して動作することを検証するテスト。

## テストの実装方法

### ユニットテストの実装

Rustの標準的なテスト機能を使用してユニットテストを実装します。テストは対応するモジュールと同じファイル内に実装するか、`tests`ディレクトリに別ファイルとして実装します。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name_condition_expected_result() {
        // テストの準備
        let input = "some input";
        
        // テスト対象の関数を呼び出し
        let result = function_to_test(input);
        
        // 結果を検証
        assert_eq!(result, "expected output");
    }
    
    #[test]
    fn test_function_name_error_condition() {
        // エラーケースのテスト
        let input = "invalid input";
        
        // テスト対象の関数を呼び出し
        let result = function_to_test(input);
        
        // エラーが発生することを検証
        assert!(result.is_err());
    }
}
```

### コンポーネントテストの実装

Dioxusコンポーネントのテストには、`dioxus-test`クレートを使用します。

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_test::*;

    #[test]
    fn test_component_renders_correctly() {
        // テスト用のコンポーネントをレンダリング
        let mut app = launch(|cx| cx.render(rsx! { ComponentToTest {} }));
        
        // レンダリング結果を検証
        assert!(app.contains("Expected text in the component"));
    }
    
    #[test]
    fn test_component_handles_user_interaction() {
        // テスト用のコンポーネントをレンダリング
        let mut app = launch(|cx| cx.render(rsx! { ComponentToTest {} }));
        
        // ユーザー操作をシミュレート
        app.find("button").click();
        
        // 状態変化を検証
        assert!(app.contains("Text that should appear after clicking"));
    }
}
```

## テストの実行方法

テストは以下のコマンドで実行できます：

```bash
# すべてのテストを実行
docker compose exec web cargo test

# 特定のテストを実行
docker compose exec web cargo test test_function_name

# 特定のモジュールのテストを実行
docker compose exec web cargo test module_name
```

## テストカバレッジ

テストカバレッジを測定するには、`cargo-tarpaulin`を使用します：

```bash
docker compose exec web cargo tarpaulin
```

新機能の追加や既存機能の変更を行う際は、テストカバレッジが低下しないように注意してください。

## テストのベストプラクティス

1. **テスト名は明確に**: テスト関数の名前は`test_機能名_条件_期待される結果`の形式にします。
2. **境界値をテスト**: 最小値、最大値、エラーケースなど、境界条件をテストします。
3. **テストは独立して実行可能に**: 各テストは他のテストに依存せず、どのような順序で実行しても成功するようにします。
4. **テストデータは明示的に**: テストデータはハードコードし、外部リソースに依存しないようにします。
5. **モックを適切に使用**: 外部依存がある場合は、モックを使用してテストを分離します。
