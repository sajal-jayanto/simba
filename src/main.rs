use crossterm::terminal;
use regex::Regex;
use std::{
  collections::HashMap,
  io::{self, BufRead, BufReader , Write},
  process::{Command, Stdio},
};

#[derive(Debug, Clone)]
struct CurrentStats {
  id: String,
  name: String,
  cpu: String,
  ram: String,
  pid: String,
}

impl CurrentStats {
  fn new(fields: &[&str]) -> Self {
    Self {
      id: fields[0].to_string(),
      name: fields[1].to_string(),
      cpu: fields[2].to_string(),
      ram: fields[3].to_string(),
      pid: fields[6].to_string(),
    }
  }
}

fn main() {
  let mut child = Command::new("docker")
    .args([
      "stats",
      "--format",
      "{{.ID}}|{{.Name}}|{{.CPUPerc}}|{{.MemUsage}}|{{.NetIO}}|{{.BlockIO}}|{{.PIDs}}",
    ])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Failed to start docker stats");

  let stdout = child.stdout.take().expect("Failed to open stdout");
  let reader = BufReader::new(stdout);
  let terminal_size = terminal::size();

  let mut running_hash_map: HashMap<String, CurrentStats> = HashMap::new();

  for line in reader.lines() {
    let line = line.unwrap();
    let clean = strip_ansi(&line).trim().to_string();

    if !clean.is_empty() {
      let split_stats: Vec<&str> = clean.split('|').collect();
      let stats = CurrentStats::new(&split_stats);
      running_hash_map.insert(stats.id.clone(), stats);

      let stoped: Vec<String> = running_hash_map
        .keys()
        .filter(|id| !is_container_running(id))
        .cloned()
        .collect();

      for id in stoped {
        running_hash_map.remove(&id);
      }

      if running_hash_map.is_empty() {
        println!("No running docker container found.");
        return;
      }

      if let Ok((win_width, _height)) = terminal_size {
        let with = 120.min(win_width / 2) as i32;
        for (_id, stats) in running_hash_map.iter() {
          let box_with = print_title(with , &stats.name);
          blank_line(box_with);
          draw_progress_bar( "CPU", with , stats
            .cpu
            .trim_end_matches('%')
            .parse::<f32>()
            .unwrap()
          );
          print!(" ║\n");
          let (used, total) = stats.ram.split_once('/').unwrap();
          let used_mib = to_mib(used).unwrap();
          let total_mib = to_mib(total).unwrap();
          let ram = memory_percentage(used_mib, total_mib);
          draw_progress_bar( "Ram", with - 20 , ram);
          println!(" {} ║" , stats.ram);
          print!("║  Pids: {}\n", stats.pid);
          blank_line(box_with);
          end_line(box_with);
          
          println!("");
        }
      }

      print!("\x1B[2J\x1B[1;1H");

      
    } else {
      if running_hash_map.is_empty() {
        eprintln!("No running docker container found.");
        return;
      }
    }
  }
}

fn is_container_running(id: &str) -> bool {
  let output = Command::new("docker")
    .args(["inspect", "-f", "{{.State.Running}}", id])
    .output()
    .expect("Failed to run docker inspect");

  if !output.status.success() {
    return false;
  }

  let result = String::from_utf8_lossy(&output.stdout);
  return result.trim() == "true";
}

fn strip_ansi(s: &str) -> String {
  let reg = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
  let reg_str = reg.replace_all(s, "").to_string();
  return reg_str;
}

fn draw_progress_bar(title: &str , width: i32, value: f32) {
  let total: f32 = 100.00;
  let value = value.clamp(0.00, total);
  let label = format!(" {}%", round_2(value));

  let bar_width = (width - label.len() as i32 - 3).max(0) as usize;
  let filled = bar_width * value as usize / total as usize;

  print!(
    "\r║  {title}:[{}{}]{}",
    "#".repeat(filled),
    "-".repeat(bar_width - filled),
    label
  );

  io::stdout().flush().unwrap();
}

fn print_title(with: i32, name: &str) -> usize {
  let mut title = format!("╔═══ {} " , name);
  let mut cnt = (with + 15) - title.len() as i32; 
  while cnt > 0 {
    title.push('═');
    cnt -= 1;
  }
  title.push('╗');
  println!("{}" , title);
  return title.chars().count();
}

fn end_line(width: usize) {
  let mut len = width;
  print!("╚");
  while len > 2 {
    print!("═");
    len -= 1;
  }
  print!("╝");
}

fn blank_line(width: usize){
  print!("║");
  let mut len = width - 2;
  while len > 0 {
    print!(" ");
    len -= 1;
  }
  print!("║\n");
}

fn to_mib(input: &str) -> Option<f32> {
  let input = input.trim();

  if let Some(value) = input.strip_suffix("MiB") {
    value.trim().parse::<f32>().ok()
  } else if let Some(value) = input.strip_suffix("GiB") {
    value.trim().parse::<f32>().ok().map(|n| n * 1024.0)
  } else {
    None
  }
}

fn memory_percentage(used: f32, total: f32) -> f32 {
  return (used / total) * 100.0;
}

fn round_2(value: f32) -> f32 {
  return (value * 100.0).round() / 100.0;
}
