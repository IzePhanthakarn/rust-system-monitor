use sysinfo::{Disks, System};

/// ข้อมูล disk ต่อ 1 mount point
#[derive(Clone, Debug)]
pub struct DiskStat {
    pub mount: String,
    pub used_gb: f64,
    pub total_gb: f64,
    pub pct: f64,
}

/// ข้อมูล process ที่จะแสดง
#[derive(Clone, Debug)]
pub struct ProcStat {
    pub pid: String,
    pub name: String,
    pub mem_mb: f64,
    pub cpu_pct: f32,
}

/// ข้อมูลภาพรวมเครื่องทั้งหมด
#[derive(Clone, Debug)]
pub struct MachineStats {
    pub cpu_pct: f32,
    pub cores: usize,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,

    pub ram_used_gb: f64,
    pub ram_total_gb: f64,
    pub ram_pct: f64,
    pub swap_used_gb: f64,
    pub swap_total_gb: f64,

    pub disks: Vec<DiskStat>,
    pub top_procs: Vec<ProcStat>,
}

/// ตัวอ่านข้อมูลจาก OS
pub struct SysReader {
    sys: System,
    disks: Disks,
}

impl SysReader {
    pub fn new() -> Self {
        // new_all() ทำให้มี capability ครบ CPU/MEM/PROCESS
        let mut sys = System::new_all();
        sys.refresh_all();

        // ใน sysinfo 0.30.x disk ถูกแยกเป็น Disks
        let mut disks = Disks::new_with_refreshed_list();
        disks.refresh();

        Self { sys, disks }
    }

    /// อ่านข้อมูลทุกอย่างที่แอปต้องใช้
    pub fn read_all(&mut self, top_n: usize) -> MachineStats {
        // ---------------------------
        // refresh เฉพาะที่ใช้จริง:
        // - เป็นแนวคิดสำคัญของงาน monitoring
        // - ไม่ refresh_all() ทุกครั้งเพื่อประหยัดเวลา/ทรัพยากร
        // ---------------------------
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();
        self.disks.refresh();

        // CPU
        let cpu_pct = self.sys.global_cpu_info().cpu_usage();
        let cores = self.sys.cpus().len();

        // load average (บาง OS จะไม่มีค่า meaningful แต่เรียกได้)
        let load = System::load_average();

        // RAM/SWAP (หน่วย KiB)
        let ram_total = self.sys.total_memory() as f64;
        let ram_used = self.sys.used_memory() as f64;

        let ram_total_gb = kib_to_gb(ram_total);
        let ram_used_gb = kib_to_gb(ram_used);
        let ram_pct = if ram_total > 0.0 { (ram_used / ram_total) * 100.0 } else { 0.0 };

        let swap_total_gb = kib_to_gb(self.sys.total_swap() as f64);
        let swap_used_gb = kib_to_gb(self.sys.used_swap() as f64);

        // DISKS
        let mut disk_stats = Vec::new();
        for d in self.disks.iter() {
            let total = d.total_space() as f64;
            let avail = d.available_space() as f64;
            let used = total - avail;

            let total_gb = bytes_to_gb(total);
            let used_gb = bytes_to_gb(used);
            let pct = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };

            disk_stats.push(DiskStat {
                mount: d.mount_point().to_string_lossy().to_string(),
                used_gb,
                total_gb,
                pct,
            });
        }

        // TOP process (sort by memory desc)
        let mut procs: Vec<_> = self.sys.processes().values().collect();
        procs.sort_by_key(|p| std::cmp::Reverse(p.memory()));

        let top_procs = procs
            .into_iter()
            .take(top_n)
            .map(|p| ProcStat {
                pid: p.pid().to_string(),
                name: p.name().to_string(),
                mem_mb: (p.memory() as f64) / 1024.0, // KiB -> MiB
                cpu_pct: p.cpu_usage(),
            })
            .collect();

        MachineStats {
            cpu_pct,
            cores,
            load_1: load.one,
            load_5: load.five,
            load_15: load.fifteen,

            ram_used_gb,
            ram_total_gb,
            ram_pct,
            swap_used_gb,
            swap_total_gb,

            disks: disk_stats,
            top_procs,
        }
    }
}

// helper: KiB -> GB
fn kib_to_gb(kib: f64) -> f64 {
    kib / 1024.0 / 1024.0
}

// helper: bytes -> GB
fn bytes_to_gb(bytes: f64) -> f64 {
    bytes / 1024.0 / 1024.0 / 1024.0
}