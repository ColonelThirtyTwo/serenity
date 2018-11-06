
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate chrono;
extern crate parking_lot;
extern crate serenity;
extern crate threadpool;

#[macro_use] mod macros;
mod rwlock_ext;
pub mod framework;
