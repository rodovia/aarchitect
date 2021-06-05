const sockets = {};

function getSequence(socket) {
    return sockets[socket];
}

function addSequenceToSocket(socket) {
    if (sockets[socket] !== undefined) {
        return;
    }
    sockets[socket] = 0;
}

function increaseSequence(socket) {
    if (socket[socket] === undefined) {
        addSequenceToSocket(socket);
    }

    return sockets[socket]++;
}

function removeSequence(socket) {
    if (sockets[socket] === undefined) {
        return;
    }

    delete sockets[socket];
}

module.exports = {
    getSequence,
    addSequenceToSocket,
    increaseSequence,
    removeSequence
};
