
//	CPU
//	/sys/devices/system/cpu/cpu0/cpufreq/

//	scaling_available_frequencies
//	scaling_available_governors
//	scaling_min_freq
//	scaling_max_freq
//	scaling_cur_freq
//	scaling_setspeed
//	scaling_governor
//	scaling_driver
//	/sys/devices/system/cpu/cpufreq/boost

// GPU
//	/sys/class/drm/card1/device/

//	pp_dpm_sclk
//	pp_dpm_mclk
//	
//	vbios_version
//	vendor
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn file_to_string(file_name: &str) -> std::io::Result<String> {
	let mut file = File::open(file_name)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents[..contents.len() - 1].to_string())
}


fn gpu_info(s: &str) -> String {
	let path: String = "/sys/class/drm/card0/device/".to_string() + s;
	return file_to_string(&path).unwrap();
}

fn cpu_info(s: &str) -> String {
	let path: String = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_".to_string() + s;
	return file_to_string(&path).unwrap();
}

fn set_cpu_boost(boost: bool) -> std::io::Result<()> {
	let mut f = File::create("/sys/devices/system/cpu/cpufreq/boost")?;
	if boost == true {
 		f.write_all(b"1")?;
	} else {
		f.write_all(b"0")?;
	}
	return Ok(())
}

fn set_cpu(s: &str, v: &str) -> std::io::Result<()> {
	let path: String = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_".to_string() + s;
	let mut f = File::create(path).unwrap();
	f.write_all(v.as_bytes()).unwrap();
	return Ok(())
}

//fn divided_M() -> String {
fn KHz_to_GHz(s: String) -> String {
	let value = s.parse::<usize>().unwrap();
	let new_value: f64 = (value/10000) as f64 / 100.0;
	return new_value.to_string()
}

fn B_to_M(s: String) -> String {
	let value = s.parse::<usize>().unwrap();
	let new_value = value/(1000*1000);// as f64 / 100.0;
	return new_value.to_string()
}


fn all_cpu_infos() {
	println!("CPU:");
	println!("  driver: {}", cpu_info("driver"));
	println!("  available governors: {}", cpu_info("available_governors"));
	println!("  available frequency range: {} GHz - {} GHz", KHz_to_GHz(cpu_info("min_freq")), KHz_to_GHz(cpu_info("max_freq")));
	println!("  current govenor: {}", cpu_info("governor"));
	println!("  current frequency: {} GHz", KHz_to_GHz(cpu_info("cur_freq")));
	println!("  boost state: {}", file_to_string("/sys/devices/system/cpu/cpufreq/boost").expect("error"));
}

fn amd_gpu_infos() {
	println!("GPU:");
	println!("  vram total: {} MB", B_to_M(gpu_info("mem_info_vram_total")));
	println!("  vram used: {} MB", B_to_M(gpu_info("mem_info_vram_used")));
	println!("  gpu usage: {} %", gpu_info("gpu_busy_percent"));

	println!("  current sclk:");
	for line in gpu_info("pp_dpm_sclk").lines() {
		println!("    {}", line);
	}
	println!("  current mclk:");
	for line in gpu_info("pp_dpm_mclk").lines() {
		println!("    {}", line);
	}
	let mut counter = 0;
	for line in gpu_info("pp_od_clk_voltage").lines() {
		if counter > 0 {
			println!("    {}", line);
			counter -= 1;
		}
		if line.contains("OD_VDDC_CURVE:") {
			println!("  OD_VDDC_CURVE:");
			counter = 3;
		}
	}
}

fn print_usage(prog_name: &str) {
	println!("usage:");
	println!("\t{} {}", prog_name, "--cpu-info");
	println!("\t{} {}", prog_name, "--gpu-info");
}

fn doit() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		print_usage(&args[0]);
	}
	let arg = &args[1].as_str();
	match arg {
		&"--cpu-info" => all_cpu_infos(),
		&"--gpu-info" => amd_gpu_infos(),
		_ => print_usage(&args[0]),
	};
}





fn main() {
	doit();
	//set_cpu("min_freq", "1300000").unwrap();
	//set_cpu_boost(false);

	//all_cpu_infos();
	//amd_gpu_infos();

}








