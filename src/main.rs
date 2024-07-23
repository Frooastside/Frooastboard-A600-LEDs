extern crate hidapi;

use hidapi::{DeviceInfo, HidApi, HidDevice};
use rand::{thread_rng, Rng};
use std::{thread, time};

fn main() {
    let time_delay = time::Duration::from_millis(80);

    let api = HidApi::new().unwrap();
    let mut devices: Vec<&DeviceInfo> = api
        .device_list()
        .filter(|device| {
            device.vendor_id().eq(&0x4642)
                && device.product_id().eq(&0xA600)
                && device.usage().eq(&0x0061)
                && device.usage_page().eq(&0xFF60)
        })
        .collect();

    if devices.is_empty() {
        panic!("no device connected!");
    } else if devices.len() > 1 {
        panic!("to many devices connected (> 1)");
    }

    let device_info = devices.remove(0);
    let device = device_info.open_device(&api).unwrap();
    println!(
        "keyboard name: {}",
        device.get_product_string().unwrap().unwrap()
    );

    //send_initialize(&device);

    for _ in 0..64 {
        let mut rng = thread_rng();
        send_update(
            &device,
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
            rng.gen_bool(0.5),
        );
        thread::sleep(time_delay);
    }

    send_update(&device, true, false, false);
}

#[derive(Clone, Copy)]
enum MessageType {
    //Initialize = 0xA4,
    Update = 0xA6,
}

/*fn send_initialize(device: &HidDevice) {
    let content: [u8; 16] = [0; 16];
    send_raw_message(device, MessageType::Initialize, &content);
}*/

fn send_update(device: &HidDevice, first_led: bool, second_led: bool, third_led: bool) {
    let mut content: [u8; 16] = [0; 16];
    content[0] = if first_led { 0x01 } else { 0x00 };
    content[1] = if second_led { 0x01 } else { 0x00 };
    content[2] = if third_led { 0x01 } else { 0x00 };
    send_raw_message(device, MessageType::Update, &content);
}

fn send_raw_message(device: &HidDevice, message_type: MessageType, content: &[u8; 16]) {
    let mut message: [u8; 33] = [0; 33];
    message[0] = 0x00;
    message[1] = message_type as u8;
    message[2..18].copy_from_slice(content);
    device.write(&message).unwrap();
    let mut response: [u8; 32] = [0; 32];
    device.read_timeout(&mut response, 1000).unwrap();
    assert_eq!(response[0], message_type as u8);
    assert_eq!(response[1], 0x00);
}
