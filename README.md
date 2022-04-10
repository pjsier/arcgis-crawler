# arcgis-crawler

[![crates.io](https://img.shields.io/crates/v/arcgis-crawler.svg)](https://crates.io/crates/arcgis-crawler)
[![Build status](https://github.com/pjsier/arcgis-crawler/workflows/CI/badge.svg)](https://github.com/pjsier/arcgis-crawler/actions?query=workflow%3ACI)

Crawl ArcGIS servers and report all available services

## Installation

If you have `cargo` installed, you can run `cargo install arcgis-crawler` and then run it from `$HOME/.cargo/bin`. More details on this are available in [`cargo-install` documentation](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

## Usage

You can run the command against the `/services` path of ArcGIS server with JSON endpoints enabled

```
arcgis-crawler https://gisapps.cityofchicago.org/arcgis/rest/services/
https://gisapps.cityofchicago.org/arcgis/rest/services/
├─ 311
│  ├─ 311_layers
│  │  └─ MapServer
│  │     ├─ ALLEYNAM
│  │     ├─ ASPHALT_DISTRICTS
│  │     ├─ COMAREA
│  │     ├─ CONGRDIS
│  │     ├─ CPS_SAFE_PASSAGE_BUFFER
│  │     ├─ CircuitSegments
│  │     ├─ Circuit_Line
...
```
