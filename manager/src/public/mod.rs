// This module provides public api access for crates that depend on this one.

pub mod collections;

pub fn sanitize_file_name(file_name: &str) -> String {
    file_name.replace(['/', '\\', '?', '*', ':', '\'', '\"', '|', '<', '>', '!'], "_")
}
