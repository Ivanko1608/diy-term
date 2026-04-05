# DIY Terminal

This is just a simple shell implementation created for the purpose of experimenting with rust it a systems programming setting.

The project uses no crates and I'm making it with the standard lib only.

## Currently it can do

- cd and pwd (including ~)
- type and echo
- run PATH binaries
- exit gracefully.

## Next to implement

- ~~history~~
- using arrows to move around in the prompt
- io redirection.
- *CTRL + C* handler (this requires some unsafe code that is platform specific so I left it for later.)
- Maybe more!

## Useful reading to do this yourself
- Xterm ANSI control codes reference https://invisible-island.net/xterm/ctlseqs/ctlseqs.html
