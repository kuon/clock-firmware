
# THIS REPOSITORY HAS MOVED

New URL: https://git.goyman.com/kuon/clock-firmware

Why I moved everything out of GitHub:

https://github.com/kuon/WhyILeftGithub/blob/main/README.md

----


# THIS REPOSITORY HAS MOVED

New URL: https://git.goyman.com/kuon/clock-firmware

Why I moved everything out of GitHub:

https://github.com/kuon/WhyILeftGithub/blob/main/README.md

----


# THIS REPOSITORY HAS MOVED

New URL: https://git.goyman.com/kuon/clock-firmware

Why I moved everything out of GitHub:

https://github.com/kuon/WhyILeftGithub/blob/main/README.md

----

## Clock firmware

I built a clock with giant 7 segments LEDs and this is the beaglebone
black firmware to control it.

I wrote it in Rust to learn the language a bit.

### Cross compilation

Configure armv7 target to cargo config.

Add to `~/.cargo/config`

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

