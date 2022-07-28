# Trezor Core

芯片迁移到esp32s3之后的编译步骤

1.  在系统中安装Fork的idf并安装运行，官方idf有bug
```bash
git clone https://github.com/JubiterWallet/esp-idf -b JubiterBase
cd esp-idf
./install.sh
source ./export.sh
```
2. idf与本项目都用了python来构建，为了让两个环境同时起作用，去掉了原trezor的Poetry virtualenv。改与idf使用相同的virtualenv。
```bash
git clone https://github.com/JubiterWallet/trezor-firmware -b JubiterBase
cd trezor-firmware
poetry install
poetry shell
```

3.  由于risc-v的gcc在idf安装的时候会自动安装，所以只安装其他必须的
```bash
sudo apt install scons llvm-dev libclang-dev clang
```

4.  安装rust的risc-v的编译器
``` bash
rustup target add riscv32imac-unknown-none-elf
```

5.  安装protoc的编译器

```bash
sudo apt install -y protobuf-compiler
```

6.  执行编译
```bash
make vendor build_boardloader build_bootloader build_lv_micropython build_firmware 

```