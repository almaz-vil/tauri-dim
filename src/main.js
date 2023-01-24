const { invoke } = window.__TAURI__.tauri;

class graf{
  
 constructor(h,w, maxmemory, canvas){
    this.w=canvas.width;
    this.h=canvas.height;
    this.lines= new Array(100);
    this.wlines=this.w/this.lines.length;
    this.max=parseFloat(maxmemory);
    this.canvas=canvas;
    this.count=0;
    for (let index = 0; index < this.lines.length; index++) {
      this.lines[index]=0;      
    }
  }


  Pix(tek){
    let y=this.h/this.max*tek;
    this.count=this.count+1;
    if (this.count===100) {
      let ctx=this.canvas.getContext('2d');
      ctx.wigth=this.w;
      ctx.height=this.h;
      ctx.fillStyle = "black";
      ctx.fillRect(0, 0, this.w, this.h);
      for (let index = 0; index < this.lines.length; index++) {
        this.lines[index]=0;      
      }      
      this.count=1;
    };
    this.lines[this.count]=y;
    let ctx=this.canvas.getContext('2d');
    ctx.wigth=this.w;
    ctx.height=this.h;
    ctx.fillStyle = "#F0E68C";
    ctx.fillRect(0, 0, this.w, this.h);
    for (let index = 0; index < this.lines.length; index++) {
      const y_e = this.lines[index];
      ctx.strokeStyle="#32CD32";
      ctx.moveTo(index*this.wlines,this.h-y_e);
      ctx.lineTo(index*this.wlines,this.h);
      ctx.lineWidth=this.wlines;
      ctx.lineWidth=this.wlines;
      
      ctx.stroke();
    }
  }

}

let procInfoSum;
let greetInputEl;
let greetMsgEl;
let versionMsgEl;
let interval;
let interval_mem;
let cheskRun = false;
let printt=new Intl.NumberFormat('ru-RU');
let startcraf=true;
let craf;
let msgLoadCPU;

let paramsort=3;

async function start() {
  interval=setInterval(version, 1000);

  document.querySelector("#version-button").style.display = 'none';
  document.querySelector("#stopversion-button").style.display = 'block';
}

function stoptimer(){
  clearInterval(interval);
  document.querySelector("#stopversion-button").style.display = 'none';
  document.querySelector("#version-button").style.display = 'block';   
}

async function loadcpu(){
  let loadcpu= await invoke("loadcpu");
  let obj = JSON.parse(loadcpu);
  if(obj.loadm>obj.loadm5){
    if(obj.loadm5>obj.loadm15){
      msgLoadCPU.innerHTML=` 1 мин: ${obj.loadm} 5 мин: <b>${obj.loadm5}</b> 15 мин: ${obj.loadm15}`;
      if(obj.loadm>obj.loadm5){
        msgLoadCPU.innerHTML=` 1 мин: <b>${obj.loadm}</b> 5 мин: <b>${obj.loadm5}</b> 15 мин: ${obj.loadm15}`;
      }
    }else{
      msgLoadCPU.innerHTML=` 1 мин: <b>${obj.loadm}</b> 5 мин: ${obj.loadm5} 15 мин: ${obj.loadm15}`;
    }
  }else{
  msgLoadCPU.innerHTML=` 1 мин: ${obj.loadm} 5 мин: ${obj.loadm5} 15 мин: ${obj.loadm15}`;
  }
}

async function meminfo() {
  let mem=await invoke("mem");
  let obj=JSON.parse(mem);
  let mem_used=obj.total-obj.free;
  if (obj.free<mem_used){
    greetMsgEl.innerHTML=`Всего памяти: ${printt.format(obj.total)}&nbsp;Mb.<br>Занято: <b>${printt.format(obj.total-obj.free)}</b>&nbsp;Mb.<br>Cвободно: ${printt.format(obj.free)}&nbsp;Mb.`;
  }else{
    greetMsgEl.innerHTML=`Всего памяти: ${printt.format(obj.total)}&nbsp;Mb.<br>Занято: ${printt.format(obj.total-obj.free)}&nbsp;Mb.<br>Cвободно: ${printt.format(obj.free)}&nbsp;Mb.`;
  }

  if (startcraf){
    startcraf=false;
    let canvas= document.querySelector('#canvas-id');
    craf = new graf(canvas.style.height, canvas.style.wight, obj.total, canvas);
   } else{
    craf.Pix(mem_used);
   }
}

function frop(s) {
  console.log("{}", s);
}

async function version() {
  let jso=await invoke("ver", {insort: paramsort});
  let mas = jso.split(/{([\s\S]+?)}/);
  let s=
  `<div style="text-shadow: none; background-color: aliceblue;  height: 80vh; overflow:auto;" id="log_error" >`+
  `<table>
    <tr><th onclick="paramsort=0;' ">ID</th><th onclick="paramsort=1;">Имя</th><th onclick="paramsort=2;">Размер в памяти</th><th onclick="paramsort=3;">CPU</th><th onclick="paramsort=4;">Cостояние</th></tr>
    `
  let count_vmsime=0;
  let count_cpu=0;
  let text=greetInputEl.value;
  mas.forEach(function(item, index, array) {
    if (item.includes("id")){
      let d=`{${item}}`;
      let obj=JSON.parse(d);
      count_vmsime=count_vmsime+parseFloat(obj.vmsize,10);
      count_cpu=count_cpu+parseInt(obj.loadcpu);
      if (text.length>0){
        if (obj.name.includes(text)){
          if (obj.state.includes("R")){
            s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${printt.format(obj.loadcpu)}%</td><td>${obj.state}</td></tr>`;
          }
          else{
            s=s+`<tr><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${printt.format(obj.loadcpu)}%</td><td>${obj.state}</td></tr>`;
          }     
        }
      }
      else {
      if (obj.state.includes("R")){
        s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${new Intl.NumberFormat('ru-RU').format(obj.vmsize)} Mb.</td><td>${printt.format(obj.loadcpu)}%</td><td>${obj.state}</td></tr>`;
      }
      else{
        if (cheskRun.checked){
          if (obj.state.includes("R")){
            s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${printt.format(obj.loadcpu)}%</td><td>${obj.state}</td></tr>`;
          }
        } else {
        s=s+`<tr><td>${obj.id}</td><td>${obj.name}</td><td>${new Intl.NumberFormat('ru-RU').format(obj.vmsize)} Mb.</td><td>${printt.format(obj.loadcpu)}%</td><td>${obj.state}</td></tr>`;
        
        }
      }
    }
    
  }
    
  });
  s=s+'</table></div>';
  versionMsgEl.innerHTML=`${s}`;
  procInfoSum.innerHTML=`Всего занято ${mas.length} процессами ${new Intl.NumberFormat('ru-RU').format(count_vmsime)}&nbsp;Mb. памяти. CPU:${count_cpu}%`;
  }
 

  
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  procInfoSum = document.querySelector("#infoproc");
  versionMsgEl = document.querySelector("#version-msg");
  cheskRun = document.querySelector("#checkrun");
  msgLoadCPU=document.querySelector("#msgloadcpu");
  interval=setInterval(version, 3000);
  setInterval(meminfo, 4000);
  setInterval(loadcpu, 4000);
  document
    .querySelector("#version-button").style.display = 'none';
    document
    .querySelector("#version-button")
    .addEventListener("click", () => start());
    document
    .querySelector("#stopversion-button")
    .addEventListener("click", () => stoptimer());
    document
    .querySelector("#id-button")
    .addEventListener("click", () => paramsort=0);
    document
    .querySelector("#name-button")
    .addEventListener("click", () => paramsort=1);
    document
    .querySelector("#mem-button")
    .addEventListener("click", () => paramsort=2);
    document
    .querySelector("#cpu-button")
    .addEventListener("click", () => paramsort=3);
});
