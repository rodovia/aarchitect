// testar facilmente o websocket kk.

const WebSocket = require("ws");
const readline = require("readline");

const socket = new WebSocket("ws://127.0.0.1:9192");

socket.on("open", () => {
    socket.send(JSON.stringify({
        "operation": "identity",
        "data": {
            "username": "Rodoviajw"
        }
    }));

    const reader = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    reader.question(">> ", answer => {
        socket.send(JSON.stringify({
            operation: "message_add",
            data: {
                content: answer
            }
        }));
    });
});

socket.on("message", (data) => {
    const json = JSON.parse(data);
    if (json["event"] == "MESSAGE_CREATE") {
        let message = json["data"]["message"];
        console.log(`${message["author"]}: ${message["content"]}`)
    }
});

socket.on("close", (code, reason) => {
    console.log(`WS disconnected with code ${code} (reason ${reason})`);
});
