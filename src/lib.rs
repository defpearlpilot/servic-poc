pub mod dots;

pub mod proto {
    tonic::include_proto!("dots"); // The string specified here must match the proto package name
}
