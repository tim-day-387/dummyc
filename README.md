# Dummy Compiler
[![Rust](https://github.com/tim-day-387/dummy_compiler/actions/workflows/rust.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/rust.yml) [![Debian Package](https://github.com/tim-day-387/dummy_compiler/actions/workflows/debian.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/debian.yml) [![Example Programs](https://github.com/tim-day-387/dummy_compiler/actions/workflows/test_programs.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/test_programs.yml)


# About

The Dummy compiler is a BASIC compiler written entirely in Rust which is intended to adhere to the 
[ECMA-55 minimal BASIC standard](https://ia903007.us.archive.org/15/items/ecma-55-1978/ecma-55-1978.pdf). The goal is to completely implement the standard using
simple, elegant code, and hopefully learn a lot about compilers along the way!

# Installation and Usage

A Github action has been setup to automatically create a Debian package from the most recent version of the repository. Download the artifact (a .deb file) from the
workflow and run the following command as root:\
\
```dpkg -i dummyc.deb```\
\
The compiler manual pages can be seen via the command:\
\
```man dummyc```

# Contribution

If you use the compiler and notice bugs or issues, feel free to submit an issue in the issues tab. 

# Disclaimer

This project is a work in progress and subject to change! 
