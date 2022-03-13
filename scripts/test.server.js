// First, import some helper libraries. `shelljs` is included in the
// devDependencies of the root project, which is why it's available here. It
// makes it easy to use *NIX-style scripting (which works on Linux distros,
// macOS, and Unix systems) on Windows as well.
const sh = require("shelljs");

sh.exec(`curl "http://localhost:8080/"`);
sh.exec(`curl "http://localhost:8080/craft-new-token?nearid='meta_irony.testnet'&tokentype='love'"`);