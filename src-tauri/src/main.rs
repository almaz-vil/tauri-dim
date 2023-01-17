#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate page_size;
use std::{path::PathBuf, fs::File, io::{BufReader, BufRead}};
use std::fmt;
use std::f32;

use tauri::Manager;

#[derive(Clone,  PartialEq, PartialOrd)]
struct InfoMem{
    total: f32,
    free: f32,
}

impl InfoMem {
    pub fn kb_in_mb(&mut self){
        self.total=self.total/1024.00;
        self.free=self.free/1024.00;
    }
}
impl std::fmt::Debug for InfoMem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s=String::new();
        s.push_str("{\"total\":\"");
        s.push_str(&self.total.to_string());
        s.push_str("\", \"free\":\"");
        s.push_str(&self.free.to_string());
        s.push_str("\"}");    
        f.write_str(s.as_str())         
    }
}
#[derive(Clone,  PartialEq, PartialOrd)]
struct InfoProc{
    id: i32,
    name: String,
    state: String,
    vmsize: f32,
    error: String,
}

impl InfoProc {
    pub fn kb_in_mb(&mut self){
        self.vmsize=self.vmsize/1024.00;
    }
}

impl std::fmt::Debug for InfoProc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s=String::new();
        s.push_str("{\"id\":\"");
        s.push_str(&self.id.to_string());
        s.push_str("\", \"name\":\"");
        s.push_str(&self.name);
        s.push_str("\", \"state\":\"");
        s.push_str(&self.state);
        s.push_str("\", \"vmsize\":\"");
        s.push_str(&self.vmsize.to_string());
        s.push_str("\", \"error\":\"");
        s.push_str(&self.error);
        s.push_str("\"}");    
        f.write_str(s.as_str())        
    }
}

#[tauri::command]
fn mem()->String{
    let mut mem= InfoMem{
        free:0.0,
        total:0.0,
    };
    let f_smaps = File::open("/proc/meminfo").unwrap();
    let buf_smaps=BufReader::new(f_smaps);
    let iter_s=buf_smaps.lines();
    for line_result in iter_s {
        let line =match line_result{
            Ok(x)=> x,
            Err(_)=>"-".to_string()
        };
        if line.contains("MemTotal:"){
            let total=line[9..].to_string().trim().to_string().trim_end_matches(" kB").to_string();
            mem.total=match i32::from_str_radix(total.as_str(), 10){
                Ok(x)=>x as f32,
                Err(_)=>0.0
            };
            continue;
        }
        if line.contains("MemFree:"){
            let free=line[8..].to_string().trim().to_string().trim_end_matches(" kB").to_string();
            mem.free= match i32::from_str_radix(free.as_str(), 10){
                Ok(x)=>x as f32,
                Err(_)=>0.0
            };
            continue;
        }
    }
    mem.kb_in_mb();
    format!("{:?}",mem)
}

#[tauri::command]
fn ver()->String{
    let home = "/proc/";
let mut path = PathBuf::new();
path.push(home);
let rd= match path.read_dir(){
    Ok(x)=>x,
    Err(e)=> return format!("Error {e}")
};
let mut list =Vec::<InfoProc>::new();
for entry_result in rd{
    let mut var_info=InfoProc{
        id: 0,
        name: " ".to_string(),
        state: " ".to_string(),
        vmsize: 0.0,
        error: " ".to_string()
    };
    let entry=entry_result.unwrap();
    let  name_file=entry.path().file_name().unwrap().to_string_lossy().into_owned();
    var_info.id= match name_file.parse() {
        Ok(x)=>x,
        _=>continue
    };
    let f = match File::open(entry.path().to_string_lossy().to_string()+"/status"){
        Ok(x)=>x,
        Err(e)=>{
            var_info.error=e.to_string();
            list.push(var_info);
            continue}
    };
    let buf=BufReader::new(f);
    let mut iter=buf.lines();
    var_info.name=iter.next().unwrap().ok().unwrap()[5..].to_string().trim().to_string();
    let f_smaps = match File::open(entry.path().to_string_lossy().to_string()+"/status"){
        Ok(x)=>x,
        Err(e)=>{
            var_info.error=e.to_string();
            list.push(var_info);
            continue}
    };
    let buf_smaps=BufReader::new(f_smaps);
    let iter_s=buf_smaps.lines();
    for line_result in iter_s {
        let line =match line_result{
            Ok(x)=> x,
            Err(_)=>"-".to_string()
        };
        if line.contains("State:"){
            var_info.state=line[6..].to_string().trim().to_string();
        }
        if line.contains("VmRSS:"){
            let vmsime=line[6..].to_string().trim().to_string().trim_end_matches(" kB").to_string();
            var_info.vmsize= match i32::from_str_radix(vmsime.as_str(), 10){
                Ok(x)=>x as f32,//(x*page_size::get() as i32/1024 )as f32,
                Err(_)=>0.0
            };
            continue;
        }
    }
    var_info.kb_in_mb();
    list.push(var_info);    
};
list.sort_by(|b, a| {a.vmsize.total_cmp(&b.vmsize)});
format!("{:?}", list)
}


fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let win = app.get_window("main").unwrap();
        win.show().unwrap();
        win.open_devtools();
        Ok(())}
        )
    .invoke_handler(tauri::generate_handler![ver, mem])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
