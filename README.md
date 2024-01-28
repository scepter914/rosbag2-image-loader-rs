# rosbag2-image-loader-rs

This crate is rosbag2 image loader by rust.

- Document
  - [crates.io](https://crates.io/crates/rosbag2_image_loader)
  - [docs.rs](https://docs.rs/rosbag2_image_loader/)
- Supported feature
  - [x] Read `sensor_msg/msg/Image`
  - [x] Read `sensor_msg/msg/CameraInfo`
  - [ ] Read `sensor_msg/msg/CompressedImage`

## Get started

### Run example

```sh
cargo run --release --example example {path_to_rosbag2}
# example
cargo run --release --example example data/rosbag/rosbag2_2022_02_05-00_54_33/rosbag2_2022_02_05-00_54_33_0.db3
```

### Use in your application

- Cargo.toml

```toml
rosbag2_image_loader = "0.1.2"
```

- In detail, see example code.

```rust
    // load Rosbag2Images interface
    let mut interfaces: Vec<Rosbag2Images> = load_images_from_rosbag2(file_name).unwrap();

    if !interfaces.is_empty() {
        loop {
            frame_index += 1;
            let input_image = interfaces[0].get_frame();
            if input_image.is_none() {
                break;
            }
            my_image_proc(&input_image.unwrap(), frame_index);
        }
```

- Rosbag2Images can be used as [simple_image_interface](https://github.com/scepter914/simple-image-interface-rs).

## History

- For new release
  - Updated docs
- v0.1.2
  - Updated docs
- v0.1.1
  - Updated docs
- v0.1.0
  - Published `rosbag2-image-loader`

