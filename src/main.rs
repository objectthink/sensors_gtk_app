use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use gtk::glib::timeout_add_local;
use gtk::subclass::prelude::ClassStruct;
use gtk::{prelude::*, Button};
use gtk::{Application, ApplicationWindow, Label, Box, Orientation, ListBox};
use gtk::AccessibleRole::ListItem;
use log::info;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorStatus {
    pub name: String,
    pub location: String,
    pub temperature: u8,
    pub high: u8,
    pub low: u8,
    pub humidity: u8
}

fn main() {
    env_logger::init();

    let application = Application::builder()
        .application_id("sensors")
        .build();
    
        application.connect_activate(build_ui);
        application.run();
}

fn build_ui(application: &Application) {

    let label = Label::builder()
        .label("sensors")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    
    let button = Button::builder()
        .label("Office")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let clear_button = Button::builder()
        .label("Basement")
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let list = ListBox::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let label_777 = Label::new(Some("777"));
    //let label_380 = Label::new(Some("380"));
    //let label_334 = Label::new(Some("334"));

    list.append(&label_777);

    let content = Box::new(Orientation::Vertical, 0);

    content.append(&label);
    content.append(&button);
    content.append(&clear_button);
    content.append(&list);

    let arc_777_label = Arc::new(label_777);
    let clone_777_1 = Arc::clone(&arc_777_label);
    let clone_777_2 = Arc::clone(&arc_777_label);

    let nats_update = Arc::clone(&arc_777_label);

    let window = ApplicationWindow::builder()
        .title("sensors")
        .application(application)
        .child(&content)
        .build();
        
    window.show();

    //TEST
    // It is a riddle, wrapped in a mystery, inside an enigma: Churchill
    
    let riddle = String::from("riddle");
    let mystery = Mutex::new(riddle);
    let enigma = Arc::new(&mystery);
    /////
    
    let sensor = String::from("");
    let arc_sensor = Arc::new(Mutex::new(sensor));
    let arc_sensor_clone = Arc::clone(&arc_sensor);
    let arc_sensor_clone_2 = Arc::clone(&arc_sensor);

    timeout_add_local(Duration::from_secs(1), move || {
        nats_update.set_text(&arc_sensor_clone.lock().unwrap());
        Continue(true)
    });

    let location_search = String::from("Office");
    let arc_locaton_search = Arc::new(Mutex::new(location_search));
    let arc_location_search_clone_1 = Arc::clone(&arc_locaton_search);
    let arc_location_search_clone_2 = Arc::clone(&arc_locaton_search);
    let arc_location_search_clone_3 = Arc::clone(&arc_locaton_search);

    clear_button.connect_clicked(move |_| {
        *arc_location_search_clone_1.lock().unwrap() = String::from("Basement");
        
        clone_777_1.set_text("");
    });

    button.connect_clicked(move |_| {
        *arc_location_search_clone_2.lock().unwrap() = String::from("Office");

        clone_777_2.set_text("777");    
    });

    thread::spawn(move || {
        let nc = nats::connect("10.5.1.216:4222").expect("connect failed");
        let sub = nc.subscribe("*.status").expect("");

        for msg in sub.messages() {
            info!("Received a request {msg:?}");

            let sensor_msg = msg.to_string();
            let sensor_status_string = String::from_utf8_lossy(&msg.data).to_string();
            let sensor_status: SensorStatus = serde_json::from_str(&sensor_status_string).unwrap();

            info!("{:?}", sensor_status);

            // if sensor_status.name.eq("breadboard") {
            //     *arc_sensor_clone_2.lock().unwrap() = String::from_utf8_lossy(&msg.data).to_string();
            //     //*arc_sensor_clone_2.lock().unwrap() = sensor_status_string.clone();
            // }

            let s = &arc_location_search_clone_3.lock().unwrap().to_owned();
            if sensor_status.location.eq(s) {
                info!("{:?}", sensor_status_string);
                *arc_sensor_clone_2.lock().unwrap() = sensor_status_string.clone();
            }
        }    
    });

}

fn clear(sne_label: &Arc<Label>) {
}

fn set(sne_label: &Arc<Label>) {
}

fn flip_coin(label: &Label, sne_label: &Rc<Label>, amc_label: &Label) {
    if random() {
        label.set_text("heads");
        sne_label.set_text("sne heads");
        amc_label.set_text("amc")
    }else{
        label.set_text("tails");
        sne_label.set_text("sne");
        amc_label.set_text("amc tails")
    }
}