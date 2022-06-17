# Dummy Interpreter
[![linux_build](https://github.com/tim-day-387/dummy_compiler/actions/workflows/linux_build.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/linux_build.yml)
[![windows_build](https://github.com/tim-day-387/dummy_compiler/actions/workflows/windows_build.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/windows_build.yml)
[![macos_build](https://github.com/tim-day-387/dummy_compiler/actions/workflows/macos_build.yml/badge.svg)](https://github.com/tim-day-387/dummy_compiler/actions/workflows/macos_build.yml)

# About

The Dummy interpreter is a BASIC interpreter written entirely in Rust which is intended to adhere to the 
[ECMA-55 minimal BASIC standard](https://ia903007.us.archive.org/15/items/ecma-55-1978/ecma-55-1978.pdf). The goal is to completely 
implement the standard using simple, elegant code, and hopefully learn a lot about programming languages along the way!

The interpreter diverges from the standard in a few ways. Any divergence from the standard is an addition, not a subtraction. The ECMA standard
is the baseline. One notable improvement is the addition of functions in seperate files. The ```std``` folder has several examples of this.

# Installation and Usage

Scripts are provided to ease the installation process. For Debian, clone the repository and run:\
\
```./scripts/debian_package --install```\
\
Otherwise, clone the repository and run:\
\
```cargo build```\
\
The compiler manual pages (once the package is installed) can be seen via the command:\
\
```man dummyc```

# Workflows

Currently, three workflows are run each time code is pushed to the repository. These workflows build and test the interpreter on different platforms.
The interpreter is intended to be multiplatform.

While the interpreter supports non-free platforms (such as Windows and MacOS), this is only to provide users of those platforms exposure to free
software. Please consider using a platform which respects your freedom, such as [GNU/Linux](https://www.gnu.org/distros/distros.html). For more information, see [the GNU website](https://www.gnu.org/philosophy/free-sw.en.html).

# References and Acknowledgements

John Gatewood Ham's [ECMA-55 BASIC compiler](https://buraphakit.sourceforge.io/BASIC.shtml) provided a source of inspiration (and tests cases).

The [Free Software Foundation](https://www.fsf.org/) provided the license, the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.en.html).

John G. Kemeny and Thomas E. Kurtz invented [BASIC at Dartmouth College](https://en.wikipedia.org/wiki/BASIC).

# Notes

The project originated as a BASIC to Rust transpiler. The last version of the transpiler can be found in the ```transpiler``` branch. 
Most of the transpiler code was reused in the interpreter before being ultimately refactored. A feature is planned that would allow the 
embedding of a BASIC program in a reduced version of the interpreter. Although the methodology is different, the end result is similar: a 
Rust program which is equivalent in function to a BASIC program.

# Roadmap

The following is a tentative roadmap:

* Finish implementing the ECMA-55 standard
* Pass the complete NBS and HAM test cases
* Implement packaging script for each major platform
* Run the [101 BASIC Computer Games](https://github.com/EriFranca/basic-computer-games)

The package will only progress to version 1.0.0 once regressions in functionality can be prevented with sufficient test cases.

# Contribution

If you use the interpreter and notice bugs or issues, feel free to submit an issue in the issues tab. 

# Disclaimer

This project is a work in progress and subject to change! 
