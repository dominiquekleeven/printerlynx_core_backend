
### Agents API
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
    "description": "This is a demo agent"
}
```

```js
Response
{
    "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
    "name": "Demo",
    "description": "This is a demo agent",
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
        "description": "This is a demo agent",
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
    "description": "This is a demo agent",
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
    "description": "This is a demo1 agent"
}
```
```js
Response
{
    "uuid": "54588f93-80df-4daf-ab8c-ad92a1333139",
    "name": "Demo1",
    "description": "This is a demo1 agent",
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
