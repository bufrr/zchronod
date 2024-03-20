use prost::Message;

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZMessage {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(enumeration = "ZType", tag = "2")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "3")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rng {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(uint64, tag = "2")]
    pub num: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ZType {
    Rng = 0,
    Event = 1,
}

impl Into<Vec<u8>> for ZMessage {
    fn into(self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.encode(&mut buf).unwrap();
        buf
    }
}

impl ZType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ZType::Rng => "Z_TYPE_RNG",
            ZType::Event => "Z_TYPE_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Z_TYPE_RNG" => Some(Self::Rng),
            "Z_TYPE_EVENT" => Some(Self::Event),
            _ => None,
        }
    }
}

/// Generated client implementations.
pub mod message_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;
    use tonic::codegen::http::Uri;

    #[derive(Debug, Clone)]
    pub struct MessageClient<T> {
        inner: tonic::client::Grpc<T>,
    }

    impl MessageClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }

    impl<T> MessageClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data=Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MessageClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                    Response=http::Response<
                        <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::BoxBody>,
                >>::Error: Into<StdError> + Send + Sync,
        {
            MessageClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn send_z_message(
            &mut self,
            request: impl tonic::IntoRequest<super::ZMessage>,
        ) -> std::result::Result<tonic::Response<super::ZMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zmessage.Message/SendZMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zmessage.Message", "SendZMessage"));
            self.inner.unary(req, path, codec).await
        }
    }
}

/// Generated server implementations.
pub mod message_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;

    /// Generated trait containing gRPC methods that should be implemented for use with MessageServer.
    #[async_trait]
    pub trait Message: Send + Sync + 'static {
        async fn send_z_message(
            &self,
            request: tonic::Request<super::ZMessage>,
        ) -> std::result::Result<tonic::Response<super::ZMessage>, tonic::Status>;
    }

    #[derive(Debug)]
    pub struct MessageServer<T: Message> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }

    struct _Inner<T>(Arc<T>);

    impl<T: Message> MessageServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessageServer<T>
        where
            T: Message,
            B: Body + Send + 'static,
            B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/zmessage.Message/SendZMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendZMessageSvc<T: Message>(pub Arc<T>);
                    impl<T: Message> tonic::server::UnaryService<super::ZMessage>
                    for SendZMessageSvc<T> {
                        type Response = super::ZMessage;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ZMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Message>::send_z_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendZMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }

    impl<T: Message> Clone for MessageServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }

    impl<T: Message> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Message> tonic::server::NamedService for MessageServer<T> {
        const NAME: &'static str = "zmessage.Message";
    }
}
