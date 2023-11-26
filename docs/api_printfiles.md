### Printfiles API
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
