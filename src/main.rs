use sysinfo::CpuExt;
use sysinfo::SystemExt;

const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;

#[derive(Debug)]
#[allow(dead_code)]
struct SystemInfo {
    os: String,
    kernel: String,
    cpu: String,
    core_count: String,
    memory: String,
}

fn main() {
    let mut sys = sysinfo::System::new();
    sys.refresh_cpu();
    sys.refresh_memory();

    let info = SystemInfo {
        os: sys
            .long_os_version()
            .unwrap_or_else(|| String::from("not available")),
        kernel: sys
            .kernel_version()
            .unwrap_or_else(|| String::from("not available")),
        cpu: sys.global_cpu_info().brand().trim().to_string(),
        core_count: sys
            .physical_core_count()
            .map(|x| x.to_string())
            .unwrap_or_else(|| String::from("not available")),
        // Convert from Bytes to GibiBytes since it's probably what people expect most of the time
        memory: format!("{:.1} GiB", sys.total_memory() as f64 * BYTES_TO_GIB),
    };

    println!("{:?}", info);
}
