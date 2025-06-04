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
4. 🧠 **Expand memory features**: things such as paging and virtual memory
5. 🗄 **Filesystem**: just basic, without temporal features

That’s it for now. After implementing those five features, I will be thinking about what I am going to do next.

---

## 🌠 The Very Future

In the very future, I am planning to implement the following projects for Qclipsing-OS:

- A **programming language and compiler** for it  
- **Connection to the internet** through a protocol  
- A **physics engine**  
- And some more stuff — but we'll see about it 😄

---

## 🎥 YouTube Channel

I am also going to make a blog and upload it on **YouTube** just to capture the progress.

Stay tuned! 🚀

