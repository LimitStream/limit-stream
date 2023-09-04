# Limit-Stream
> 当前文档 Protocol 版本为 0.1.0

此文当符合 [RFC2119](https://datatracker.ietf.org/doc/html/rfc2119)

## Terminology
- Frame: 包含请求、响应或协议处理的单个消息
- Fragment: 消息的一部分已经包含与Frame中，详细清查看 `重组`
- Payload: 流消息， 包含先前请求的关联数据
- eps/end: 流完成传输
- dual: 二元，一个消息的服务协议**必须**能够生成一个消息的客户协议，详细请谷歌 `session type`

## 版本控制
请查看[语义化版本](https://semver.org/)

## Framing

### Transport Protocol
底层的传输协议**必须**包括以下内容：
1. 可靠传播
2. 保证 Frame 的顺序
3. (假设)有FCS(帧校验序列)

### Framing Protocol Usage
有一些传输协议**不能**保证消息的边界，所以有一些协议需要设定帧长度

| protocol       | need frame length |
| -------------- | ----------------- |
| TCP            | Yes               |
| WebSocket      | No                |
| Http2.0 Stream | Yes               |
| WebTransport   | No                |

### Framing Format
当协议可以提供帧传输的时候直接使用协议的帧传输协议，

当协议**不能**保证消息边界或者不兼容帧传输的时候需要添加长度
```
 0                   1                   2
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Frame Length               |
+-----------------------------------------------+
|                LimitStream Frame              |
+-----------------------------------------------+
```

> 帧长度 ：（24 位 = 最大值 16,777,215）无符号 24 位整数，表示帧长度（以字节为单位）。 不包括帧长度字段。 

注意 ：字节顺序是大尾数。 

### Header Format
帧总是以帧头为开始，布局如下
```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|0|                         Stream ID                           |
+-----------+-+-+---------------+-------------------------------+
|Frame Type |I|M|     Flags     |     Depends on Frame Type  ....
+-------------------------------+-------------------------------+
```

- Stream ID: （31 位 = 最大值 2^31-1 = 2,147,483,647）无符号 31 位整数，表示该帧的流标识符，或 0 表示整个连接。
    - 如果各方都同意，包括解复用的传输协议（例如 HTTP/2）可以省略流 ID 字段。 协商和协议的方式由传输协议决定。 
- Frame Type:（6 位 = 最大值 63）种帧类型。 
- Flags:（10 位）帧类型中未明确指示的任何标志位在发送时应设置为 0，且不解释 接待。 标志通常取决于帧类型，但所有帧类型都**必须**为以下标志提供空间： ( I )gnore：如果不理解则忽略框架 ( M )etadata：存在元数据 

#### Handling Ignore Flag
当一个帧被服务器接收，服务器不理解使用的协议或没有办法完成，且(I)gnore标志并不是1的时候，服务器会抛出 `ERROR[CONNECTION_ERROR]` 然后关闭传输管道。

#### Metadata Optional Header
特定帧类型可以包含元数据。 如果该帧类型同时支持数据和元数据，则**必须**包含可选的元数据标头。 该元数据标头位于帧标头和任何有效负载之间。 

元数据长度**必须**等于帧长度减去帧头长度和帧有效负载长度（如果存在）的总和。 如果元数据长度不等于该值，则该帧无效，并且接收方**必须**发送一个 `ERROR [CONNECTION_ERROR]` 帧并在接收时关闭底层传输连接，除非设置了该帧的 `I` 标志。

在具有数据和元数据的框架上： 

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|              Metadata Length                  |
+---------------------------------------------------------------+
|                       Metadata Payload                       ...
+---------------------------------------------------------------+
|                       Payload of Frame                       ...
+---------------------------------------------------------------+
```

当数据长度为0时，**不能**发送 `Payload of Frame`

在只有元数据的帧上**不能**发送`Metadata Length`

## Stream Identifiers
流的ID由请求者生成。
ID为`0`值的任何操作都是涉及到连接保留和内部使用的。
流的ID**必须**是本地唯一的。
流ID对偶的双方需要采取的规则是客户端为`N`，服务器端的响应**必须**是`N+1`, 然后双方自增`2`, 服务器上的流以2开始，客户端以1开始

打个比方

```
channel sum =
    recv int ->
    offer 
        | sum
        | recv Done ->
            send int ->
            end
```
ID 在 `[里面]`
```
client [1]: send 1
server [2]: recv 1
client [3]: send 2
server [4]: recv 2
client [5]: choose2
server [6]: offer2
client [7]: send Done
server [8]: recv Done
server [10]: send 3
client [11]: recv 3
```

### 寿命
一旦使用了最大流 ID (2^31-1)，请求者可以重复使用流 `ID`。 响应者**必须**假设流 `ID` 将被重用。

当最大`Stream ID`已被使用时：
- 如果不使用 Stream ID 重用： **应该**
    - 无法创建新的流，因此一旦达到最大值，**必须**建立新的连接来创建新的流。 
- 如果重新使用流 ID： **可以**
    - 请求者**必须**通过包装并重新启动客户端的 ID 1 和服务器的 ID 2 来重新使用 ID，并按如上所述按 2 秒顺序递增。
    - 请求者**必须**跳过仍在使用的 ID。
    - 可以选择 `ERROR [CONNECTION_ERROR]` 如果响应方认为 ID 仍在使用中，则 任何流 ID。 请求者可以重试下一个被认为未使用的连续流 ID。
    - 如果所有Stream ID同时使用，则无法创建新的流，因此**必须**建立新的连接来创建新的流。 

流 ID 重用**应该**仅与可恢复性结合使用。 