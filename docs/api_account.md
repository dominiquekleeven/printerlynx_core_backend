### Accounts API
Endpoints require the Authorization header
```js
"Authorization":"Bearer <Token>"
```
##### GET /api/v1/accounts/me
Retrieve the token account information
```js
Response
{
    "uuid": "2ef442fe-b89c-446d-9d43-0246da7e1836",
    "username": "apidemo",
    "email": "demo@demo.com"
}
```
---
##### PUT /api/v1/accounts/me
Edit the account e-mail/details.

```js
Request
{     
    "email": "demo@demo.com"
}
```

```js
Response
{ "Status": 200 }
```
---
##### PUT /api/v1/accounts/me/password
Edit the account password

```js
Request
{     
    "old_password": "password",
    "password": "password123",
    "password_confirmation": "password123"
}
```

```js
Response
{ "Status": 200 }
```
---
##### DELETE /api/v1/accounts/me
Deletes the token associated account

```js
Response
{ "Status": 200 }
```