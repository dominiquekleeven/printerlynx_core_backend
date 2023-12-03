# Printerlynx Core Backend WebSockets
WebSockets messaging specification for the Printerlynx Core Backend
## General
All messages are sent as JSON objects with the following structure:
```js
{
    "type": "message_type",
    "data": {
        "key": "value"
    }
}
```
The type field is a string that identifies the type of message being sent. The data field is an object that contains the message data. The data field may be omitted if the message does not contain any data.

**The following message types are available**
* authentication
* print_job
* printer_command
* agent_status
* printer_status

Each message type data is processed by a corresponding handler function. The handler function is responsible for validating the data and performing the appropriate action. The handler function **may** also send a response message back to the client.


---

## /api/v1/ws
#### User Messaging (Front-end <--> Back-end)

<details>
<summary><b style="color: cornflowerblue">Authentication</b></summary>

The **authentication** message is used to authenticate the front-end client and establish a websocket connection.

**Client --> Server**
```js
{
    "type": "authentication",
    "data": {
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MzI1Njg4MTEsImlzcyI6IlByaW50ZXJseW54Iiwic3ViIjoiMmVmNDQyZmUtYjg5Yy00NDZkLTlkNDMtMDI0NmRhN2UxODM2In0.U11rn1AjfzbAAvWrzMOJHOyiQkcogYF2FJZ0RijqgU0"
    }
}
```
If the token is valid, the server will respond with a success message.
```js
{
    "type": "authentication",
    "data": {
        "status": "OK"
    }
}
```

If the token is invalid, the server will respond with an error message.
```js
{
    "type": "authentication",
    "data": {
        "status": "ERROR",
        "message": "error message"
    }
}
```
</details>

<details>
<summary><b style="color: cornflowerblue">Print Jobs</b></summary>

The **print_job** message is used to send print jobs to the backend. The print job is then forwarded to the appropriate agent and printer.

**Client --> Server**
```js
{
    "type": "print_job",
    "data": {
        "type": "START", // START, PAUSE, RESUME, CANCEL
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "print_file_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "printer_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722"
    }
}
```
If the message is valid, the server will respond with a success message.
```js
{
    "type": "print_job",
    "data": {
        "status": "OK"
    }
}
```
    
If the message is invalid, the server will respond with an error message.
```js
{
    "type": "print_job",
    "data": {
        "status": "ERROR",
        "message": "error message"
    }
}
```
</details>

<details>
<summary><b style="color: cornflowerblue">Printer Commands</b></summary>

The **printer_command** message is used to send printer G-Code commands to the backend. The command is then forwarded to the appropriate agent and printer.
This allows the front-end client to send commands to the printer.

**Client --> Server**
```js
{
    "type": "printer_command",
    "data": {
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "printer_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722"
        "command": "G28"
    }
}
```
If the message is valid, the server will respond with a success message.
```js
{
    "type": "printer_command",
    "data": {
        "status": "OK"
    }
}
```
    
If the message is invalid, the server will respond with an error message.
```js
{
    "type": "printer_command",
    "data": {
        "status": "ERROR",
        "message": "error message"
    }
}
```
</details>

The following message types are the ones that the server will send to the client.
<details>
<summary><b style="color: cornflowerblue">Agent Status</b></summary>

The **agent_status** message is used to let the front-end client know the status of the specified agent.


**Agent has established a connection**
```js
{
    "type": "agent_status",
    "data": {
        "agent_name": "Demo",
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "status": "online" // online, offline
    }
}
```
**Agent has lost connection**
```js
{
    "type": "agent_status",
    "data": {
        "agent_identifier": "Demo",
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "status": "offline" // online, offline
    }
}
```


</details>

<details>
<summary><b style="color: cornflowerblue">Printer Status</b></summary>

The **agent_status** message is used to let the front-end client know the status of the specified agent.


**Printer available**
```js
{
    "type": "printer_status",
    "data": {
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722", 
        "printer_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "printer_identifier": "Demo",
        "printer_adapter_identifier": "SERIAL",
        "printer_adapter_interface": "/dev/ttyUSB0",
        "status": "available", // available, unavailable, busy
        "job": null // null or object
        "state": {
            "temperature": {
                "bed": 0,
                "tool0": 0,
                "tool1": 0,
                "tool2": 0,
                "tool3": 0
            },
        }
    }
}
```
**Printer unavailable**
```js
{
    "type": "printer_status",
    "data": {
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722", 
        "printer_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "printer_identifier": "Demo",
        "status": "unavailable"
    }
}
```
**Printer busy (print job example)**
```js
{
    "type": "printer_status",
    "data": {
        "agent_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722", 
        "printer_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
        "printer_identifier": "Demo",
        "printer_adapter_identifier": "SERIAL",
        "printer_adapter_interface": "/dev/ttyUSB0",
        "status": "busy",
        "job": {
            "print_file_uuid": "7d4ce00f-d60a-4504-96ab-31a83f848722",
            "name": "File.gcode",
            "size": 4106612, // bytes
            "progress": 0.5, // 0.0 - 1.0
        },
        "state": {
            "temperature": {
                "bed": 0,
                "tool0": 0,
                "tool1": 0,
                "tool2": 0,
                "tool3": 0
            },
        }
    }
}
```


</details>


---

## /api/v1/ws/agent
#### Agent Registration
**Note:** This endpoint is only used for agent registration and internal messaging with the connected agent. It is not used for user messaging.
<details>
<summary><b style="color: cornflowerblue">Authentication</b></summary>

The authentication endpoint is used to authenticate the agent and establish a websocket connection with the backend.

**Note**: The agent must be a valid agent token, not a user token. These are generated through the web interface / RESTful API

**Agent --> Server**
```js
{
    "type": "authentication",
    "data": {
        "token": "J0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9"
    }
}
```
If the token is valid, the server will respond with a success message.
```js
{
    "type": "authentication",
    "data": {
        "status": "success"
    }
}
```

If the token is invalid, the server will respond with an error message.
```js
{
    "type": "authentication",
    "data": {
        "status": "error",
        "message": "Invalid token"
    }
}
```
</details>

