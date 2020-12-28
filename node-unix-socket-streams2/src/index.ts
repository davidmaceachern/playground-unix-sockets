var UnixStream = require('unix-socket-streams2');
let socketPath: string = '/home/davidmaceachern/github/nodesozu/tmp/sock'
var socket = new UnixStream(socketPath, { type: 'tcp' });
socket.connect(socketPath, function (err, stream) { });
socket.write(callback);
