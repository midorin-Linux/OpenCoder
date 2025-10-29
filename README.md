# OpenCoder
「OpenAI API互換のモデルが使えるコーディングエージェントあったらおもろそうじゃね」という軽い気持ちで書いています。
コードの質がどうかは知りませんが参考にしていただけると嬉しいです。

## 実行前に
.envを以下のように設定してください
```
API_URL=
API_KEY=
MODEL=
RUST_LOG=
TIMEOUT_SECS=
```
- **API_URL** APIエンドポイント([http://127.0.0.1:1234/v1/models](http://127.0.0.1:1234/v1/models)でモデルを取得で見る場合は[http://127.0.0.1:1234/v1](http://127.0.0.1:1234/v1)と入力)
- **API_KEY** APIキーを入力してください
- **MODEL** 使うモデル名を入力してください
- **RUST_LOG** ロギングレベル(debug, info, warn, error)を入力してください
- **TIMEOUT_SECS** タイムアウト時間を入力してください