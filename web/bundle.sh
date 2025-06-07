#!/bin/bash

# Strict mode: エラー時即座に終了、未定義変数でエラー、パイプラインでエラー
set -euo pipefail

# ファイル名の環境変数定義
CARGO_TOML="Cargo.toml"
CARGO_TOML_BACKUP="Cargo.toml.backup"
CARGO_PRD_TOML="Cargo-prd.toml"

# クリーンアップ関数
cleanup() {
    echo "クリーンアップを実行中..."
    
    # バックアップが存在する場合、元のファイルを復元
    if [[ -f "${CARGO_TOML_BACKUP}" ]]; then
        echo "元のCargo.tomlを復元中..."
        mv "${CARGO_TOML_BACKUP}" "${CARGO_TOML}"
    fi
    
    echo "クリーンアップ完了"
}

# スクリプト終了時（正常・異常問わず）にクリーンアップを実行
trap cleanup EXIT

# メイン処理
echo "開発環境用設定に切り替え中..."

# 元のCargo.tomlをバックアップ
cp "${CARGO_TOML}" "${CARGO_TOML_BACKUP}"

# 開発用設定に置き換え
cp "${CARGO_PRD_TOML}" "${CARGO_TOML}"

echo "アプリケーションを bundle 中"

# アプリケーション bundle（エラーが発生してもcleanupが実行される）
dx bundle \
  --features production \
  --platform web \
  --ssg \
  --release

echo "アプリケーション bundle 成功"
