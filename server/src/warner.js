const tracers = require("./tracers");

var useszlib = {}

function emit(sockets, data, operation) {
    for (let socket of sockets) {
        let seq = tracers.getSequence(socket);
        socket.send(JSON.stringify({
            sequence: seq,
            operation,
            data
        }));
    }
}

function emitManyEvent(sockets, event, data) {
    for (let socket of sockets) {
        emitEvent(socket, event, data);
    }
}

function emitEvent(socket, event, data) {
    let template = getMessageTemplate(socket, "dispatch", data);
    template.event = event;
    tracers.increaseSequence(socket);
    
    socket.send(JSON.stringify(template));
}

function getMessageTemplate(socket, operation, data) {
    return {
        sequence: tracers.getSequence(socket),
        operation,
        data
    };
}

module.exports = {
    emit,
    getMessageTemplate,
    emitManyEvent,
    emitEvent
};
