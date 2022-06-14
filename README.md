# learn [rust os dev](https://rust-osdev.com/) from [phil-opp](https://os.phil-opp.com/)

## dependencies

- [qemu](https://www.qemu.org/)

we will run our os kernel in the QEMU vm(on Linux host machine), so install it first  

- [vnc viewer](https://www.realvnc.com/en/connect/download/viewer/)

if you boot the kernel in QEMU from remote Linux system with no GUI, you can config QEMU with vnc support(see `run-command` and `run-args` in Cargo.toml), then
 use vnc-viewer to connect QEMU vnc server to see kernel screen.
