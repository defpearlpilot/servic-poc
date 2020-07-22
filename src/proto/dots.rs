#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DotCreateRequest {
    #[prost(int32, tag = "1")]
    pub worker: i32,
    #[prost(string, tag = "2")]
    pub author: std::string::String,
    #[prost(string, tag = "3")]
    pub subject: std::string::String,
    #[prost(int32, tag = "4")]
    pub rating: i32,
    #[prost(int32, tag = "5")]
    pub importance: i32,
    #[prost(string, tag = "6")]
    pub attribute: std::string::String,
    #[prost(string, tag = "7")]
    pub comment: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DotCreateResponse {
    #[prost(string, tag = "1")]
    pub dot_id: std::string::String,
    #[prost(string, tag = "2")]
    pub author: std::string::String,
    #[prost(string, tag = "3")]
    pub subject: std::string::String,
    #[prost(int32, tag = "4")]
    pub rating: i32,
    #[prost(int32, tag = "5")]
    pub importance: i32,
    #[prost(string, tag = "6")]
    pub attribute: std::string::String,
    #[prost(string, tag = "7")]
    pub comment: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod dots_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DotsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DotsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DotsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " function which can be called"]
        pub async fn create_dot(
            &mut self,
            request: impl tonic::IntoRequest<super::DotCreateRequest>,
        ) -> Result<tonic::Response<super::DotCreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/dots.Dots/CreateDot");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DotsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DotsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DotsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dots_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DotsServer."]
    #[async_trait]
    pub trait Dots: Send + Sync + 'static {
        #[doc = " function which can be called"]
        async fn create_dot(
            &self,
            request: tonic::Request<super::DotCreateRequest>,
        ) -> Result<tonic::Response<super::DotCreateResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct DotsServer<T: Dots> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Dots> DotsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DotsServer<T>
    where
        T: Dots,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/dots.Dots/CreateDot" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDotSvc<T: Dots>(pub Arc<T>);
                    impl<T: Dots> tonic::server::UnaryService<super::DotCreateRequest> for CreateDotSvc<T> {
                        type Response = super::DotCreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DotCreateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_dot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Dots> Clone for DotsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Dots> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Dots> tonic::transport::NamedService for DotsServer<T> {
        const NAME: &'static str = "dots.Dots";
    }
}
