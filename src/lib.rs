// Declare the modules created from protbufs.
pub mod trust {
    tonic::include_proto!("trust");
}

pub mod components;
pub mod core;
