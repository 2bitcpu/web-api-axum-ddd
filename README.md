# Rust + Axum + Sqlx(Sqlite) Web Api Example


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