### Authentication API
##### POST /api/v1/auth/register
Register an account.
```js
Request
{
    "username": "apidemo",
    "email": "demo@demo.com",
    "password": "demo123",
    "password_confirmation": "demo123"
}
```

```js
Response
{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3Masdasg4MTEsImlzcyI6IlByaW50ZXJseW54Iiwic3ViIjoiMmVmNDQyZmUtYjg5Yy00NDZasdDI0NmRhN2UxODM2In0.U11rn1AjfzbAAvWrzMOJHOyiQkcogYF2FJZ0RijqgU0"
}
```
---

##### POST /api/v1/auth/login
Authenticate an account.
```js
Request
{
    "username": "apidemo",
    "password": "demo123",
}
```
```js
Response
{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MzI1Njg4MTEsImlzcasdByaW50ZXJseW54Iiwic3ViIjoiMmVmNDQyasdasdA0NDZkLTlkNDMtMDI0NmRhN2UxODM2In0.U11rn1AjfzbAAvWrzMOJHOyiQkcogYF2FJZ0RijqgU0"
}
```
---

##### POST /api/v1/auth/logout
Logout an account and invalidate the associated token.

```js
Response
{ "Status": 200 }
```
