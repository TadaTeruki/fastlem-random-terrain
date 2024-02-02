# fastlem-random-terrain

A simple random terrain generator using [fastlem](https://crates.io/crates/fastlem).

## build & run

### Using cargo

```
$ cargo build --release
```

The binary is generated in `target/release/fastlem-random-terrain`. If you want to move it into current directory, run the following command.
```
$ mv target/release/fastlem-random-terrain .
```

### Using docker

```
$ docker build -t fastlem-random-terrain .
```

The image is generated as `fastlem-random-terrain`. To run:
```
$ docker run -it fastlem-random-terrain fastlem-random-terrain
```

Command line options can be passed as follows:

```
$ docker run -it fastlem-random-terrain fastlem-random-terrain --help
```

## Options

```
$ fastlem-random-terrain --help
```

```
Usage: fastlem-random-terrain [OPTIONS]

Options:
  -b, --bound <BOUND>
          Width and height (width:height) of the bound [default: 100.0:100.0]
  -s, --seed <SEED>
          Seed of the noise generator [default: 0]
  -p, --particle-num <PARTICLE_NUM>
          Number of particles. The larger the value, the more the quality of the terrain is improved. [advanced] [default: 50000]
  -f, --fault-scale <FAULT_SCALE>
          Scale of the fault. The larger the value, the more virtual faults effect the terrain. [advanced] [default: 35.0]
  -e, --erodibility-distribution-power <ERODIBILITY_DISTRIBUTION_POWER>
          Power of the erodibility distribution. The larger the value, the more the erodibility is concentrated on the lower side. [advanced] [default: 4.0]
  -c, --colormap-json-file <COLORMAP_JSON_FILE>
          JSON file of the colormap. If not specified, the grayscale colormap is used [default: ]
  -i, --image-size <IMAGE_SIZE>
          Width and height (width:height) of the image. If -1 is specified, the aspect ratio is the same as the bound [default: 1024:-1]
  -i, --image-file <IMAGE_FILE>
          File name of the output image [default: terrain.png]
  -h, --help
          Print help
```

## License

MPL-2.0
