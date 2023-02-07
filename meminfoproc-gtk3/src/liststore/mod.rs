use super::info::InfoProc;

use gtk::glib;
use gtk::prelude::*;

enum Columns {
    ID,
    Name,
    Memory,
    State,
    CPU,
}

pub fn create_model(data: Vec<InfoProc>)->gtk::ListStore{
    let col_types: [glib::Type; 5] = [
        glib::Type::I32,
        glib::Type::STRING,
        glib::Type::STRING,
        glib::Type::STRING,
        glib::Type::STRING,
        ];
    let store = gtk::ListStore::new(&col_types);

    for (_, col) in data.iter().enumerate() {
        let values:[(u32, &dyn ToValue); 5] =[
            (0, &col.id),
            (1, &col.name),
            (3, &col.state),
            (2, &format!("{:.2} Mb",&col.vmsize)),
            (4, &format!("{:.2}%",&col.loadcpu)),
        ];
        store.set(&store.append(), &values);
    }
    store
}

pub fn update_model(model: &gtk::ListStore, data: Vec<InfoProc>){
    model.clear();
    for (_i, col) in data.iter().enumerate() {
        let values:[(u32, &dyn ToValue); 5] =[
            (0, &col.id),
            (1, &col.name),
            (3, &col.state),
            (2, &format!("{:.2} Mb",&col.vmsize)),
            (4, &format!("{:.2}%",&col.loadcpu)),
        ];
        model.set(&model.append(), &values);
    }
}

pub fn add_columns(treeview: &gtk::TreeView, ) {
     
    // Column for bug ID
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        TreeViewColumnExt::pack_start(&column, &renderer, true);
        column.set_title("ID");
        TreeViewColumnExt::add_attribute(&column, &renderer, "text", Columns::ID as i32);
        column.set_sort_column_id(Columns::ID as i32);
        treeview.append_column(&column);
    }

    // Column for Name
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        TreeViewColumnExt::pack_start(&column, &renderer, true);
        column.set_title("Имя");
        TreeViewColumnExt::add_attribute(&column, &renderer, "text", Columns::Name as i32);
        column.set_sort_column_id(Columns::Name as i32);
        treeview.append_column(&column);
    }

    // Column for bug Memory
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        TreeViewColumnExt::pack_start(&column, &renderer, true);
        column.set_title("Память");
        TreeViewColumnExt::add_attribute(&column, &renderer, "text", Columns::Memory as i32);
        column.set_sort_column_id(Columns::Memory as i32);
        treeview.append_column(&column);
    }

    // Column for State
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        TreeViewColumnExt::pack_start(&column, &renderer, true);
        column.set_title("Статус");
        TreeViewColumnExt::add_attribute(&column, &renderer, "text", Columns::State as i32);
        column.set_sort_column_id(Columns::State as i32);
        treeview.append_column(&column);
    }

    // Column for bug CPU
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        TreeViewColumnExt::pack_start(&column, &renderer, true);
        column.set_title("CPU");
        TreeViewColumnExt::add_attribute(&column, &renderer, "text", Columns::CPU as i32);
        column.set_sort_column_id(Columns::CPU as i32);
        treeview.append_column(&column);
    }
}

