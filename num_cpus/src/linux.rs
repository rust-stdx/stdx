//! Linux-specific helpers.
//!
//! The logical CPU count is provided by [`std::thread::available_parallelism`], which already
//! accounts for CPU affinity and cgroup quotas. This module only provides the physical core
//! count, for which the standard library has no equivalent.

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// Returns the number of physical cores of the current system.
///
/// Counts the distinct physical processor packages and their cores by parsing
/// `/proc/cpuinfo`. Falls back to the number of logical CPUs when the information cannot be
/// read or the file reports no cores.
pub fn get_num_physical_cpus() -> usize {
    let file = match File::open("/proc/cpuinfo") {
        Ok(val) => val,
        Err(_) => return crate::get_num_cpus(),
    };
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut physid: u32 = 0;
    let mut cores: usize = 0;
    let mut chgcount = 0;
    for line in reader.lines().filter_map(|result| result.ok()) {
        let mut it = line.split(':');
        let (key, value) = match (it.next(), it.next()) {
            (Some(key), Some(value)) => (key.trim(), value.trim()),
            _ => continue,
        };
        if key == "physical id" {
            match value.parse() {
                Ok(val) => physid = val,
                Err(_) => break,
            };
            chgcount += 1;
        }
        if key == "cpu cores" {
            match value.parse() {
                Ok(val) => cores = val,
                Err(_) => break,
            };
            chgcount += 1;
        }
        if chgcount == 2 {
            map.insert(physid, cores);
            chgcount = 0;
        }
    }
    let count = map.into_iter().fold(0, |acc, (_, cores)| acc + cores);

    if count == 0 { crate::get_num_cpus() } else { count }
}
