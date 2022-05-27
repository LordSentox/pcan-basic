use pcan_basic::bus::UsbBus;
use pcan_basic::df::SetAcceptanceFilter29Bit;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::CanRead;

fn main() {
    let usb_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(socket) => socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match usb_socket.set_acceptance_filter_29bit(&[0x1_FF, 0x3_FF]) {
        Ok(_) => {}
        Err(_) => return,
    }

    loop {
        let can_frame = usb_socket.read();
        match can_frame {
            Ok((frame, timestamp)) => {
                println!("{:?}", frame);
                println!("{:?}", timestamp);
            }
            Err(_) => {}
        }
    }
}
