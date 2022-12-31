# rockstar docs

rockstar is written in rust, using [mlua](https://lib.rs/mlua), so auto-generated docs isn't possible - I hope these hand-written docs can provide enough information for you to enjoy rockstar!

rockstar APIs reside under `rockstar` in the global table.

## rockstar

- `rockstar.sleep(ms: number)` - sleep for `ms` milliseconds
  - this will block, so be careful!
- `rockstar.on(event: string, callback: function)` - register a callback for an event
  - see the events below
  - the callback will get one argument, which is a table of data

### rockstar.screen

- `rockstar.screen.capture(idx: number | nil): CaptureData` - take a screenshot of a display
  - when not supplied, `idx` defaults to `0`

### rockstar.mouse

- `rockstar.mouse.left_pressed: boolean` - whether the left mouse button is pressed
- `rockstar.mouse.right_pressed: boolean` - whether the right mouse button is pressed
- `rockstar.mouse.middle_pressed: boolean` - whether the middle mouse button is pressed
  - these three are getter/setters, so you can set these to true or false to set the state of the mouse button
- `rockstar.mouse.click(button: MouseButton)` - click a mouse button

### rockstar.keyboard

- `rockstar.keyboard.press(key: Key)` - press a key

### rockstar.sound

- `rockstar.sound.play(path: string)` - play the sound file at `path` on the default output device
  - this runs in another thread, so while it won't blok, you won't get an error if it fails

## custom types

### CaptureData

- `width: number` - the width of the capture
- `height: number` - the height of the capture
- `get(x: number, y: number): Rgb` - get the color at certain coordinates
  - `x` and `y` are 0-indexed

### Rgb

- `r: number` - the red value of this pixel
- `g: number` - the green value of this pixel
- `b: number` - the blue value of this pixel

### MouseButton

one of `1` (left), `2` (right), or `3` (middle)

### Key

a string matching the [rdev::Key](https://docs.rs/rdev/latest/rdev/enum.Key.html) enum

## events

### ready

(the table passed to the callback is empty)

### mouse_press

- `button: MouseButton` - the button that was pressed

### mouse_release

- `button: MouseButton` - the button that was released

### mouse_move

- `x: number` - the x coordinate of the mouse
- `y: number` - the y coordinate of the mouse

### key_press

- `key: Key` - the name of the key

### key_release

- `key: Key` - the name of the key
