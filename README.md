<div align="center">

# Dinkleberg

Find out who is pretending to be offline

<img src="assets/icon.png" alt="logo" width="500"/>

<!--

<br>

[![Build project](https://github.com/oSumAtrIX/dinkleberg/actions/workflows/rust.yml/badge.svg)](https://github.com/oSumAtrIX/dinkleberg/actions/workflows/rust.yml)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/4474e5fcc9064562b5d653601ee356f3)](https://www.codacy.com/gh/oSumAtrIX/DownOnSpot/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=oSumAtrIX/DownOnSpot&amp;utm_campaign=Badge_Grade)
[![GitHub license](https://img.shields.io/github/license/oSumAtrIX/dinkleberg)](https://github.com/oSumAtrIX/dinkleberg/blob/main/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/oSumAtrIX/dinkleberg)](https://github.com/oSumAtrIX/dinkleberg/issues)
[![GitHub forks](https://img.shields.io/github/forks/oSumAtrIX/dinkleberg)](https://github.com/oSumAtrIX/dinkleberg/network)
[![GitHub stars](https://img.shields.io/github/stars/oSumAtrIX/dinkleberg)](https://github.com/oSumAtrIX/dinkleberg/stargazers)
[![Stability: Experimental](https://masterminds.github.io/stability/experimental.svg)](https://masterminds.github.io/stability/experimental.html)
-->

</div>

## Preview

<img src="assets/preview.png" alt="preview image"/>

## Disclaimer

```text
Dinkleberg was developed for educational, private and fair use.
I am not responsible in any way for the usage of the source code.
```

## Features

-   Rust
-   Automatically track an entire guild and its users
-   Fancy colored output
-   Find out who really goes offline or just pretends to
-   Easy to use

## Building

Clone the repository using git and change to the local repository directory:

```bash
git clone https://github.com/oSumAtrIX/dinkleberg.git
cd dinkleberg
```

`Nightly Rust` is required to build this project. Install it by following [rustup.rs](https://rustup.rs) instructions.

```bash
cargo build --release
```

## Bot setup

The bot needs the following gateway intents to operate.

`GUILD_PRESENCES`

`DISCORD_TOKEN`

## Environment variables

To use this project you will need to set the following enviroment key with your discord token as the value:

`DISCORD_TOKEN`

## Usage/ Examples

```text
$ dinkleberg.exe

'########::'####:'##::: ##:'##:::'##:'##:::::::'########:'########::'########:'########:::'######:::
 ##.... ##:. ##:: ###:: ##: ##::'##:: ##::::::: ##.....:: ##.... ##: ##.....:: ##.... ##:'##... ##::
 ##:::: ##:: ##:: ####: ##: ##:'##::: ##::::::: ##::::::: ##:::: ##: ##::::::: ##:::: ##: ##:::..:::
 ##:::: ##:: ##:: ## ## ##: #####:::: ##::::::: ######::: ########:: ######::: ########:: ##::'####:
 ##:::: ##:: ##:: ##. ####: ##. ##::: ##::::::: ##...:::: ##.... ##: ##...:::: ##.. ##::: ##::: ##::
 ##:::: ##:: ##:: ##:. ###: ##:. ##:: ##::::::: ##::::::: ##:::: ##: ##::::::: ##::. ##:: ##::: ##::
 ########::'####: ##::. ##: ##::. ##: ########: ########: ########:: ########: ##:::. ##:. ######:::
........:::....::..::::..::..::::..::........::........::........:::........::..:::::..:::......::::

Usage: dinkleberg.exe <guid_id>
```

## Known issues

-   Occasional false detection of mobile clients

## Authors

-   [@oSumAtrIX](https://osumatrix.me/#github)

## License

[GPL3](https://choosealicense.com/licenses/agpl-3.0/)
