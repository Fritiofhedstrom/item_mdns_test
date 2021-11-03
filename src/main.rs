use std::{num::IntErrorKind, thread::sleep, time::Duration};
use log::{error, trace};
use libmdns::{Responder, Service};

extern crate libmdns;
struct BrokerAnnounce {
    service: Option<Service>,
}
impl BrokerAnnounce {
    fn new() -> Self {
        trace!("New BrokerAnnounce");
        BrokerAnnounce { service: None }
    }
    fn start(&mut self) {
        let responder = Responder::new().expect("noBueno");
        self.service = Some(responder.register(
            "_mqtt._tcp".into(),
            "item broker0".into(),
            1883,
            &["finger_print=134543kjkld43j4klt345923jklfn434tnujgkrnleu5h35ngjrke"],
        ));
    }
    fn stop(&mut self) {
        self.service = None;

    }
}

enum BrokerState {
    Leader,
    NotLeader,
}
// struct ItemBroker{
//     observable_state: SharedBehaviorSubject<BrokerState>
// }

// impl ItemBroker {
//     fn new() -> Self{
//         let me = ItemBroker{
//             observable_state,
//         }
//     }
// }



pub fn main() {
    println!("MAIN()");
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("trace");
    builder.init();

    error!("main()");
    if cfg!(target_os = "linux") {
        trace!("linux");
    }else{
        trace!("not linux");
    }
    let mut broker_announce = BrokerAnnounce::new();
    broker_announce.start();
    loop {
        sleep(Duration::from_secs(15));

    }

    broker_announce.stop();
    println!("Stopped");
    sleep(Duration::from_secs(20));

}


