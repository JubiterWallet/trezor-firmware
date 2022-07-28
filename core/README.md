# Trezor Core

芯片迁移到esp32s3之后的编译步骤


1.  由于risc-v的gcc在idf安装的时候会自动安装，所以只安装其他必须的
```bash
sudo apt install scons llvm-dev libclang-dev clang
```

2.  安装rust的risc-v的编译器
``` bash
rustup target add riscv32imac-unknown-none-elf
```

3.  安装protoc的编译器

```bash
sudo apt install -y protobuf-compiler
```

4.  在系统中安装Fork的idf并安装运行，官方idf有bug
```bash
git clone https://github.com/JubiterWallet/esp-idf -b JubiterBase
cd esp-idf
./install.sh
source ./export.sh     #每次新运行bash，都要export
```
5. idf与本项目都用了python来构建，为了让两个环境同时起作用，去掉了原trezor的Poetry virtualenv。改与idf使用相同的virtualenv。
```bash
git clone https://github.com/JubiterWallet/trezor-firmware -b JubiterBase
cd trezor-firmware
poetry install
poetry shell
```


6.  执行编译
```bash
make vendor build_boardloader_esp32s3 build_bootloader_esp32s3 build_lv_micropython build_firmware_esp32s3 

```