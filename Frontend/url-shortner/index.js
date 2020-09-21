const shortId = require('short-uid');
const express = require('express');
const server = express();

const genId = new shortId();

server.use(express.static(__dirname));
server.use(express.json())
server.use(express.urlencoded({ extended: true }))

server.get('/', (req, res) => {
    res.render('index.html');
});

server.get('/short', (req, res) => {
    const newId = genId.randomUUID();
    res.send({newId: newId});
});

server.listen('2300', () => {
    console.log("server is running");
});