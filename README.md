# 🌘 Qclipsing-OS

![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)
![In Development](https://img.shields.io/badge/status-WIP-yellow?style=for-the-badge)

---

## 📜 About

The **Qclipsing-OS** is an OS built completely from scratch using **Rust**. It is built around an idea of a **temporal filesystem**.

I am building it by following the [**OSTEP** book](https://pages.cs.wisc.edu/~remzi/OSTEP/), [**OSDev Wiki**](https://wiki.osdev.org/Expanded_Main_Page), and just a little bit of [**Phil Opp's course** - building OS on rust](https://os.phil-opp.com/), while trying to figure out everything by myself. Hence, it will have a unique architecture.

The Qclipsing-OS is going to have proper **memory management**, **filesystem**, **shell**, **drivers**, and other stuff in the future.

---

## 🗂 The Temporal FS

The main feature of the Qclipsing-OS is going to be the **temporal filesystem**, which means the OS will **snapshot the current FS** every little while and store it on the disk.

Therefore, the OS can mount the filesystem at any given point in the past.

Yes, it is kind of similar to Git; however, it is different. It can snapshot individual directories and files, as well as the whole filesystem.

This is extremely useful because:

- It basically makes it impossible to ruin the system by user error, virus, malware, etc.
- It's just interesting to investigate how your system changes over time.

---

## 📌 Development Roadmap

I did sketch a little development plan for Qclipsing-OS:

1. 🧮 **Basic memory allocator**: alternatives to the `malloc()` and `free()` in C/C++
2. ⌨️ **Minimal drivers**: keyboard, screen I/O
3. 🐚 **Simple shell**: just to communicate with the system
4. 🧠 **Expand memory features**: things such as paging and virtual memory🚀
5. 🗄 **Filesystem**: just basic, without temporal features

That’s it for now. After implementing those five features, I will be thinking about what I am going to do next.

---
## 🚀 Getting Started 

To test the OS by ***yourself***, you can either download boot image and boot into it, or build it yourself.

### Download bootable image

1. Go to '/target/x86_64-specification_of_target/debug and download' 'bootimage-qclipsing-os.bin'
2. Boot into OS using any VM, for qemu:
```sh
qemu-system-x86_64 -drive format=raw,file=<path to the boot image>
```
### Build it yourself

#### Prerequirements
1.  Make sure [rustup](https://www.rust-lang.org/tools/install) and Rust are installed.
2.  Install the ***Rust nightly toolchain*** (the version with experimental features enabled) by running:
    ```sh
    rustup install nightly
    ```
3.  Set Rust nightly as the default Rust version for this project with:
    ```sh
    rustup override set nightly
    ```
4.  Install the required Rust components:
    ```sh
    rustup component add rust-src
    rustup component add llvm-tools-preview
    ```
5.  Install the `bootimage` tool (used to create bootable images) with:
    ```sh
    cargo install bootimage
    ```
6.  Install ***QEMU*** (a virtual machine) via your system’s package manager or from the [official website](https://www.qemu.org/download/).

#### Running the OS

1.  Clone this ***repository*** with:
    ```sh
    git clone [https://github.com/Alkotikotik/Qclipsing-os](https://github.com/Alkotikotik/Qclipsing-os)
    ```
2.  Change directory to ***Qclipsing OS*** with:
    ```sh
    cd Qclipsing-os
    ```
3.  Build the OS with:
    ```sh
    cargo bootimage
    ```
4.  Finally, run the OS in ***QEMU*** with:
    ```sh
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-specification_of_target/debug/bootimage-qclipsing-os.bin
    ```

Done, now you can boot into the OS and enjoy a lovely message!

