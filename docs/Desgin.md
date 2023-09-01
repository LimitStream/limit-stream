# Overall design of Limit-Stream

Limit-Stream is a state-of-the-art stream-oriented cloud-native type-checked RPC/streaming framework. It provides a powerful and efficient way to communicate and stream data between clients and servers in a distributed system. In this document, we will explore the various features and design considerations of Limit-Stream.

## Grammar

```lstr
struct Done {}

service SumService {
    channel sum:
        recv int ->
        offer 
            sum |
            recv Done ->
            end
}
```

The Limit-Stream framework uses a specific IDL grammar to define the structure and behavior of its services. The grammar allows developers to define service interfaces, message passing, and endpoints. The example provided showcases a SumService with a sum endpoint that receives an integer and offers either another sum(recursively receiving) or on receiving Done message to terminate the session.


## Session Type

A notable aspect of Limit-Stream is its emphasis on session types. Session types provide a formal way to describe the communication protocols between client and server, ensuring correctness and robustness at runtime. The session type for each service endpoint describes the expected sequence of messages and their allowed order.

For example, the session type for the SumService endpoint sum describes a client-server interaction where the client sends an integer, and the server can respond either with another sum request or a Done message to end the session. This session type definition ensures that the interactions between clients and servers are adhered to, preventing communication errors and ensuring correct behavior.

## Cancel Safe

Another fundamental design consideration of Limit-Stream is its cancel-safe feature. Cancel-safety means that the framework provides mechanisms to handle the cancellation of ongoing operations. In a distributed system, it is crucial to handle cancellations gracefully and ensure that resources are released appropriately.

Limit-Stream achieves cancel-safety by providing built-in mechanisms for canceling ongoing operations at both client and server sides. This allows for efficient resource utilization and prevents unnecessary processing or transmission of data that is no longer needed.

## Resumable

An additional feature that enhances the resumable streaming capability of Limit-Stream is the use of the #[pure] annotation. With this annotation, the responsibility of managing the state change and resumption of streams is seamlessly handled by the server. As a result, developers no longer need to explicitly write code for recovering the state in case of interruptions or failures.

By marking certain functions or methods with the #[pure] annotation, Limit-Stream ensures that they only rely on their input parameters and have no dependency on external states or resources. This allows the framework to automatically handle the resumption of streams by storing and restoring the necessary state information.

With the #[pure] annotation, developers can focus on defining the functionality of their services without worrying about the complexities of managing the state and resuming streams. This abstraction simplifies the development process and improves code readability, ultimately leading to more efficient and reliable streaming applications.

## Next-Gen Protocol

Limit-Stream leverages the advantages of the QUIC (Quick UDP Internet Connections) protocol and its implementation known as WebTransport. QUIC is a transport protocol designed to improve performance, security, and reliability in communication over the internet. It is built on top of UDP (User Datagram Protocol) and offers several notable benefits over traditional protocols like TCP (Transmission Control Protocol).

One significant advantage of QUIC and WebTransport is their ability to work directly in modern web browsers, eliminating the need for proxy servers. Unlike other web protocols that require proxy servers to facilitate communication between clients and servers, QUIC and WebTransport allow direct communication, enhancing speed and reducing latency.

QUIC's multiplexing capability enables multiple streams to be transmitted over a single connection, improving efficiency and reducing connection setup overhead. It also implements congestion control algorithms that adapt to network conditions, providing better performance in varying network environments.

WebTransport, the specific implementation of QUIC for web-based communication, adds additional features and APIs that further enhance its usability. It allows web applications to establish secure, bidirectional, low-latency, and high-throughput connections between clients and servers. This makes it an ideal choice for Limit-Stream to provide efficient and reliable communication between distributed systems.

By utilizing QUIC and WebTransport as the underlying transport protocol, Limit-Stream ensures compatibility with modern web environments, providing developers with a seamless and optimized streaming framework that can be readily used in web applications.

## Precise Codegen

One of the advantages of using session types in Limit-Stream is the ability to perform precise code generation. Session types allow us to determine the dual of the server or client after type checking, which helps in generating code that accurately reflects the behavior described by the session type. Additionally, the session type provides a clear definition of the expected continuation behavior, allowing for the generation of only the necessary code.

During the code generation process in Limit-Stream, the framework leverages the information provided by the session type to generate code specifically tailored to the desired behavior. The dual of the server or client can be determined, which enables the generation of code that matches the expected interactions between the two endpoints.

By generating code based on the session type, developers can be confident that the resulting code accurately represents the intended functionality and communication patterns. Unnecessary code is avoided, leading to cleaner and more efficient code generation.

This precise code generation approach allows for streamlined development and reduces the chances of errors or inconsistencies in the generated code. It also promotes code readability and maintainability, as the generated code aligns closely with the high-level design described by the session type.