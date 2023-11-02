# playdate-docdef

## What am I looking at?

Unofficial LuaCATS types for the PlayDate API:

* [stub.lua](stub.lua) (api spec & types only)
* [notpeter/playdate-luacats stub.lua](https://github.com/notpeter/playdate-luacats/)
(api sec, types and scraped text Official Function Docs)

This repository contains the Playdate Lua API specification
and tools for generating Lua Comment And Type System (LUACats)
compliant Lua comments for use with the Lua Language Server
([sumneko.lua](https://marketplace.visualstudio.com/items?itemName=sumneko.lua))
in VSCode, NeoVIM, etc.

These tools can optionally integrate the offical
Playdate Docs into the generated output by scraping the API
docs either locally (e.g. `"~/Developer/PlaydateSDK/Inside Playdate.html"`)
or directly from [https://sdk.play.date](https://sdk.play.date)
to generate a [fully annotated stub.lua](https://github.com/notpeter/playdate-luacats/blob/main/library/stub.lua)

## Usage / Contributing

If you have Playdate type annotation changes/addition just edit [Playdate.luars](playdate.luars)
and then run `cargo run > stub.lua` to see the changes reflected.

For the complete set of annotations: `cargo run -- annotate > stub_annotated.lua`

See: [DESIGN.md](DESIGN.md) for more.

I'm happy to accept corrections/additions to my PlaydateSDK API type annotations
or the rats nest of novice rust code.

## License

Copyright (c) 2023 Peter Tripp

This project is licensed under the [Apache License, Version 2.0](LICENSE-APACHE)
or the [MIT license](LICENSE-MIT) at your option.
