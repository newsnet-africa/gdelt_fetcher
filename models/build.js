const pbjs = require("protobufjs-cli/pbjs");
const pbts = require("protobufjs-cli/pbts")
const fs = require("fs");
const path = require("path")
// Increase the maximum number of listeners for the EventEmitter
const EventEmitter = require('events');
EventEmitter.defaultMaxListeners = 20; // Increase the limit as needed

// Ensure lodash is installed
// Run the following command in your terminal
// npm install lodash

const basePath = path.join(__dirname, "../proto");

const list_of_files = fs.readdirSync(basePath);
console.log(list_of_files)

for (i = 0; i < list_of_files.length; i++) {
    let file_base = list_of_files[i].split(".")[0]
    const argsjs = [
        "-t", "static-module",
        "-w", "default",
        "--root", `${file_base}`,
        "-o", `./generated/${file_base}.js`,
        // "-es6",
        `../proto/${list_of_files[i]}`,
    ];

    const argsts = [
        "-t", "static-module",
        "-w", "default",
        "-o", `./generated/${file_base}.d.ts`,
        `./generated/${file_base}.js`,
    ];

    console.log("Trying file:", file_base);

    pbjs.main(argsjs, function (err, output) {
        if (err) {
            console.error("Error running pbjs:", err);
            throw err;
        }

        console.log("Successfully (I hope) created JAVASCTRIPT types");
    });

    pbts.main(argsts, function (err, output) {
        if (err) {
            console.error("Error running pbts:", err);
            throw err;
        }

        console.log("Successfully (I hope) created TYPESCRIPT types");
    });
}