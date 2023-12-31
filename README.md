# Hyperview Asset Tool

> [!NOTE]  
> This project is under active development. 

Hyperview asset tool (hvat) is a command line program to read and search for data within Hyperview.

# Usage

To use this tool simply download a pre-built binary from the [Releases](https://github.com/HyperviewHQ/asset_tool/releases) section.

```console
A command line interface to interact with asset data stored in Hyperview

Usage: hvat [OPTIONS] <COMMAND>

Commands:
  list-asset-properties         List asset properties
  list-custom-asset-properties  List custom asset properties
  search-assets                 Search assets
  help                          Print this message or the help of the given subcommand(s)

Options:
  -d, --debug-level <DEBUG_LEVEL>  Debug level [default: error] [possible values: error, warn,
         debug, info, trace]
  -h, --help                       Print help
  -V, --version                    Print version
```

The `list-asset-properties` command will list all set, inherited and available properties for an asset id. 

The `list-custom-asset-properties` command will list all set and available custom properties for an asset id.

The `search-assets` command is the main entry point for the application and it provides various methods to search for
assets in Hyperview.

```console
Usage: hvat search-assets [OPTIONS]

Options:
  -p, --search-pattern <SEARCH_PATTERN>
          Search pattern or string, e.g. chrome [default: *]
  -t, --asset-type <ASSET_TYPE>
          Optional asset type, e.g. Crah [possible values: BladeEnclosure, BladeNetwork,
          BladeServer, BladeStorage, Busway, Camera, Chiller, Crac, Crah, Environmental,
          FireControlPanel, Generator, InRowCooling, KvmSwitch, Location, Monitor,
          NetworkDevice, NetworkStorage, NodeServer, PatchPanel, PduAndRpp, PowerMeter,
          Rack, RackPdu, Server, SmallUps, TransferSwitch, Unknown, Ups, VirtualServer]
  -c, --location-path <LOCATION_PATH>
          Optional prefix of location path, e.g. "All/"
  -P, --properties <PROPERTIES>
          Optional property or custom property to filter on, e.g. serialNumer=SN1234567890
  -C, --custom-properties <CUSTOM_PROPERTIES>
          Optional custom property or custom property to filter on, e.g. serialNumer=SN1234567890
  -i, --id <ID>
          Primary ID. It must be a valid GUID/UUID, e.g. 2776f6c6-78da-4087-ab9e-e7b52275cd9e
  -M, --manufacturer <MANUFACTURER>
          Manufacturer name, e.g. dell
  -R, --product <PRODUCT>
          Product name, e.g. poweredge
  -s, --skip <SKIP>
          Number of records to skip (0 -> 99999), e.g. 100 [default: 0]
  -l, --limit <LIMIT>
          Record limit (1 -> 1000), e.g. 100 [default: 100]
  -o, --output-type <OUTPUT_TYPE>
          Output type, e.g. csv-file [default: record] [possible values: csv-file, json, record]
  -f, --filename <FILENAME>
          output filename, e.g. output.csv
  -h, --help
          Print help
  -V, --version
          Print version
```

All commands allow the user to set to output to `record`, `json` or `csv-file`. Most commands give the user the ability to provide an output file. Where applicable, users can limit the output records and the number of records to skip. 

## Examples

### Search by property (JSON output)

```console
hvat search-assets -P serialNumber=SERIALNUMBEREXAMPLE1234 -o json
[
  {
    "id": "\"58af63dc-1e9e-4b8b-b2b7-e0451aaca8fb\"",
    "name": "\"UpsExample\"",
    "assetLifecycleState": "\"Active\"",
    "assetTypeId": "\"Ups\"",
    "manufacturerId": "\"cd85e92d-869c-470a-a3ba-df8b2b7196e3\"",
    "manufacturerName": "\"Liebert\"",
    "monitoringState": "\"On\"",
    "parentId": "\"9a877a93-1f21-4895-a078-5c67f531ea0b\"",
    "parentName": "\"Simulated SNMP Devices\"",
    "productId": "\"aedbd4b9-06ae-4768-ba4a-64847b60d334\"",
    "productName": "\"eXM\"",
    "status": "\"Normal\"",
    "path": "\"All/Simulated SNMP Devices/UpsExample\"",
    "serialNumber": "[\"SERIALNUMBEREXAMPLE1234\"]"
  }
]

```

### Search by text pattern (record output)

```console
hvat search-assets -p "UpsExampl*"
---- [0] ----
id: "58af63dc-1e9e-4b8b-b2b7-e0451aaca8fb"
name: "UpsExample"
asset_lifecycle_state: "Active"
asset_type_id: "Ups"
manufacturer_id: "cd85e92d-869c-470a-a3ba-df8b2b7196e3"
manufacturer_name: "Liebert"
monitoring_state: "On"
parent_id: "9a877a93-1f21-4895-a078-5c67f531ea0b"
parent_name: "Simulated SNMP Devices"
product_id: "aedbd4b9-06ae-4768-ba4a-64847b60d334"
product_name: "eXM"
status: "Normal"
path: "All/Simulated SNMP Devices/UpsExample"
serial_number: ["SERIALNUMBEREXAMPLE1234"]
```

### Combination search (JSON output)

```console
hvat search-assets -p "UpsExample" --location-path "All/Simulated SNMP Devices/" -M "Liebert" -o json
[
  {
    "id": "\"58af63dc-1e9e-4b8b-b2b7-e0451aaca8fb\"",
    "name": "\"UpsExample\"",
    "assetLifecycleState": "\"Active\"",
    "assetTypeId": "\"Ups\"",
    "manufacturerId": "\"cd85e92d-869c-470a-a3ba-df8b2b7196e3\"",
    "manufacturerName": "\"Liebert\"",
    "monitoringState": "\"On\"",
    "parentId": "\"9a877a93-1f21-4895-a078-5c67f531ea0b\"",
    "parentName": "\"Simulated SNMP Devices\"",
    "productId": "\"aedbd4b9-06ae-4768-ba4a-64847b60d334\"",
    "productName": "\"eXM\"",
    "status": "\"Normal\"",
    "path": "\"All/Simulated SNMP Devices/UpsExample\"",
    "serialNumber": "[\"SERIALNUMBEREXAMPLE1234\"]"
  }
]
```

# Configuration
A valid Hyperview API client must be used. The API client must have the appropriate access. The configuration file must be placed in `$HOME/.hyperview/hyperview.toml`

## Example

```console
client_id = 'c33472d0-c66b-4659-a8f8-73c289ba4dbe'
client_secret = '2c239e21-f81b-472b-a8c3-82296d5f250d'
scope = 'HyperviewManagerApi'
auth_url = 'https://example.hyperviewhq.com/connect/authorize'
token_url = 'https://example.hyperviewhq.com/connect/token'
instance_url = 'https://example.hyperviewhq.com'
```

# Building

## Linux
If you are experimenting with the code on a single platform the usual `cargo build` and `cargo build --release` will work. However, if the desire is to build a binary that can run on multiple Linux distributions it is recommended to install the `x86_64-unknown-linux-musl` target and to build a statically-linked binary to avoid dependency problems. 

The command to build a statically-linked version is:

```console
PKG_CONFIG_SYSROOT_DIR=/ RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-unknown-linux-musl --release
```

## Windows & MacOS
The usual `cargo build` and `cargo build --release` will work. 
