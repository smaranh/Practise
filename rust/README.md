# Rust cross-compilation for Windows from Mac

# Pre-requisites
Make sure you have the following installed on Mac

MacPorts for Mojave - https://www.macports.org/install.php

`sudo port install x86_64-w64-mingw32-gcc`

`rustup target add x86_64-pc-windows-gnu`

# Ready to create .exe
`cargo build --release  --target x86_64-pc-windows-gnu`
