# rockstar

rockstar is a lua scripting tool. i like to think of it as an AHK replacement, but with a better language (insert your lua bad joke here)

## using

run `rockstar path/to/script.lua` to run a script.

after your script evaluates, rockstar will start emitting events to the callbacks you register in your script. from here, you can access state in the `rockstar` global.

your script will not receive events or state updates until after the evaluation completes (e.g. `rockstar.mouse.left_pressed` will be `false` until after your script evaluates).

to access a REPL, pass `-r` or `--repl`.

for more information, see [the docs](DOCS.md).
