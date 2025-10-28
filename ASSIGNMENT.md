# 今後の課題
このドキュメントは将来的に追加する機能、潜在的な不具合をまとめたドキュメントです。
> 自戒: 本名でcommitしない

## app/
- **config.rs** 将来的に.envとsettings.tomlで設定できるようにする
- **runner.rs** Commandのベクトルを消して、`commands/registry.rs`ですべてのコマンドを管理するようにする
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
- **config/** 何するか忘れた
- **lm/clinet.rs** セッション内でモデルを変える機能を実装。マルチターン対応
- **lm/error.rs** エラーハンドリングの実装
- **lm/streaming.rs** SSEプロトコルでのリアルタイム出力。
- **storage/** セッションをベクトルサーバーに保存。またアクティブセッションの履歴をメモリに保存
## utils/
- **logging.rs** ログ出力のフォーマットを変更
- **regex.rs** 正規表現の実装
## /
- **main.rs** 処理の最適化
