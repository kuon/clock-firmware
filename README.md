Configure armv7 target to cargo config.

Add to `~/.cargo/config`

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

