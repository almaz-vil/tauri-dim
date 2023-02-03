mod info;
mod liststore;

use info::*;
use liststore::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("ru.Dimon.MemInfoProc-gtk")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    
    let edit = gtk::Entry::new();
    let  label_mem_free= gtk::Label::new(Some("Общая память"));
    label_mem_free.set_halign(gtk::Align::Start);
    let  label_mem_total= gtk::Label::new(Some("Общая память"));
    label_mem_total.set_halign(gtk::Align::Start);
    let  label_mem_zaneto= gtk::Label::new(Some("Общая память"));
    label_mem_zaneto.set_halign(gtk::Align::Start);
    let  label_proc_total= gtk::Label::new(Some("Процессов"));
    label_proc_total.set_halign(gtk::Align::Start);
    let  label_load_avg= gtk::Label::new(Some("Загрузка CPU"));
    label_load_avg.set_halign(gtk::Align::Start);
    let box_h=gtk::Box::builder()
    .orientation(gtk::Orientation::Vertical).build();
    let gtk_box_g=gtk::Box::builder()
       .orientation(gtk::Orientation::Vertical)
       .halign(gtk::Align::Fill)
       .margin_top(12)
       .margin_bottom(2)
       .margin_start(2)
       .margin_end(2)
       .build();
    gtk_box_g.add(&edit);
    gtk_box_g.add(&label_load_avg);
    gtk_box_g.add(&label_proc_total);
    gtk_box_g.add(&label_mem_total);
    gtk_box_g.add(&label_mem_free);
    gtk_box_g.add(&label_mem_zaneto);
    box_h.add(&gtk_box_g);

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

    let box_v=gtk::Box::builder()
    .orientation(gtk::Orientation::Horizontal).build();
    box_v.add(&botton_id);
    box_v.add(&botton_name);
    box_v.add(&botton_cpu);
    box_v.add(&botton_memory);
    box_v.add(&botton_state);
    box_h.add(&box_v);
    gtk_box_g.set_widget_name("panel");
    label_load_avg.set_widget_name("labelmemtotal");
    label_mem_total.set_widget_name("labelmemtotal");
    label_mem_free.set_widget_name("labelmemfree");
    label_mem_zaneto.set_widget_name("labelmemzaneto");
    let mem_tisk=move ||{
        let mut mem=InfoMem::mem();
        mem.kb_in_mb();
        label_mem_total.set_markup(format!("Всего памяти: <b>{:.2}</b> Гб.", mem.total).as_str());
        label_mem_free.set_markup(format!("Свободно: <b>{:.2}</b> Гб.", mem.free).as_str());
        label_mem_zaneto.set_markup(format!("Занято: <b>{:.2}</b> Гб", mem.total-mem.free).as_str());
        glib::Continue(true)
    };
    let scrolled =gtk::ScrolledWindow::builder()
    .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
     .propagate_natural_height(true)
     .propagate_natural_width(true)
    .build();
    box_h.add(&scrolled);

    let model = Rc::new(create_model(InfoProc::proc_info(SortInfoProc::CPU, edit.text().to_string())));
    let treeview = gtk::TreeView::with_model(&*model);
    scrolled.add(&treeview);
    add_columns(&treeview);
     label_proc_total.set_widget_name("labelproctotal");
    let proc_tisk=move ||{
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
        let vec_info_proc=InfoProc::proc_info(b, edit.text().to_string());
        let mut count_mem_proc=0.0;
        vec_info_proc.iter().for_each(|v|{
            count_mem_proc+=v.vmsize;
        });
        let count_proc=vec_info_proc.len();
        update_model(&model,vec_info_proc);
        label_load_avg.set_markup(format!("Нагрузка CPU: за 1 мин:<b>{:.2}</b> 5 мин:<b>{:.2}</b> 15 мин.<b>{:.2}</b>", load_avg.loadm, load_avg.loadm_5, load_avg.loadm_15).as_str());
        label_proc_total.set_markup(format!("Процессов: <b>{}</b>  занято памяти <b>{:.2}</b>Мб.",count_proc, count_mem_proc).as_str());
        glib::Continue(true)
    };
    
    
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(620)
            .default_height(900)
            .title("MemInfoProc-gtk")
            .child(&box_h)
            .build();
         let css = gtk::CssProvider::new();
        css.load_from_data(include_bytes!("main.css")).unwrap();

       gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
         &css,
         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Show the window.
        window.show_all();
        glib::timeout_add_seconds_local(1, mem_tisk);
        glib::timeout_add_seconds_local(1, proc_tisk);


}