#![allow(clippy::module_name_repetitions)]
use crate::config::Severity;

pub fn alarm(host: &str, reason: &str, severity: &Severity, message: &str) {
    log::error!(target:"Logging alarm","Host: {host}, reason: {reason}, severity: {severity}, {message}" );
}
