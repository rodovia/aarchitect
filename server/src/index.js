const WebSocket = require("ws");
const WebsocketServer = WebSocket.Server;
const tracers = require("./tracers");
const warner = require("./warner");

var usernames = {};
var users = [];

var wss = new WebsocketServer({ port: 9192 });

wss.on("connection", (socket, request) => {
    tracers.addSequenceToSocket(socket);

    socket.on("message", (data) => {
        let json = JSON.parse(data);
        console.log(`Received from socket: ${data}`);

        if (json["operation"] === "identity") {
            let username = null;
            
            Object.values(usernames).forEach((item) => {
                if (item === json["data"]["username"]) {
                    username = json["data"]["username"]
                } 
            });
            usernames[socket] = json["data"]["username"]

            if (username !== null) {
                socket.close(4001, "4001 - NOT AVAILABLE USER");
                return;
            }
            
            warner.emitManyEvent(wss.clients, "USER_ADD", {
                user: {
                    username: json["data"]["username"]
                }
            });
            
            warner.emitEvent(socket, "CHAT_STATE_CREATE", {});
            users.push(socket);
        }

        if (json["operation"] === "message_add") {
            let date = Date.now();
            let data = {
                message: {
                    content: json["data"]["content"],
                    created_at: parseInt(date.toString()),
                    author: usernames[socket]
                }
            }

            warner.emitManyEvent(wss.clients, "MESSAGE_CREATE", data);
        }


        if (json["sequence"] !== undefined) {
            socket.close(4356, "4356 - CHANGED ROLES");
        }

    });

    socket.on("close", (code, reason) => {
        tracers.removeSequence(socket);
        warner.emitManyEvent(wss.clients, "USER_REMOVE", {
            user: {
                username: usernames[socket]
            }
        });

        delete usernames[socket];
        let u = users.findIndex((val) => val == socket);
        delete users[u];
    });
});
