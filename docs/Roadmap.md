# Limit Stream Roadmap

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

- [x] core grammar
- [x] core parser
- [ ] `import` files
- [ ] core type checker
- [ ] core code generator
  - [x] formatter
  - [ ] Option C Serialize/Deserialize CodeGen
  - [ ] Rust Language Interface
  - [ ] TypeScript Language Interface
  - [ ] Golang Language Interface
- [ ] server and client for languages
  - [ ] Rust
    - [ ] TCP
    - [ ] WebSocket Multiplexing
    - [ ] Webtransport
  - [ ] Golang
    - [ ] TCP
    - [ ] WebSocket Multiplexing
    - [ ] Webtransport
  - [ ] TypeScript(browser) (client)
    - [ ] WebSocket Multiplexing
    - [ ] Webtransport
  - [ ] Dart flutter
    - [ ] WebSocket Multiplexing

## 0.2.0

- [ ] type support range type for example `int from 0 to 255`
- [ ] type support string regex for example `string matches "ID[0-9]+"`
- [ ] operator `recvs(N) <TYPE> stop by <SESSION TYPE>` : less strict but industrial for induction(stream has an end) type with buffer size `N`.
  - [ ] and its dual which is `send(N) <TYPE> stop by <SESSION TYPE>`

## 1.0.0

- [ ] package manager
- [ ] debug tools
- [ ] mock tool set
- [ ] Fuzzer
