var Player = require('mpris-service');
const { exec, execSync } = require('child_process');


var player = Player({
	name: 'rapd',
	identity: 'Rust Audio Player Daemon MPRIS Client',
	supportedUriSchemes: ['file'],
	supportedMimeTypes: ['audio/mpeg', 'application/ogg', 'application/mp3'],
	supportedInterfaces: ['player']
});

player.on("pause", () => {
  execSync("rapc player_pause");
});

player.on("play", () => {
  execSync("rapc player_unpause");
});

player.on("stop", () => {
  execSync("rapc player_stop");
});
