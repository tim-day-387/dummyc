# Dummy Interpreter
[![linux_build](https://github.com/tim-day-387/dummy_compiler/actions/workflows/linux_build.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/linux_build.yml)

# About

The Dummy interpreter is a BASIC interpreter written entirely in Rust which is intended to adhere to the 
[ECMA-55 minimal BASIC standard](https://ia903007.us.archive.org/15/items/ecma-55-1978/ecma-55-1978.pdf). The goal is to completely 
implement the standard using simple, elegant code, and hopefully learn a lot about programming languages along the way!

# Installation and Usage

A Github action has been setup to automatically create a Debian package from the most recent version of the repository. Download the artifact 
(a .deb file) from the workflow and run the following command as root:\
\
```dpkg -i dummyc.deb```\
\
The compiler manual pages can be seen via the command:\
\
```man dummyc```\
\
Alternatively, the repository can be cloned and built using cargo.

# Notes

The project originated as a BASIC to Rust transpiler. The last version of the transpiler can be found in the transpiler branch. Most of the transpiler
code was reused in the interpreter before being ultimately refactored. A feature is planned that would allow the embedding of a BASIC program in a
reduced version of the interpreter. Although the methodology is different, the end result is similar: a Rust program which is equivalent in 
function to a BASIC function.  

# Contribution

If you use the interpreter and notice bugs or issues, feel free to submit an issue in the issues tab. 

# Disclaimer

This project is a work in progress and subject to change! 
