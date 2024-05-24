 #!/bin/bash
 
 # 打印欢迎信息
echo "bootimage script start"

cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
