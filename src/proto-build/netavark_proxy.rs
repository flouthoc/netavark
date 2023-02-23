/// Netavark sends the proxy the Network Configuration that it wants to setup
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    #[prost(string, tag = "1")]
    pub host_iface: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub container_iface: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub container_mac_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub domain_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    #[prost(enumeration = "Version", tag = "6")]
    pub version: i32,
    #[prost(string, tag = "7")]
    pub ns_path: ::prost::alloc::string::String,
}
/// Lease can either contain a IPv4 or IPv6 DHCP lease, and the common IP information
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    #[prost(uint32, tag = "1")]
    pub t1: u32,
    #[prost(uint32, tag = "2")]
    pub t2: u32,
    #[prost(uint32, tag = "3")]
    pub lease_time: u32,
    #[prost(uint32, tag = "4")]
    pub mtu: u32,
    #[prost(string, tag = "5")]
    pub domain_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub mac_address: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub is_v6: bool,
    #[prost(string, tag = "11")]
    pub siaddr: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub yiaddr: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub srv_id: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub subnet_mask: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub broadcast_addr: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "19")]
    pub dns_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "20")]
    pub gateways: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "21")]
    pub ntp_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "22")]
    pub host_name: ::prost::alloc::string::String,
}
/// Empty Message to send when calling for a shutdown
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// Response to netavark on successful teardown
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NvIpv4Addr {
    #[prost(bytes = "vec", tag = "1")]
    pub octets: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NvIpv6Addr {
    #[prost(bytes = "vec", tag = "1")]
    pub octets: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Version {
    V4 = 0,
    V6 = 1,
}
impl Version {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Version::V4 => "V4",
            Version::V6 => "V6",
        }
    }
}
/// Generated client implementations.
pub mod netavark_proxy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct NetavarkProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetavarkProxyClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NetavarkProxyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
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
        ) -> NetavarkProxyClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NetavarkProxyClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn setup(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkConfig>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status> {
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
                "/netavark_proxy.NetavarkProxy/Setup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn teardown(
            &mut self,
            request: impl tonic::IntoRequest<super::NetworkConfig>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status> {
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
                "/netavark_proxy.NetavarkProxy/Teardown",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clean(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::OperationResponse>, tonic::Status> {
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
                "/netavark_proxy.NetavarkProxy/Clean",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod netavark_proxy_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NetavarkProxyServer.
    #[async_trait]
    pub trait NetavarkProxy: Send + Sync + 'static {
        async fn setup(
            &self,
            request: tonic::Request<super::NetworkConfig>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status>;
        async fn teardown(
            &self,
            request: tonic::Request<super::NetworkConfig>,
        ) -> Result<tonic::Response<super::Lease>, tonic::Status>;
        async fn clean(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::OperationResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct NetavarkProxyServer<T: NetavarkProxy> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: NetavarkProxy> NetavarkProxyServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NetavarkProxyServer<T>
    where
        T: NetavarkProxy,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/netavark_proxy.NetavarkProxy/Setup" => {
                    #[allow(non_camel_case_types)]
                    struct SetupSvc<T: NetavarkProxy>(pub Arc<T>);
                    impl<
                        T: NetavarkProxy,
                    > tonic::server::UnaryService<super::NetworkConfig> for SetupSvc<T> {
                        type Response = super::Lease;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkConfig>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).setup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/netavark_proxy.NetavarkProxy/Teardown" => {
                    #[allow(non_camel_case_types)]
                    struct TeardownSvc<T: NetavarkProxy>(pub Arc<T>);
                    impl<
                        T: NetavarkProxy,
                    > tonic::server::UnaryService<super::NetworkConfig>
                    for TeardownSvc<T> {
                        type Response = super::Lease;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NetworkConfig>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).teardown(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TeardownSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/netavark_proxy.NetavarkProxy/Clean" => {
                    #[allow(non_camel_case_types)]
                    struct CleanSvc<T: NetavarkProxy>(pub Arc<T>);
                    impl<T: NetavarkProxy> tonic::server::UnaryService<super::Empty>
                    for CleanSvc<T> {
                        type Response = super::OperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clean(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CleanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: NetavarkProxy> Clone for NetavarkProxyServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: NetavarkProxy> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NetavarkProxy> tonic::server::NamedService for NetavarkProxyServer<T> {
        const NAME: &'static str = "netavark_proxy.NetavarkProxy";
    }
}
