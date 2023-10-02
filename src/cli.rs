//! This file contains structs and functionality that are relevant to the Command Line Interface part of the program.
use std::ffi::OsString;
use clap::Parser;

/// Custom enum to define the desired loglevel during run-time.
#[derive(clap::ValueEnum, Clone)]
pub enum LogLevel {
  TRACE,
  DEBUG,
  INFO,
  WARN,
  ERROR,
}

/// Implement Into<OsString> for LogLevel, so it can actually be used by env_logger.
impl Into<OsString> for LogLevel {
  fn into(self) -> OsString {
    match self {
      LogLevel::TRACE => "trace".into(),
      LogLevel::DEBUG => "debug".into(),
      LogLevel::INFO => "info".into(),
      LogLevel::WARN => "warn".into(),
      LogLevel::ERROR => "error".into(),
    }
  }
}

/// TODO: comments in this file
#[derive(Parser)]
pub struct Cli { 
  /// The name of the config file to use. 
  #[arg(short = 'c', long = "config", default_value = "default")]
  pub config_file: String,
  /// Set log level.
  /// 
  /// Sets the log level to print to stdout.
  #[arg(long = "log", value_enum, default_value = "error")]
  pub log_level: LogLevel
}