# nannou sketches
The primary target for these sketches are modern 64 bit Raspberry Pi's.

## issues
- make build_rpi64 does not compile the binary due to wrong environment settings regarging PKGCONFIG. 
However make shell -> ./build_rpi64.sh manually *does* complete successfully.
- [ ] segfaulting on the rpi due to:
  - [ ] missing .drirc files --- no effect
  - [ ] user not added to video group 
