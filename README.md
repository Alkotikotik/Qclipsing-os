# Qclipsing-OS
The Qclipsing-OS is an OS build from very scratch using rust it is build around an idea of temporal filesystem. I am building it by following OSTEP book, OSDEV wiki and a little bit Phil Opp's course while trying to figure out everything by myself hence it will have unique architeture. The Qclipsing-OS is going to have proper memory mangment, filesystem, shell, drivers and other stuff in the future. 

# The temporal FS
The main feature of the Qclipsing-OS is going to be the temporal filesystem, which means that OS will snapshot current FS every little while and storing it on the disk. And so therefore OS can mount FS at any given point in the past. Yes it is kind of similar to git, however it is different. It can snaphot invividual directories and files as well as whole FS. This is extrimely useful because firstly it basically makes it impossible to ruin the system by user error, or any virus, malware etc. And secondly it is just interesting to invistigate how your system changes over time.

# The development priority
I did scatch a little development plan for Qclipsing os:

 1) Basic memory allocator: alternatives to the malloc() and free() in c/c++
 2) Minimal drivers: keyboard screen I/Oa
 3) Simple Shell: just to communicate with the system
 4) Expand memory features: things such as paging and virtual memory
 5) FS: just basic without temporal features

Thats it for now, after implement those 5 feature I will be thinking about what I am going to next.

# The very future
In the very future I am planning to implementfollowing project for Qclipsing-OS: programing languge and compilator for it, connection to the internet thorugh protacol, physics engine and some more stuff, but we'll see about it.

# Youtube chanell 
I am also going to make a blog and upload it on youtube just to capture the progress. 
