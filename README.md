# 🌍 gm_maxminddb_geoip

This module reads, queries and serializes to Lua tables data from the [MaxMindDB GeoIP](https://www.maxmind.com/en/geoip2-services-and-databases) database.

In other words, IP address goes in, GeoIP data comes out.

# Installation

## Downloading the module

First, run this in your server's console and it will tell you which module you need to download from the [releases page](https://github.com/WilliamVenner/gm_maxminddb_geoip/releases)

```lua
lua_run print("gmsv_maxminddb_geoip_" .. ((system.IsLinux() and "linux" .. (jit.arch == "x86" and "" or "64")) or (system.IsWindows() and "win" .. (jit.arch == "x86" and "32" or "64")) or "UNSUPPORTED") .. ".dll")
```

Then, put the downloaded `dll` file in `garrysmod/lua/bin`, if that folder doesn't exist, create it.

## Downloading the MaxMindDB GeoIP database

[Click here to download the MaxMindDB GeoIP database](https://www.maxmind.com/en/accounts/current/geoip/downloads)

You'll need to create an account unfortunately. Alternatively, you could always [download a sketchy one from GitHub](https://github.com/search?l=&q=extension%3Ammdb&type=code) if you are lazy.

One you've created an account, log in and go to "Download Files" or "Download Databases" and download the database you need.

Once downloaded, copy it to `garrysmod/maxminddb.mmdb` on your server.

### Which MaxMindDB GeoIP database do I need?

If you only need country information, download "GeoLite2 Country" (NOT the CSV one)

<br>

# Developers

All data available in the [`maxminddb` crate documentation](https://docs.rs/maxminddb/0.21.0/maxminddb/geoip2) is serialized and available.

If you want to automate the downloading of the database in some way, the module will also accept a MaxMindDB stored in `garrysmod/data/maxminddb.dat` which can be written to using [`file.Write`](https://wiki.facepunch.com/gmod/file.Write)

<br>

## Rust -> Lua Type Conversions

Not all servers will have the full databases installed, and the free database contains limited data, which most servers will install. Therefore, do note that every field can be `nil`.

|        Rust       |           Lua           | English                     |
|:-----------------:|:-----------------------:|-----------------------------|
| `BTreeMap<K, V>`  | `{ [K] = V, ... }`      | A key-value table           |
| `Vec<T>`          | `{ T, ... }`            | A sequential table          |
| `Option<T>`       | `T \| nil`              | Something that can be `nil` |
| `&str`            | `string`                | Text                        |
| `f64`             | `number`                | A decimal number            |
| `u32` `u16`       | `integer`               | A positive integer          |

<br>

## Loading the module

```lua
require("maxminddb_geoip")

-- Prints the version of the module
print(maxminddb.VERSION)

-- This will reread the database from the disk, useful if you've just written to garrysmod/data/maxminddb.dat
-- You don't need to call this function after require()ing the module, it will load automatically
-- maxminddb.refresh() -> (success: bool, error: [string | nil])
maxminddb.refresh()
```

<br>

## Simple Querying

```lua
-- Provided for convenience
-- maxminddb.country(ipAddress: string, lang: string = "en") -> [(country: string, nil) | (nil, error: string)]
local country, err = maxminddb.country("1.1.1.1", "en")
if err then
    error(err)
else
    print(country) -- "Australia"
end
```

<br>

## Advanced Querying

```lua
-- Advanced querying
-- Records can be found here: https://docs.rs/maxminddb/latest/maxminddb/0.21.0
-- maxminddb.query(ipAddress: string, record) -> [(data: table, nil) | (nil, error: string)]
local data, err = maxminddb.query("1.1.1.1", maxminddb.records.Country)
if err then
    error(err)
else
    PrintTable(data) -- see below
end
```

```lua
-- Database used for example: GeoLite2-Country

continent:
    code        =       "OC"
    geoname_id  =       6255151
    names:
        de      =       "Ozeanien"
        en      =       "Oceania"
        es      =       "Oceanía"
        fr      =       "Océanie"
        ja      =       "オセアニア"
        pt-BR   =       "Oceania"
        ru      =       "Океания"
        zh-CN   =       "大洋洲"

country:
    geoname_id  =       2077456
    iso_code    =       "AU"
    names:
        de      =       "Australien"
        en      =       "Australia"
        es      =       "Australia"
        fr      =       "Australie"
        ja      =       "オーストラリア"
        pt-BR   =       "Austrália"
        ru      =       "Австралия"
        zh-CN   =       "澳大利亚"

registered_country:
    geoname_id  =       2077456
    iso_code    =       "AU"
    names:
        de      =       "Australien"
        en      =       "Australia"
        es      =       "Australia"
        fr      =       "Australie"
        ja      =       "オーストラリア"
        pt-BR   =       "Austrália"
        ru      =       "Австралия"
        zh-CN   =       "澳大利亚"
```