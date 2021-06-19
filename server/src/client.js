// testar facilmente o websocket kk.

const WebSocket = require("ws");
const readline = require("readline");
const socket = null;

const rl = readline.createInterface({
    input: process.stdin, 
    output: process.stdout
  });

let username = rl.question("Nome de usuário: ");
let ipAddress = rl.question("IP do servidor: ");

const actualIpAddr = `ws://${ipAddress}:9192`;
const webs = new WebSocket(actualIpAddr);

webs.on("open", () => {
    webs.send(JSON.stringify({
        operation: "identity",
        data: {
            "username": username
        }
    }));
    
    (async function () {
        for await (const line of rl) {
            console.log(line)
        }
    })();
    
});

webs.on("message", (data) => {
    let dat = JSON.parse(data);
    if (dat["operation"] == "dispatch") {
        if (dat["event"] == "MESSAGE_CREATE") {
            let content = dat["data"]["message"]["content"];
            console.log(`${dat['event']['data']['message']['author']}: ${content}`);
        }
    }
});
