```
# https://home.nature.global/home
export NATURE_REMO_TOKEN=
cargo run -- list
```

```
$ cargo run -- list
...
- メイン (ID: ...)
  - スイッチ (ID: ...)
  - エアコン (ID: c36c3a20-6d40-4e78-acdb-fb8f223e0f2b)
  - リビング照明 (ID: ...)
- Remo E lite (ID: ...)
  - Smart meter (ID: ...)
...
```

```
$ cargo run -- show --appliance c36c3a20-6d40-4e78-acdb-fb8f223e0f2b
...
State for エアコン (ID: c36c3a20-6d40-4e78-acdb-fb8f223e0f2b)
Type: AC
  Temperature: 25
  Mode: cool
  Volume: 1
  Direction: auto
  Button:
```


```
cargo run -- set --appliance c36c3a20-6d40-4e78-acdb-fb8f223e0f2b --temperature 25
```
