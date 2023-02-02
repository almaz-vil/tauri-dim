use std::{fs::File, io::{BufReader, BufRead}};
use std::fmt;
use std::path::PathBuf;
use std::f32;
use std::str::FromStr;
///Информация о памяти
pub struct InfoMem{
    ///Доступная память **total** 
    pub total: f32,
    ///Свободная память **free** 
    pub free: f32,
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
impl InfoMem {
    pub fn kb_in_mb(&mut self){
        self.total=self.total/1024.00;
        self.free=self.free/1024.00;
    }
    ///Загрузка из системы proc
    pub fn mem()->InfoMem{
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
        mem
    }
}

///Информация о загрузки процессора
/// #[derive(Clone)]
pub struct LoadAvg{
   pub loadm: f32,
   pub loadm_5: f32,
   pub loadm_15: f32,    
}

impl LoadAvg{
    pub fn get_load_avg()->LoadAvg{
        let mut loadcpu= LoadAvg{
            loadm: 0.0,
            loadm_5: 0.0,
            loadm_15: 0.0,
        };
        let f_smaps = File::open("/proc/loadavg").unwrap();
        let buf_smaps=BufReader::new(f_smaps);
        let mut iter_s=buf_smaps.lines();
        let line= match iter_s.next() {
            Some(l) => l.unwrap(),
            None => return  loadcpu
        };
        let v: Vec<&str>=line.split(" ").collect();
        loadcpu.loadm=v[0].parse::<f32>().unwrap();
        loadcpu.loadm_5=v[1].parse::<f32>().unwrap();
        loadcpu.loadm_15=v[2].parse::<f32>().unwrap();
        loadcpu
    }
}    

pub enum SortInfoProc{
    Name,
    ID,
    Memory,
    State,
    CPU
} 
#[derive(Clone,  PartialEq, PartialOrd)]
pub struct InfoProc{
   pub id: i32,
   pub name: String,
   pub state: String,
   pub vmsize: f32,
   pub loadcpu: f64,
   pub error: String,
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
        s.push_str("\", \"loadcpu\":\"");
        s.push_str(&self.loadcpu.floor().to_string());
        s.push_str("\", \"vmsize\":\"");
        s.push_str(&self.vmsize.to_string());
        s.push_str("\", \"error\":\"");
        s.push_str(&self.error);
        s.push_str("\"}");    
        f.write_str(s.as_str())        
    }
}

impl InfoProc {
    pub fn kb_in_mb(&mut self){
        self.vmsize=self.vmsize/1024.00;
    }
    pub fn loadcpu(&mut self, namefile: String){
        let mut sysytem_uptime_sec=0.0;
        let f_stat = match File::open("/proc/uptime"){
            Ok(x)=>x,
            Err(e)=> {self.error=e.to_string(); return}
        };
        let buf_smaps=BufReader::new(f_stat);
        let iter_s=buf_smaps.lines();
        for line_result in iter_s {
            let line =match line_result{
                Ok(x)=> x,
                Err(_)=>"-".to_string()
            };
            let mut s_p=line.split_whitespace();
            let uptime=s_p.nth(0).unwrap();
            sysytem_uptime_sec= match f64::from_str(uptime){
                Ok(x)=>x as f64,
                Err(_)=>0.0
            };
        }

        let f_stat = match File::open(namefile+"/stat"){
            Ok(x)=>x,
            Err(e)=> {self.error=e.to_string(); return}
        };
        let buf_smaps=BufReader::new(f_stat);
        let iter_s=buf_smaps.lines();
        for line_result in iter_s {
            let line =match line_result{
                Ok(x)=> x,
                Err(_)=>"-".to_string()
            };
            let  stat=line.split_whitespace();
            let mut il=0;
            let mut utimm="0";
            let mut stimm="0";
            let mut process_starttime="0";
            for s in stat.clone()  {
                match il {
                    13=>utimm=s,
                    14=>stimm=s,
                    21=>process_starttime=s,
                    _=>()
                }
                il=il+1;

            }
            let ut=100.00  ;
            let utime=f64::from_str(utimm).unwrap();
            let stime=f64::from_str(stimm).unwrap();
            let startime=f64::from_str(process_starttime).unwrap();
            let process_utime_sec=utime/ut;
            let process_stime_sec=stime/ut;
            let process_startime_sec=startime/ut;
            let process_elapsed_sec=sysytem_uptime_sec-process_startime_sec;
            let process_usage_sec=process_utime_sec+process_stime_sec;
            let mut process_usage=1.00;
            if process_elapsed_sec>0.0 {
            process_usage=process_usage_sec*100.00/process_elapsed_sec;}
            self.loadcpu=process_usage;
            
        }
    }


    pub fn proc_info(sort: SortInfoProc, filter: String)->Vec<InfoProc>{
        let home = "/proc/";
        let mut path = PathBuf::new();
        path.push(home);
        let mut list =Vec::<InfoProc>::new();
        let rd= match path.read_dir(){
            Ok(x)=>x,
            Err(e)=> {list.push(InfoProc{
                id: 0,
                name: " ".to_string(),
                state: " ".to_string(),
                vmsize: 0.0,
                loadcpu:0.0,
                error: format!("Ошибка {}",e)
            }); return list;}
        };
        let mut list =Vec::<InfoProc>::new();
        for entry_result in rd{
            let mut var_info=InfoProc{
                id: 0,
                name: " ".to_string(),
                state: " ".to_string(),
                vmsize: 0.0,
                loadcpu:0.0,
                error: " ".to_string()
            };
            let entry=entry_result.unwrap();
            let  name_file=entry.path().file_name().unwrap().to_string_lossy().into_owned();
            var_info.id= match name_file.parse() {
                Ok(x)=>x,
                _=>continue
            };
            let n_file=entry.path().to_string_lossy().into_owned().as_str().to_string();
            let f = match File::open(n_file.clone()+"/status"){
                Ok(x)=>x,
                Err(e)=>{
                    var_info.error=e.to_string();
                    list.push(var_info);
                    continue}
            };
            let buf=BufReader::new(f);
            let mut iter=buf.lines();
            let name=iter.next().unwrap().ok().unwrap()[5..].to_string().trim().to_string();
            if filter.len()>0{
                if name.contains(&filter){
                    var_info.name=name;
                } else{
                    continue;
                }
            } else {
               var_info.name=name;
            }
            let f_smaps = match File::open(n_file.clone()+"/status"){
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
            var_info.loadcpu(n_file.clone());
            list.push(var_info);    
        };
        match sort {
            SortInfoProc::ID=>{list.sort_by(|a, b| {a.id.cmp(&b.id)});} ,
            SortInfoProc::Name=>{list.sort_by(|a, b| {a.name.cmp(&b.name)});},
            SortInfoProc::CPU=>{list.sort_by(|b, a| {a.loadcpu.total_cmp(&b.loadcpu)});},
            SortInfoProc::Memory=> {list.sort_by(|b, a| {a.vmsize.total_cmp(&b.vmsize)});},
            SortInfoProc::State=>{list.sort_by(|b, a| {a.state.cmp(&b.state)});},
        }
        
        list
    }
}