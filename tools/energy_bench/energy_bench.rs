// Energy consumption benchmark for hash functions (SHA-256, SHA-512, SHA3-256, BLAKE3).
//
// Reads RAPL (Running Average Power Limit) energy counters from sysfs to measure
// CPU energy consumed per byte hashed.  Despite the "intel" naming, the Linux
// intel_rapl driver also supports AMD Zen processors since kernel 5.4.
//
// # Kernel modules / packages required
//
// ## Intel
//
//     Kernel CONFIG_INTEL_RAPL       (built-in on most distro kernels)
//     lsmod | grep intel_rapl         # verify it's loaded
//     modprobe intel_rapl             # load it if missing
//
// ## AMD
//
//     Kernel CONFIG_INTEL_RAPL       (reused by AMD since kernel 5.4; CONFIG_AMD_RAPL alias)
//     - or -
//     Kernel CONFIG_AMD_ENERGY       (separate driver on some older kernels)
//     lsmod | grep -E 'intel_rapl|amd_energy'
//     modprobe intel_rapl             # most common
//
// ## AMD – MSR prerequisite
//
// The powercap interface may also require the `msr` module on AMD:
//
//     modprobe msr                    # needed on some AMD configs
//
// To verify RAPL is available after loading modules:
//
//     ls /sys/class/powercap/intel-rapl:*/
//
// ## CPU frequency governor (for stable measurements)
//
//     cpupower frequency-set -g performance   # intel
//     - or -
//     cpufreq-set -g performance              # AMD (older)
//     - or -
//     echo performance > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
//
//     # cpupower / cpufreq-set packages:
//     apt install linux-cpupower     # Debian/Ubuntu
//     dnf install kernel-tools       # Fedora
//     pacman -S cpupower             # Arch
//
// # Running
//
//     sudo make run                  # sets governor, pins to core 0, restores after
//     cargo run -p energy_bench --release

use std::{
    fs,
    hint::black_box,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};

use crypto::{
    Hasher,
    blake3::Blake3,
    sha2::{Sha256, Sha512},
    sha3::Sha3_256,
};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB
const BENCH_SECS: u64 = 10;
const COOLDOWN_SECS: u64 = 30;
const IDLE_SECS: u64 = 5;

struct RaplReader {
    paths: Vec<PathBuf>,
    names: Vec<String>,
}

impl RaplReader {
    fn detect() -> Result<Self, String> {
        let powercap = Path::new("/sys/class/powercap");
        if !powercap.exists() {
            return Err("/sys/class/powercap not found. RAPL is not available on this system.".into());
        }

        let mut paths = Vec::new();
        let mut names = Vec::new();
        let entries = fs::read_dir(powercap).map_err(|e| format!("cannot read /sys/class/powercap: {e}"))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("error reading powercap entry: {e}"))?;
            let fname = entry.file_name().to_string_lossy().to_string();

            if fname.starts_with("intel-rapl:") && fname.matches(':').count() == 1 {
                let energy_path = entry.path().join("energy_uj");
                if energy_path.exists() {
                    let name = fs::read_to_string(entry.path().join("name"))
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    names.push(name);
                    paths.push(energy_path);
                }
            }
        }

        if paths.is_empty() {
            return Err("no intel-rapl energy counters found in /sys/class/powercap.\n\
                 RAPL is available on Intel CPUs (Sandy Bridge+) and AMD Zen CPUs (kernel 5.4+)."
                .into());
        }

        Ok(Self {
            paths,
            names,
        })
    }

    fn read_energy_uj(&self) -> Result<u64, String> {
        let mut total: u64 = 0;
        for path in &self.paths {
            let s = fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let val: u64 = s
                .trim()
                .parse()
                .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
            total += val;
        }
        Ok(total)
    }
}

struct Measurement {
    algo: &'static str,
    total_bytes: u64,
    energy_j: f64,
    wall_secs: f64,
}

fn measure_hash<H: Hasher>(
    rapl: &RaplReader,
    data: &[u8],
    duration: Duration,
    algo: &'static str,
) -> Result<Measurement, String> {
    let chunk_bytes = data.len() as u64;
    let mut iterations: u64 = 0;

    let energy_before = rapl.read_energy_uj()?;
    let wall_start = Instant::now();

    while wall_start.elapsed() < duration {
        let h = H::hash(black_box(data));
        black_box(h);
        iterations += 1;
    }

    let wall_elapsed = wall_start.elapsed();
    let energy_after = rapl.read_energy_uj()?;
    let total_bytes = chunk_bytes * iterations;

    let energy_uj = if energy_after >= energy_before {
        energy_after - energy_before
    } else {
        u64::MAX - energy_before + energy_after + 1
    };

    Ok(Measurement {
        algo,
        total_bytes,
        energy_j: energy_uj as f64 / 1_000_000.0,
        wall_secs: wall_elapsed.as_secs_f64(),
    })
}

fn measure_idle(rapl: &RaplReader, secs: u64) -> Result<(f64, f64), String> {
    let before = rapl.read_energy_uj()?;
    let start = Instant::now();
    while start.elapsed().as_secs() < secs {
        std::hint::spin_loop();
    }
    let after = rapl.read_energy_uj()?;
    let elapsed = start.elapsed().as_secs_f64();

    let energy_uj = if after >= before {
        after - before
    } else {
        u64::MAX - before + after + 1
    };

    Ok((energy_uj as f64 / 1_000_000.0, elapsed))
}

fn commafy(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn main() {
    let rapl = match RaplReader::detect() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let governor = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .unwrap_or_default()
        .trim()
        .to_string();

    if governor != "performance" {
        eprintln!(
            "warning: CPU governor is '{governor}', not 'performance'.\n\
             run 'make run' or set governor manually for stable measurements.\n"
        );
    }

    for (n, p) in rapl.names.iter().zip(rapl.paths.iter()) {
        eprintln!("found RAPL domain: {} ({})", n, p.display());
    }

    let data = vec![0xA5u8; CHUNK_SIZE];
    let bench_duration = Duration::from_secs(BENCH_SECS);

    eprintln!("\nmeasuring idle baseline ({IDLE_SECS}s)...");
    let (idle_j, idle_secs) = measure_idle(&rapl, IDLE_SECS).unwrap_or_else(|e| {
        eprintln!("warning: idle measurement failed: {e}");
        (0.0, IDLE_SECS as f64)
    });
    let idle_w = idle_j / idle_secs;

    eprintln!("\nbenchmarking SHA-256 ({BENCH_SECS}s)...");
    let sha = measure_hash::<Sha256>(&rapl, &data, bench_duration, "SHA-256").unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    eprintln!("cooling down ({COOLDOWN_SECS}s)...");
    thread::sleep(Duration::from_secs(COOLDOWN_SECS));

    eprintln!("\nbenchmarking BLAKE3 ({BENCH_SECS}s)...");
    let blake3 = measure_hash::<Blake3>(&rapl, &data, bench_duration, "BLAKE3").unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    eprintln!("cooling down ({COOLDOWN_SECS}s)...");
    thread::sleep(Duration::from_secs(COOLDOWN_SECS));

    eprintln!("\nbenchmarking SHA-512 ({BENCH_SECS}s)...");
    let sha512 = measure_hash::<Sha512>(&rapl, &data, bench_duration, "SHA-512").unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    eprintln!("cooling down ({COOLDOWN_SECS}s)...");
    thread::sleep(Duration::from_secs(COOLDOWN_SECS));

    eprintln!("\nbenchmarking SHA3-256 ({BENCH_SECS}s)...");
    let sha3 = measure_hash::<Sha3_256>(&rapl, &data, bench_duration, "SHA3-256").unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    println!();
    println!(
        "idle power: {:.1} W  |  governor: {}  |  bench duration: {}s",
        idle_w, governor, BENCH_SECS,
    );
    println!();

    let mut results = vec![&sha, &sha512, &sha3, &blake3];
    results.sort_by(|a, b| {
        let a_ratio = a.total_bytes as f64 / a.energy_j;
        let b_ratio = b.total_bytes as f64 / b.energy_j;
        b_ratio.partial_cmp(&a_ratio).unwrap()
    });

    println!(
        "{:<12} {:>16} {:>12} {:>16} {:>16} {:>12}",
        "Algorithm", "Bytes Hashed", "Energy (J)", "bytes/J", "J/byte", "Thruput"
    );
    println!("{:-<12} {:-<16} {:-<12} {:-<16} {:-<16} {:-<12}", "", "", "", "", "", "");

    for m in &results {
        let b_per_j = m.total_bytes as f64 / m.energy_j;
        let j_per_b = m.energy_j / m.total_bytes as f64;
        let thr_mb_s = m.total_bytes as f64 / m.wall_secs / 1_048_576.0;

        println!(
            "{:<12} {:>16} {:>11.1} J {:>16} {:>15.9e} {:>11.1} MB/s",
            m.algo,
            commafy(m.total_bytes),
            m.energy_j,
            commafy(b_per_j as u64),
            j_per_b,
            thr_mb_s,
        );
    }
}
