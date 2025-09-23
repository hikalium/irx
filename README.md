# irx

A universal command-line tool for smart remote controllers.

Currently, only Nature Remo is supported.

## Setup

### 1. Set Access Token

Set your Nature Remo access token as an environment variable. You can get your token from [home.nature.global](https://home.nature.global/).

Create `.env` file with following lines (or set as an environment variable):

```bash
NATURE_REMO_TOKEN='YOUR_NATURE_REMO_TOKEN'
```

## Usage

### `list`: List devices and appliances

Lists all registered devices and their associated appliances.

```bash
cargo run -- list
```

**Example Output:**
```
- Living Room Remo (ID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
  - Air Conditioner (ID: yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy)
  - Light (ID: zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz)
- Bed Room Remo (ID: aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa)
```

### `show`: Show appliance state

Displays the current state of the appliance specified by the `--appliance` ID.

```bash
cargo run -- show --appliance <APPLIANCE_ID>
```

**Example:**
```bash
cargo run -- show --appliance yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy
```

**Example Output:**
```
State for Air Conditioner (ID: yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy)
Type: AC
  Temperature: 25
  Mode: cool
  Volume: auto
  Direction: swing
  Button:
```

### `set`: Set appliance state

Modifies the state of the appliance specified by the `--appliance` ID.  
Currently, only setting the temperature (`--temperature`) for air conditioners is supported.

```bash
cargo run -- set --appliance <APPLIANCE_ID> --temperature <TEMPERATURE>
```

**Example:**
```bash
cargo run -- set --appliance yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy --temperature 27
```

## License

This project is licensed under the MIT License.
