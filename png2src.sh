#!/bin/sh
w4 png2src --template png2src_template.txt --rust assets/objects.png > src/assets/objects_png.rs
w4 png2src --template png2src_template.txt --rust assets/controls.png > src/assets/controls_png.rs
