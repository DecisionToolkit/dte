**dsntk** | Decision Toolkit

# Decision table editor

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/dte.svg
[crates-url]: https://crates.io/crates/dte
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://opensource.org/licenses/MIT
[mit-license-url]: https://github.com/dsntk/dte/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://www.apache.org/licenses/LICENSE-2.0
[apache-license-url]: https://github.com/dsntk/dte/blob/main/LICENSE
[apache-notice-url]: https://github.com/dsntk/dte/blob/main/NOTICE
[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg
[cc-url]: https://github.com/dsntk/dte/blob/main/CODE_OF_CONDUCT.md
[repository-url]: https://github.com/DecisionToolkit/dte

## Overview

Decision table editor.

## Project status

Work in progress.

## Installation

```shell
$ cargo install dte
```

## Keystrokes

| Key           | Action                                              |
|---------------|-----------------------------------------------------|
| CTRL + Q      | Quit without saving.                                |
| Arrow right   | Move cursor one character right                     |
| Arrow left    | Move cursor one character left                      |
| Arrow up      | Move cursor one row up                              |
| Arrow down    | Move cursor one row down                            |
| End           | Move cursor to the end of the current cell          |
| Home          | Move cursor to the beginning of the current cell    |
| PgUp          | Move cursor to the top row of the current cell      |
| PgDown        | Move cursor to the bottom row of the current cell   |
| CTRL + End    | Move cursor to the end of the current row           |
| CTRL + Home   | Move cursor to the beginning of the current row     |
| CTRL + PgUp   | Move cursor to the top row of the current column    |
| CTRL + PgDown | Move cursor to the bottom row of the current column |
| Backspace     | Delete character before the cursor                  |
| Delete        | Delete character under the cursor                   |
| Any character | Insert character at the cursor position             |
| Insert        | Toggle cursor from caret to block and back          |
| ALT + Insert  | Toggle cursor from caret to underscore and back     |

## Example decision table

```text

 ┌─────────────────┐
 │  Order options  │
 ├───┬───────────┬─┴─────╥─────────────────────╥─────────────┬───────────┐
 │ U │           │       ║    Order options    ║             │           │
 │   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
 │   │   type    │ size  ║ Discount │ Priority ║             │           │
 │   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
 │   │"Business",│       ║          │"Normal", ║             │           │
 │   │"Private"  │       ║          │ "High",  ║             │           │
 │   │           │       ║          │ "Low"    ║             │           │
 ╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
 │ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
 ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
 │ 2 │"Business" │ >=10  ║   0.15   │  "High"  ║ Large order │   Ref 2   │
 ├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
 │ 3 │"Private"  │   -   ║   0.05   │  "Low"   ║ All orders  │   Ref 3   │
 └───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘

```

Copy and save this decision table to any file, e.g. `e.txt` and run the editor:

```shell
$ dte e.txt
``` 

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [**dte**][repository-url] are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
