mod info;

use info::InfoProc;
use info::InfoMem;
/*use crate::grid_cell::Entry;
use crate::grid_cell::GridCell;
*/use gtk4 as gtk;
use gdk4 as gdk;
use gtk::glib;
use gtk::prelude::*;
//use gtk::gio;
//use gtk::glib::BoxedAnyObject;

use gtk::{Application, ApplicationWindow};


/*
struct Row {
    col1: String,
    col2: String,
}

use std::cell::Ref;
*/

/*\\
   
#[derive(Default)]
pub struct FeedItemTemplate {
  name: Cell<String>,
  url: Cell<String>,
}
#[object_subclass]
impl ObjectSubclass for FeedItemTemplate {
  const NAME: &'static str = "FeedItem";
  type Type = FeedItem;
}
impl ObjectImpl for FeedItemTemplate {
  fn properties() -> &'static [ParamSpec] {
    static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
      vec![
        ParamSpecString::new("name", "name", "The name of the RSS 
          feed", Some(""), ParamFlags::READWRITE),
        ParamSpecString::new("url", "url", "The url of the RSS 
          feed", Some(""), ParamFlags::READWRITE),]
    });
    PROPERTIES.as_ref()
  }
  fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
    match pspec.name() {
      "name" => {
        let name_string = value.get()
          .expect("The value needs to be of type `String`.");
        self.name.replace(name_string);
      }
      "url" => {
        let url_string = value.get()
          .expect("The value needs to be of type `String`.");
        self.url.replace(url_string);
      }
      _ => unimplemented!(),
    }
  }
fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
    match pspec.name() {
      "name" => {
        let result = self.name.take();
        
        self.name.set(result.clone());
        result.to_value()
      }
      "url" => {
        let result = self.url.take();
        self.url.set(result.clone());
        result.to_value()
      }
      _ => unimplemented!(),
    }
  }
}

wrapper! {
    pub struct FeedItem(ObjectSubclass<FeedItemTemplate>);
  }
  impl FeedItem {
    pub fn new(name: &str, url: &str) -> Self {
      Object::new(&[("name", &name), ("url", &url)])
        .expect("Failed to create `FeedItem`.")
    }
  }
*/  
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
    gtk_box_g.append(&edit);
    gtk_box_g.append(&label_proc_total);
    gtk_box_g.append(&label_mem_total);
    gtk_box_g.append(&label_mem_free);
    gtk_box_g.append(&label_mem_zaneto);
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
    let text=gtk::TextBuffer::new(None);
    let view=gtk::TextView::with_buffer(&text);
    view.set_widget_name("textw");
    view.set_buffer(Some(&text));
    
    let scrolled_window =gtk::ScrolledWindow::builder()
    .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
     .child(&view)
     .propagate_natural_height(true)
     .propagate_natural_width(true)
    .build();
    gtk_box_g.append(&scrolled_window);
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
        css.load_from_data(include_bytes!("main.css"));
 
       let sr=gdk::Display::default().unwrap();
       gtk::StyleContext::add_provider_for_display(&sr, &css,
         gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Show the window.
        window.present();
        glib::timeout_add_seconds_local(1, mem_tisk);
        glib::timeout_add_seconds_local(1, proc_tisk);
    
   
}