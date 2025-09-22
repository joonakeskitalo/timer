# Timer

<img width="1010" height="456" alt="timer" src="https://github.com/user-attachments/assets/745af616-4ff3-4ef5-8ddb-e7557fbada74" />

Simple timer & alarm app in one. You can set the desired duration of a timer or the time when the alarm will end. When the timer ends, it'll play a beeping sound until dismissed.

## Hotkeys

- `n` create new window
- `Escape`: stop timer
- `q` or `§`: toggle window size
- `Enter` when focused on input fields will start the timer

## Installation

- Download the app from the releases section in Github, open the `timer.dmg` and drag Timer.app to macOS applications folder
- If you want to run the code in the browser, open `./src/index.html`

## Development

- Install [Tauri v2](https://v2.tauri.app/)
- Run `bun install`
- Run `bun run dev`
- To build release version, run `bun run build`