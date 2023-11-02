# Playdate-docdef Design

## What is this?

tldr: This is an overly complicated tool with the following parts:

1. DSL (domain specific language) for Lua typing information (LUARS)
2. PEG Grammar for LUARS using the [pest.rs](https://pest.rs) crate (see: [luars.pest](src/luars.pest))
3. Manually documented Types for the PlaydateSDK expressed in LUARS (see: [playdate.luars](playdate.luars))
4. A web scraper which scrapes the PlaydateSDK documentation using the [scraper](https://crates.io/crates/scraper) crate.
5. A rust which combines the scraped documentation and types and generates LuaLS compatible Type Annotations with documentation and types for the entire SDK.

## No seriously, what the actual fuck?

1. I've been coding Lua for Playdate.
2. The PlaydateSDK has no types and leverages Lua dynamic nature for variadic params and returns.
3. There's a pretty good Language server for Lua which created a standard for Lua typing via comments (LuaCATS)
4. I've been learning rust.
6. I wrote a scraper for the PlaydateSDK docs (in rust)
7. For each function I generated a TOML skeleton and manually determined parameter and return types.
8. TOML turned out to overly verbose, fragile and unsearchable for ~1000 functions and ~3500 type definitions (~15K lines).
9. So I came up with a format for function signatures which only requires one line per function.
10. I learned PEG and wrote a parser for function signature format I came up with.
11. Iterate. Iterate. Iterate.

Now we can do static code analysis, type checking and autocomplete in VSCode and other IDEs
that support the LuaLS Language Server's LUACATS style type annotation comments.
Just use [notpeter/playdate-luacats](https://github.com/notpeter/playdate-luacats).

## LUARS format documentation

Semicolon terminated statements which begin with `global|local|fun`.

Global is for tables in the global `playdate.*` table.

Local is for table types used by the SDK.

Fun is functions (and parameters, parameter types, returns, return types and optionality of each).

See the [LUARS pest.rs PEG Grammar](src/luars.pest) and [Playdate.luars](playdate.luars) for more.
