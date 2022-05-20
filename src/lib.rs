// Declare the modules created from protbufs.
pub mod trust {
    tonic::include_proto!("trust");
}

pub mod action;
pub mod animation;
pub mod components;
pub mod core;
pub mod input;
pub mod resources;
pub mod systems;
