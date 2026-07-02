# Luna OS

Luna is experimental simple operating system designed for one single task: host and serve AI models.

Naive idea is that by stripping everything except the job itself, we might achieve improved performance. In reality, this idea is not unique and was already implemented by number of other projects. However, this is educational project, with no intention of competing with modern commercial solutions

---

Start emulation:
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-luna_os/debug/bootimage-luna_os.bin
```