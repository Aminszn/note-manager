// src/domain/user/mod.rs

pub mod builder;
pub mod commands;
pub mod dto;
pub mod entity;
pub mod repo;
pub mod service;

pub use builder::UserBuilder;
pub use dto::UserDto;
pub use entity::User;
pub use repo::UserRepository;
pub use service::UserService;