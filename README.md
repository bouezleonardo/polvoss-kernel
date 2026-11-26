# POLVOSS kernel

## Build

### Debug
```bash
cargo kdebug
```

### Release
```bash
cargo krelease
```

## Assembly generation

```bash
cargo kasm
```

## Documentation

```bash
cargo kdoc
```

## Code analysis

```bash
cargo kclippy
```

## Clean

```bash
cargo kclean
```

## Running the kernel in a virtual machine

### QEMU

```bash
qemu-system-riscv32 \
    -machine virt \
    -m 4M \
    -bios none \
    -kernel <path_to_kernel_executable> \
    -nographic
```

### POLVOSS-VM

Using [POLVOSS-VM](https://gitlab.com/ianmarcosgomesefreitas/polvoss-vm-rs):

```bash
cargo run --release <path_to_kernel_executable>
```

## Terminal commands

To execute terminal commands inside the POLVOSS shell, simply type the command name (e.g., `clear`).

To execute a program, use `./<program_name>`.
