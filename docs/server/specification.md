# Specification of server handling.

## Receiving and sending payloads

All payloads sent by client will not have a `sequence` field. If it has, the socket will disconnect with code *4356 - CHANGED ROLES*.

## Connecting

When the users connects and properly finishes handshake, the user shall send the IDENTIFY payload as JSON string encoded in UTF-8.

# Payloads

### IDENTIFY

A simple template of a identify payload.

```json
{
    "operation": "identity",
    "data": {
        "username": "Rodoviajw"
    }
}
```

If the identity is valid, the socket will return a `CHAT_STATE_CREATE` payload.
Some possibilities of the server closing in this moment can be, but not limited to:

    * Using a already taken nickname (`4001 - NOT AVAILABLE USER`)

### USER_ADD

Sent when a user joins the chat.

```json
{
    "sequence": 1337,
    "operation": "dispatch",
    "event": "USER_ADD",
    "data": {
        "user": {
            "username": "oJoão"
        }
    }
}
```

### MESSAGE_CREATE

Dispatched when a user sents a message

```json
{
    "message": {
        "content": "Howdy!",
        "author": "Rodoviajw",
        "created_at": "big-number-here"
    }
}
```

`created_at` represents when the message was created, in UNIX timestamp format.

### USER_REMOVE

Sent when a user quits from the chat.

```json
{
    "user": {
        "username": "oJoão"
    }
}
```

### How to send messages

The client can send messages using the `message_add` operation. It's short and simple:

```json
{
    "operation": "message_add",
    "data": {
        "content": "Howdy!"
    }
}
```

If the payload is correct and it was sucessfully sent, the server will dispatch to all clients (including you) **MESSAGE_CREATE** event.


