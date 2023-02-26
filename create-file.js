const fs = require("fs");

async function main() {
    const stream = fs.createWriteStream("./input.txt");

    for (let i = 0; i < 5000000; i++) {
        await new Promise((resolve, reject) => stream.write("foo bar baz foo bar baz foo bar baz foo bar baz [ foo bar baz foo bar baz foo bar baz foo bar baz ] foo bar baz [ foo ] bar baz foo bar baz foo bar baz foo bar baz [] foo\n", (err) => {
            if (err) reject(err);
            else resolve();
        }));
    }
}

main();
