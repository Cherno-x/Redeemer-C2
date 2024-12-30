https://bbs.kanxue.com/thread-284782.htm
Redeemer C2是一款使用Rust编写的平台型C2，旨在滥用可信域名的API平台进行命令控制，用来对抗恶意域名巡查，是一款专注于权限维持的C2工具。

目前已经支持的平台
1. Github

后续安排支持的平台
1. gitee
2. 云OSS
3. Wechat
4. ...

## Feature
- [x] Multiplayer-mode
- [x] Credible TeamServer
- [x] COFF Loader
- [ ] Dynamic code generation->shellcode generator
- [ ] Linux rootkit
- [ ] In-memory .NET assembly execution
- [ ] Encrypt integration
- [ ] Much more!


## Quick Start

Github TeamServer：
Github TeamServer是利用仓库的issue提交功能进行远程控制的，对提交和返回的信息均进行加密处理。
https://github.com/settings/tokens

安装Rust编译环境（https://www.rust-lang.org/），使用nightly版本开发
```shell
cd redeemer-c2
rustup install nightly
rustup override set nightly
# rustup override unset 恢复标准版
rustup run nightly cargo build --release
```
```shell
#跨平台交叉编译- windows—>linux
rustup toolchain install nightly 
rustup target add x86_64-unknown-linux-gnu --toolchain nightly
rustup run nightly cargo build --target x86_64-unknown-linux-gnu --release
```
配置config.yaml文件

1. 创建一个新的仓库，最好使用匿名github账户
![[image-20241008103813136.png]]
2. 设置为private仓库，创建。
3. https://github.com/settings/tokens 生成token，将对应内容填写到config.yaml文件中。
![[image-20241008104106563.png]]

运行程序进入console，implants查看全部上线主机
```shell
client.exe --config ".\redeemer-c2\config.yaml" github
```
![[Pasted image 20241211174731.png]]
使用use 9Mt8Pfsv进入编号为9Mt8Pfsv的implant中，可以执行其他命令。
![[Pasted image 20241211180118.png]]

执行任意命令
```shell
shell whoami
```

## Release Log

### V0.1
- [x] 基础通信架构构建
- [x] 基本命令执行
- [x] client美化

### V0.2
- [x] 随机数逻辑优化，算法生成伪随机数，不依赖外部库
- [ ] 交叉编译，windows和linux执行测试
- [x] COFF Loader，支持CS BOF
- [x] 修复部分情况执行后无法上线的问题（label查询需要小于100）
- [x] 其他bug修复（时间错误）
![[Pasted image 20241211174847.png]]

### V0.3
- [ ] linux rootkit
- [ ] aliyun oss support
- [ ] 存活验证
