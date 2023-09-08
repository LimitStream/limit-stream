# Limit Stream 路线图

Limit Stream 是一个通过定义语言生成**流式**远程调用方法、接口及自动序列化/反序列化代码的框架

定义有4个层次

1. IDL Level: 流方法/状态转移描述层
2. 从 IDL 生成的**编程语言接口层**，通过语言本身提供的约束一定程度保证业务代码正确性
3. 从 IDL 生成的序列化/反序列化器，可选增量
4. 一个基础协议转接层，作为序列化反序列化器的下层协议，默认为 udp/quic

```txt
Interface Define Language Level
|           |
| Programming Language Interface Level
|           |                   |
Serialize/Deserialize Level     |
                |               |
              Protocol Adapter Level
```

## 0.1.0

- [x] 核心语法
- [x] Parser
- [ ] `import` files
- [ ] core type checker
- [ ] core code generator
  - [x] formatter
  - [ ] 可选的 C 序列化反序列化器 代码生成器
  - [ ] Rust 编程语言接口
  - [ ] TypeScript 编程语言接口
  - [ ] Golang 编程语言接口
- [ ] server and client for languages
  - [ ] Rust
    - [ ] TCP
    - [ ] WebSocket 多路复用器
    - [ ] Webtransport
  - [ ] Golang
    - [ ] TCP
    - [ ] WebSocket 多路复用器
    - [ ] Webtransport
  - [ ] TypeScript(browser) (client)
    - [ ] WebSocket 多路复用器
    - [ ] Webtransport
  - [ ] Dart flutter
    - [ ] WebSocket 多路复用器

## 0.2.0

- [ ] 支持 Range 类型，例子：`int from 0 to 255`
- [ ] 支持 Regex 字符串类型匹配，例子：`string matches "ID[0-9]+"`
- [ ] operator `recvs(N) <TYPE> stop by <SESSION TYPE>` : less strict but industrial for induction(stream has an end) type with buffer size `N`.
  - [ ] and its dual which is `send(N) <TYPE> stop by <SESSION TYPE>`

## 1.0.0

- [ ] 包管理器
- [ ] Debug 工具
- [ ] Mock 工具集
- [ ] Fuzzer
