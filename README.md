# OpenCoder
「OpenAI API互換のモデルが使えるコーディングエージェントあったらおもろそうじゃね」という軽い気持ちで書いています。
コードの質がどうかは知りませんが参考にしていただけると嬉しいです。

## 使用時には
.envを自分で追加してください。形式は以下の通りです。
````aiexclude
API_KEY=
RUST_LOG=
OPENAI_API_URL=
TIMEOUT_SECS=
````
空白の場合デフォルトの設定が適用されます
- API_KEY...適切なAPIキーを入力してください。デフォルトは「 suwako 」です。
- RUST_LOG...ログのレベルです。debug, info, warn, errorのいずれかが使えるはずです。デフォルトは「 info 」です。
- OPENAI_API_URL...OpenAI API互換のエンドポイントのURLです。デフォルトは「 http://127.0.0.1:1234/v1 」です。(例: http://127.0.0.1:1234/v1/models -> http://127.0.0.1:1234/v1 と入力)
- TIMEOUT_SECS...http/https通信でのタイムアウト秒数を設定できます。デフォルトは「 60 」です。