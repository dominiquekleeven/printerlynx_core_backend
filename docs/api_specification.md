# Printerlynx Core Backend API
RESTful API Specification for Printerlynx Core Backend
## Authentication API
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
---
## Accounts API
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
---
## Printfiles API
Endpoints require the Authorization header
```js
"Authorization":"Bearer <Token>"
```
##### POST /api/v1/printfiles
Upload a multi-part file, requires appended formdata, mimetype: gcode.
```js
Request
{ 
    "Content-Type": 'multipart/form-data'
}
```

```js
Response
{ "Status": 200 }
```
---
##### GET /api/v1/printfiles
Retrieves token associated print files.

```js
Response
[
    {
        "uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "name": "File.gcode",
        "size": 4106612,
        "checksum": "45696c9a8b06eed6287f1c1a233a8f74841999d56e08145a6e50903828751067",
        "file_type": "Gcode",
        "file_storage_type": "s3",
        "created_at": "1701016434"
    }
]
```
---
##### GET /api/v1/printfiles/:uuid
Retrieve print file details based on uuid.

```js
Response
{
    "uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
    "name": "File.gcode",
    "size": 4106612,
    "checksum": "45696c9a8b06eed6287f1c1a233a8f74841999d56e08145a6e50903828751067",
    "file_type": "Gcode",
    "file_storage_type": "s3",
    "created_at": "1701016434"
}
```
---

##### GET /api/v1/printfiles/:uuid/download
Retrieve print file data

```js
Response
{ 
    "Content-Type": 'application/octet-stream'
}
```
---
##### DELETE /api/v1/printfiles/:uuid
Delete print file based on uuid.

```js
Response
{ "Status": 200 }
```
---
## Agents API
Endpoints require the Authorization header
```js
"Authorization":"Bearer <Token>"
```
##### POST /api/v1/agents
Add a new print agent.
```js
Request
{
    "name": "Demo",
    "description": "This is a demo agents"
}
```

```js
Response
{
    "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
    "name": "Demo",
    "description": "This is a demo agents",
    "token": "854d8e05-b423-48a5-a8ec-5d4c24489439",
    "created_at": "1701035283"
}
```
---
##### GET /api/v1/agents
Retrieves the token associated agents
```js
Response
[
    {
        "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
        "name": "Demo",
        "description": "This is a demo agents",
        "token": "854d8e05-b423-48a5-a8ec-5d4c24489439",
        "created_at": "1701035283"
    }
]
```
---
##### GET /api/v1/agents/:uuid
Retrieves a specific agent
```js
Response
{
    "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
    "name": "Demo",
    "description": "This is a demo agents",
    "token": "854d8e05-b423-48a5-a8ec-5d4c24489439",
    "created_at": "1701035283"
}
```
---
##### PUT /api/v1/agents/:uuid
Changes an existing agent
```js
Request
{
    "name": "Demo1",
    "description": "This is a demo1 agents"
}
```
```js
Response
{
    "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
    "name": "Demo1",
    "description": "This is a demo1 agents",
    "token": "854d8e05-b423-48a5-a8ec-5d4c24489439",
    "created_at": "1701035283"
}
```
---
##### DELETE /api/v1/agents/:uuid
Deletes an existing agent
```js
Response
{
    "status": 200
}
```

