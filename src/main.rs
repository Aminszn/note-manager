mod app;
mod auth;
mod domains;
mod infrastructure;
mod shared;
pub mod cli;

fn main() {
    app::run();
}