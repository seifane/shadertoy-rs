[![Build Status](https://travis-ci.org/fmenozzi/shadertoy-rs.svg?branch=master)](https://travis-ci.org/fmenozzi/shadertoy-rs)
[![Crates.io Version](https://img.shields.io/crates/v/shadertoy-rs.svg)](https://crates.io/crates/shadertoy-rs)

Desktop [Shadertoy](https://www.shadertoy.com) client, written in Rust.

While it's still a work in progress, you can run some example shaders to see that it's working:

```
cargo run --release -- --example seascape
cargo run --release -- --example elemental-ring
```

Make sure you build/run in release mode; textures take several seconds to load in debug mode. You can also download from [crates.io](https://crates.io/crates/shadertoy-rs).

So long as you restrict yourself to the supported uniforms, shaders copy-pasted directly from Shadertoy should run with no modifications (if they don't, feel free to file an issue or open a pull request). The following uniforms are currently supported, with more coming soon:

* `iGlobalTime`
* `iTime` (same as `iGlobalTime`; `iGlobalTime` will eventually be deprecated by Shadertoy)
* `iResolution`
* `iMouse`
* `iFrame`
* `iChannel0`, `iChannel1`, `iChannel2`, `iChannel3`
    * These are 2D RGBA textures

You can press `F5` to reload the shader if you've edited it since launching the app.

You can also download (and optionally run) shaders directly from Shadertoy if you have the URL or shader ID. For example, to download the classic [Seascape](https://www.shadertoy.com/view/Ms2SD1) shader, you can run

```
shadertoy get https://www.shadertoy.com/view/Ms2SD1
```

or just

```
shadertoy get Ms2SD1
```

Add the `-r/--run` flag to automatically run the downloaded shader

For now, the CLI looks like this:

```
shadertoy 0.6.2
Federico Menozzi <federicogmenozzi@gmail.com>
Desktop client for Shadertoy

USAGE:
    shadertoy [FLAGS] [OPTIONS] [shader] [SUBCOMMAND]

FLAGS:
        --force_srgb_off    Forces srgb to be off. (Use this you have color blending issues)
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -e, --example <example>      Run example shader from examples/ directory
    -H, --height <height>        Sets window height [default: 400]
        --texture0 <texture0>    Path to 2D RGBA texture for iChannel0
        --texture1 <texture1>    Path to 2D RGBA texture for iChannel1
        --texture2 <texture2>    Path to 2D RGBA texture for iChannel2
        --texture3 <texture3>    Path to 2D RGBA texture for iChannel3
    -W, --width <width>          Sets window width [default: 600]
    -t, --title <title>          Sets the window title
        --wrap0 <wrap0>          Wrap mode for iChannel0 [default: repeat]
                                 [possible values: clamp, repeat, mirror, border]
        --wrap1 <wrap1>          Wrap mode for iChannel1 [default: repeat]
                                 [possible values: clamp, repeat, mirror, border]
        --wrap2 <wrap2>          Wrap mode for iChannel2 [default: repeat]
                                 [possible values: clamp, repeat, mirror, border]
        --wrap3 <wrap3>          Wrap mode for iChannel3 [default: repeat]
                                 [possible values: clamp, repeat, mirror, border]
        --filter0 <filter0>      Filtering for iChannel0 [default: mipmap]
                                 [possible values: scale, mipmap, bilinear, trilinear, anisotropic]
        --filter1 <filter1>      Filtering for iChannel1 [default: mipmap]
                                 [possible values: scale, mipmap, bilinear, trilinear, anisotropic]
        --filter2 <filter2>      Filtering for iChannel2 [default: mipmap]
                                 [possible values: scale, mipmap, bilinear, trilinear, anisotropic]
        --filter3 <filter3>      Filtering for iChannel3 [default: mipmap]
                                 [possible values: scale, mipmap, bilinear, trilinear, anisotropic]
        --anisotropic_max <max>  Max steepness for anisotropic filtering (1-16) [default: 1]


ARGS:
    <shader>    Path to fragment shader

SUBCOMMANDS:
    get     Download shaders from shadertoy.com
    help    Prints this message or the help of the given subcommand(s)
````
