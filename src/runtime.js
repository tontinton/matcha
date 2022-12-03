((globalThis) => {
    const { core } = Deno;
    const { ops } = core;
    // Note: Do not call this when snapshotting, it should be called
    // at runtime. This example does not use V8 snapshots.
    core.initializeAsyncOps();

    const RESET_COLOR = '\u001b[0m';
    const RED_COLOR = '\u001b[31m';
    const YELLOW_COLOR = '\u001b[33m';

    function printColored(color, stderr, ...args)  {
        core.print(`${color}${args.join(" ")}\n${RESET_COLOR}`, stderr);
    }

    globalThis.console = {
        log: (...args) => {
            printColored(RESET_COLOR, false, ...args);
        },
        warn: (...args) => {
            printColored(YELLOW_COLOR, false, ...args);
        },
        error: (...args) => {
            printColored(RED_COLOR, true, ...args);
        },
    };

    globalThis.Matcha = {
        readFile: (path) => {
            return ops.op_read_file(path);
        },
        writeFile: (path, contents) => {
            return ops.op_write_file(path, contents);
        },
        removeFile: (path) => {
            return ops.op_remove_file(path);
        },
    };
})(globalThis);
