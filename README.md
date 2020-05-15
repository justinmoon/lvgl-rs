<h1 align="center"> LittlevGL - Open-source Embedded GUI Library in Rust</h1>

<p align="center">
<img src="https://littlevgl.com/github/cover_ori_reduced_2.gif">
</p>

<p align="center">
LittlevGL provides everything you need to create a Graphical User Interface (GUI) on embedded systems with easy-to-use graphical elements, beautiful visual effects and low memory footprint. 
</p>
<p align="center">
LittlevGL is compatible with <samp>#![no_std]</samp> environments by default.
</p>

<h4 align="center">
<a href="https://littlevgl.com">Official LittlevGL Website </a> &middot; 
<a href="https://github.com/littlevgl/lvgl">C library repository</a> &middot;
<a href="https://littlevgl.com/live-demo">Live demo</a>
</h4>

---

![Rust bindings usage demo code.](demo.png)

## Usage

Edit your `Cargo.toml` file dependencies with:
```
$ cargo add lvgl
```

The build requires the environment variable bellow to be set:

- `DEP_LV_CONFIG_PATH`: Path to the directory containing the `lv_conf.h` header file used for configuration of LittlevGL library.

We recommend the `lv_conf.h` file to be in your project's root directory. If so, the command to build your project would be:
```shell script
$ DEP_LV_CONFIG_PATH=`pwd` cargo build
```

## Running the demo

[This project contains an example that can run in a desktop simulator.](./examples/demo)
