/**
 * 指定された環境変数を取得する
 *
 * @param {string} name 環境変数名
 * @throws {Error} 環境変数が取得できなかった場合
 */
export function requireEnv(name: string): string {
  const value = process.env[name];

  if (value) {
    return value as string;
  }

  throw new Error(`環境変数 ${name} が設定されていません。デプロイを続行できません。`);
}
