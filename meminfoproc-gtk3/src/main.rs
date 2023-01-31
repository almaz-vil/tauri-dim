mod info;

use info::InfoProc;
use info::InfoMem;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

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
    let  label_mem_total= gtk::Label::new(Some("Общая память"));
    let  label_mem_zaneto= gtk::Label::new(Some("Общая память"));
    let  label_proc_total= gtk::Label::new(Some("Всего процессов"));
    let gtk_box_g=gtk::Box::builder()
       .orientation(gtk::Orientation::Vertical)
       .halign(gtk::Align::Fill)
       .margin_top(12)
       .margin_bottom(2)
       .margin_start(2)
       .margin_end(2)
       .build();  
    gtk_box_g.add(&edit);
    gtk_box_g.add(&label_proc_total);
    gtk_box_g.add(&label_mem_total);
    gtk_box_g.add(&label_mem_free);
    gtk_box_g.add(&label_mem_zaneto);
    gtk_box_g.set_widget_name("panel");
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
    let text=gtk::TextBuffer::builder().build();
    
    let view=gtk::TextView::with_buffer(&text);
    view.set_widget_name("textw");
    view.set_buffer(Some(&text));
    
    let scrolled_window =gtk::ScrolledWindow::builder()
    .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
     .child(&view)
     .propagate_natural_height(true)
     .propagate_natural_width(true)
    .build();
    gtk_box_g.add(&scrolled_window);
    label_proc_total.set_widget_name("labelproctotal");
    let proc_tisk=move ||{
      let vec_info_proc:Vec<InfoProc>=InfoProc::proc_info(2, edit.text().to_string());
      
      label_proc_total.set_markup(format!(" Всего процессов: <b>{}</b>",vec_info_proc.len()).as_str());
       let mut s="".to_string();
       for p_i in &vec_info_proc{
        s.push_str(format!("{:.7}\t{:^52}\t{:.20}\t{:.2}Mb.\t{:.2}%\n", p_i.id, p_i.name, p_i.state, p_i.vmsize, p_i.loadcpu).as_str());
        
        };
        text.set_text(s.as_str());
        s.clear();
        glib::Continue(true)
    };

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(620)
            .default_height(900)
            .title("MemInfoProc-gtk")
            .child(&gtk_box_g)  
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