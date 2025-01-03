# nannou sketches
The primary target for these sketches are modern 64 bit Raspberry Pi's.

## issues
- make build_rpi64 does not compile the binary due to wrong environment settings regarging PKGCONFIG. 
However make shell -> ./build_rpi64.sh manually *does* complete successfully.

- Need to create a custom device descriptor with lower limits, this at least allows for the creation of the window to not segfault

- The rendering of relative simple examples is not going too well; many, well.. only artefacts. 

This project is on hiatus, keeping it around as a reference for the cross-compling docker image.
