# rockstar

![bocchi from hit anime bocchi the rock](https://cdn.discordapp.com/attachments/546437828767121424/1057030115269283860/image.png)

rockstar is a "lua scripting tool that gives you social anxiety". i like to think of it as an AHK replacement, but with a better language (insert your lua bad joke here)

**rockstar is alpha software** - so far, I'm building this on one machine over my new years vacation. things may break (but please tell me if they do)!

## using

run `rockstar path/to/script.lua` to run a script.

after your script evaluates, rockstar will start emitting events to the callbacks you register in your script. from here, you can access state in the `rockstar` global.

your script will not receive events or state updates until after the evaluation completes (e.g. `rockstar.mouse.left_pressed` will be `false` until after your script evaluates).

for more information, see [the docs](DOCS.md).
