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
        
    
      this.count=1;};
    this.lines[this.count]=y;
    let ctx=this.canvas.getContext('2d');
    ctx.wigth=this.w;
    ctx.height=this.h;
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, this.w, this.h);
    
    for (let index = 0; index < this.lines.length; index++) {
      const y_e = this.lines[index];
      ctx.strokeStyle="blue";
      ctx.moveTo(index*this.wlines,this.h-y_e);
      ctx.lineTo(index*this.wlines,this.h);
      ctx.lineWidth=this.wlines;
      ctx.lineWidth=this.wlines;
      
      ctx.stroke();
    }
  }

}


let greetInputEl;
let greetMsgEl;
let versionMsgEl;
let interval;
let interval_mem;
let cheskRun = false;
let printt=new Intl.NumberFormat('ru-RU');
let startcraf=true;
let craf;

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

async function meminfo() {
  let mem=await invoke("mem");
  let obj=JSON.parse(mem);
  greetMsgEl.innerHTML=`Всего памяти: ${printt.format(obj.total)} Mb. Занято: ${printt.format(obj.total-obj.free)} Mb. Cвободно: ${printt.format(obj.free)} Mb.`;
  if (startcraf){
    startcraf=false;
    let canvas= document.querySelector('#canvas-id');
    craf = new graf(canvas.style.height, canvas.style.wight, obj.total, canvas);
   } else{
    craf.Pix(obj.total-obj.free);
   }
}

async function version() {
  let jso=await invoke("ver");
  let mas = jso.split(/{([\s\S]+?)}/);
  let s=
  `<div style="text-shadow: none; background-color: aliceblue;  height: 80vh; overflow:auto;" id="log_error" >`+
  `<table>
    <tr><th>ID</th><th>Имя</th><th>Размер в памяти</th><th>Cостояние</th></tr>
    `
  let count_vmsime=0;
  let text=greetInputEl.value;
  mas.forEach(function(item, index, array) {
    if (item.includes("id")){
      let d=`{${item}}`;
      let obj=JSON.parse(d);
      count_vmsime=count_vmsime+parseFloat(obj.vmsize,10);
      if (text.length>0){
        if (obj.name.includes(text)){
          if (obj.state.includes("R")){
            s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${obj.state}</td></tr>`;
          }
          else{
            s=s+`<tr><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${obj.state}</td></tr>`;
          }     
        }
      }
      else {
      if (obj.state.includes("R")){
        s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${new Intl.NumberFormat('ru-RU').format(obj.vmsize)} Mb.</td><td>${obj.state}</td></tr>`;
      }
      else{
        if (cheskRun.checked){
          if (obj.state.includes("R")){
            s=s+`<tr class="run" ><td>${obj.id}</td><td>${obj.name}</td><td>${printt.format(obj.vmsize)} Mb.</td><td>${obj.state}</td></tr>`;
          }
        } else {
        s=s+`<tr><td>${obj.id}</td><td>${obj.name}</td><td>${new Intl.NumberFormat('ru-RU').format(obj.vmsize)} Mb.</td><td>${obj.state}</td></tr>`;
        
        }
      }
    }
    
  }
    
  });
  s=s+'</table></div>'
  versionMsgEl.innerHTML=`Всего занято ${mas.length} процессами ${new Intl.NumberFormat('ru-RU').format(count_vmsime)} Mb. памяти. ${s}`;

  }
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  versionMsgEl = document.querySelector("#version-msg");
  cheskRun = document.querySelector("#checkrun");
  interval=setInterval(version, 1000);
  setInterval(meminfo, 2000);
  document
    .querySelector("#version-button").style.display = 'none';
    document
    .querySelector("#version-button")
    .addEventListener("click", () => start());
    document
    .querySelector("#stopversion-button")
    .addEventListener("click", () => stoptimer());
});
