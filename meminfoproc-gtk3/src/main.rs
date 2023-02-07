mod info;
mod liststore;

use info::*;
use liststore::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    let app = Application::builder()
        .application_id("ru.Dimon.MemInfoProc-gtk")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let edit = gtk::Entry::new();
    let label_mem_free= gtk::Label::new(Some("Общая память"));
    label_mem_free.set_halign(gtk::Align::Start);
    let label_mem_total= gtk::Label::new(Some("Общая память"));
    label_mem_total.set_halign(gtk::Align::Start);
    let label_mem_zaneto= gtk::Label::new(Some("Общая память"));
    label_mem_zaneto.set_halign(gtk::Align::Start);
    let label_proc_total= gtk::Label::new(Some("Процессов"));
    label_proc_total.set_halign(gtk::Align::Start);
    let label_load_avg= gtk::Label::new(Some("Загрузка CPU"));
    label_load_avg.set_halign(gtk::Align::Start);
    let box_h_in_line=gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::End)
        .name("panelfon")
        .build();
  
    let box_v_chek=gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical).build();
    let box_v=gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical).build();
    let box_label_v=gtk::Box::builder()
       .orientation(gtk::Orientation::Vertical)
       .halign(gtk::Align::Fill)
       .margin_bottom(2)
       .margin_start(2)
       .margin_end(2)
       .build();
    let check_run=gtk::CheckButton::builder()
        .label("Только выполняемые")
        .build();
    let check_stop=gtk::CheckButton::builder()
        .label("Остановка")
        .build();
    box_v_chek.add(&edit);
    box_v_chek.add(&check_run) ;
    box_v_chek.add(&check_stop);
    box_h_in_line.add(&box_v_chek);
    box_label_v.add(&label_load_avg);
    box_label_v.add(&label_proc_total);
    box_label_v.add(&label_mem_total);
    box_label_v.add(&label_mem_free);
    box_label_v.add(&label_mem_zaneto);
    
    let botton_id=gtk::RadioButton::builder()
        .label("ID")
        .active(false)
        .build();
    botton_id.set_widget_name("labelmemtotal");
    let botton_name=gtk::RadioButton::from_widget(&botton_id);
    botton_name.set_label("Имя");
    let botton_cpu=gtk::RadioButton::from_widget(&botton_id);
    botton_cpu.set_label("CPU");
    let botton_memory=gtk::RadioButton::from_widget(&botton_id);
    botton_memory.set_label("Память");
    botton_memory.set_active(true);
    let botton_state=gtk::RadioButton::from_widget(&botton_id);
    botton_state.set_label("Статус");

    let box_select_v=gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal).build();
    box_select_v.add(&botton_id);
    box_select_v.add(&botton_name);
    box_select_v.add(&botton_cpu);
    box_select_v.add(&botton_memory);
    box_select_v.add(&botton_state);

    label_load_avg.set_widget_name("labelmemtotal");
    label_mem_total.set_widget_name("labelmemtotal");
    label_mem_free.set_widget_name("labelmemfree");
    label_mem_zaneto.set_widget_name("labelmemzaneto");
    
    let mem_tisk=move ||{
        let mut mem=InfoMem::mem();
        mem.kb_in_mb();
        label_mem_total.set_markup(format!("Всего памяти: <b>{:.2}</b> Мб.", mem.total).as_str());
        label_mem_free.set_markup(format!("Свободно: <b>{:.2}</b> Мб.", mem.free).as_str());
        label_mem_zaneto.set_markup(format!("Занято: <b>{:.2}</b> Мб", mem.total-mem.free).as_str());
        glib::Continue(true)
    };
    let frame_canvas=gtk::Frame::builder()
        .halign(gtk::Align::End)
        .valign(gtk::Align::End)
        .build();
    frame_canvas.set_height_request(80);
    frame_canvas.set_width_request(250);
    frame_canvas.set_widget_name("canvas");
    let area=gtk::DrawingArea::new();
    frame_canvas.add(&area);
    box_h_in_line.add(&frame_canvas);
    box_h_in_line.add(&box_label_v);
   
    box_v.add(&box_h_in_line);
    box_v.add(&box_select_v);
    
    let box_scrolled_list =gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .propagate_natural_height(true)
        .propagate_natural_width(true)
        .build();
        
    box_v.add(&box_scrolled_list);
   
    let  lines = RefCell::new(VecDeque::<f64>::with_capacity(200));
    let wlines=frame_canvas.width_request()as f64/400f64;
    let height=frame_canvas.height_request() as f64;  
     
    area.connect_draw(move |_, c| {
        if lines.borrow().len() == 400 {
            lines.borrow_mut().pop_front();
        }
        let  mem=InfoMem::mem();
        let y = (height/mem.total)*(mem.total-mem.free);
        lines.borrow_mut().push_back(y);
        c.set_source_rgb(0f64, 0f64, 0f64);
        c.rectangle(0., 0., frame_canvas.width_request() as f64, height);
        let _=c.fill();
        c.set_source_rgb(0f64, 255f64, 0f64);
        c.set_line_width(wlines);
        let mut i=0.0;
        for y_e in lines.borrow().iter(){
            i=i+1.0;
            let x=i*wlines;
           c.move_to(x, height-y_e);
           c.line_to(x, height);
           let _=c.stroke();
        };     
        gtk::Inhibit(true)
    });
    
    let run= check_run.is_active();
    let model = Rc::new(create_model(InfoProc::proc_info(SortInfoProc::CPU, edit.text().to_string(), run)));
    let treeview = gtk::TreeView::with_model(&*model);
    add_columns(&treeview);
    
    label_proc_total.set_widget_name("labelproctotal");
   
    let proc_tisk=move ||{
     
        let run= check_run.is_active();     
        area.queue_draw();
        let mut b=SortInfoProc::ID;
        if botton_cpu.is_active(){
            b=SortInfoProc::CPU;
        };
        if botton_memory.is_active(){
            b=SortInfoProc::Memory;
        };
        if botton_name.is_active(){
            b=SortInfoProc::Name;
        };
        if botton_state.is_active(){
            b=SortInfoProc::State;
        };
        if botton_id.is_active(){
            b=SortInfoProc::ID;
        };
        let load_avg=LoadAvg::get_load_avg();
        let vec_info_proc=InfoProc::proc_info(b, edit.text().to_string(), run);
        let mut count_mem_proc=0.0;
        vec_info_proc.iter().for_each(|v|{
            count_mem_proc+=v.vmsize;
        });
        let count_proc=vec_info_proc.len();   
        if !check_stop.is_active(){
            update_model(&model,vec_info_proc);
        }    
        label_load_avg.set_markup(format!("Нагрузка CPU: за 1 мин: <b>{:.2}</b> 5 мин: <b>{:.2}</b> 15 мин: <b>{:.2}</b>", load_avg.loadm, load_avg.loadm_5, load_avg.loadm_15).as_str());
        label_proc_total.set_markup(format!("Процессов: <b>{}</b> занято памяти <b>{:.2}</b> Мб.",count_proc, count_mem_proc).as_str());
        glib::Continue(true)
    };
    
    box_scrolled_list.add(&treeview); 
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(620)
        .default_height(900)
        .title("MemInfoProc-gtk")
        .child(&box_v)
        .build();
    let css = gtk::CssProvider::new();
    css.load_from_data(include_bytes!("main.css")).unwrap();

    gtk::StyleContext::add_provider_for_screen(
    &gdk::Screen::default().expect("Error initializing gtk css provider."),
         &css,
         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    window.show_all();
    glib::timeout_add_seconds_local(1, mem_tisk);
    glib::timeout_add_seconds_local(1, proc_tisk);
}