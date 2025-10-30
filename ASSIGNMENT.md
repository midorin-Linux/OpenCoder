# 今後の課題
このドキュメントは将来的に追加する機能、潜在的な不具合をまとめたドキュメントです。
> 自戒: 本名でcommitしない

## app/
- **config.rs** 特になし
- **runner.rs** 特になし
## cli/
- **output.rs** warningやerrorを独自のフォーマットで出力。またモデルからのレスポンスをmd対応にする
- **prompt.rs** デザイン変更
## commands/
- **command.rs** コマンドの関数をtraitで定義する
- **parser.rs** 特になし
- **registry.rs** 特にないができれば構造を変更したい
- **handlers/** コマンド実装
## domain/
固有ドメインに対応
## infrastructure/
- **lm/clinet.rs** 特になし
- **lm/error.rs** エラーハンドリングの実装
- **storage/** セッションをベクトルサーバーに保存。
## utils/
- **logging.rs** 特になし
## /
- **main.rs** 処理の最適化
