# Rust + Axum + Sqlx(Sqlite) Web Api Example

## 実装
- [x] 一通りのCRUD実装
- [ ] 認証/認可
  - [x] signup
  - [X] sinin
  - [x] signout
  - [x] 更新系のエンドポイントは認可必要
  - [ ] 参照系のエンドポイントは認可不要
  - [ ] アカウントロック
    - [x] パスワードを3回間違うとアカウントロック
    - [ ] 間違い回数 x 8時間後にはログイン試行可能(3回で24時間)
    - [ ] ログインに成功するまで間違い回数はクリアしない(4回目を間違うと32時間ロック)
- [ ] ロギング
  - [x] 簡易版(標準出力のみ)
  - [ ] ファイル出力(ローテーション)
- [ ] 一覧取得(ページングあり)
- [ ] 一覧取得(フィルタ(前方一致)、ページングあり)
- [ ] エラーハンドリング
- [ ] バリデーション
- [ ] Graceful shutdown

### 動作確認用curlコマンド
```
# ユーザー登録API
curl -i -X POST http://localhost:3000/service/auth/signup \
-H "Content-Type: application/json" \
-d '{
    "account": "tester",
    "password": "p@55w0rd",
    "confirmPassword": "p@55w0rd",
    "email": "tester@local",
    "name": "no name"
}'

# ユーザーログインAPI
curl -i -X POST http://localhost:3000/service/auth/signin \
-H "Content-Type: application/json" \
-d '{
    "account": "tester",
    "password": "p@55w0rd"
}'

# コンテンツ投稿API
curl -i -X POST http://localhost:3000/service/contents/post -H "Content-Type: application/json" \
-H "Authorization: Bearer token" \
-d '{
    "contentId": 0,
    "account": "tester",
    "postAt": "2025-03-30T17:55:01Z",
    "title": "test",
    "body": "test"
}'

# コンテンツ取得API
curl -i -X GET http://localhost:3000/service/contents/get/1 \
-H "Authorization: Bearer -H "Authorization: Bearer token" \
"

# コンテンツ更新API
curl -i -X POST http://localhost:3000/service/contents/edit -H "Content-Type: application/json" \
-H "Authorization: Bearer token" \
-d '{
    "contentId": 1,
    "account": "tester",
    "postAt": "2025-03-30T17:55:59Z",
    "title": "test-xx",
    "body": "test-xx"
}'

# コンテンツ削除API
curl -i -X GET http://localhost:3000/service/contents/remove/1 \
-H "Authorization: Bearer token"


# ログアウトAPI
curl -i -X GET http://localhost:3000/service/auth/signout \
-H "Authorization: Bearer token"

```