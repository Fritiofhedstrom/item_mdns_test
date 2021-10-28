extern crate env_logger;
use std::{num::IntErrorKind, thread::sleep, time::Duration};

use env_logger::Builder;
use libmdns::{Responder, Service};
use rxrust::{behavior_subject::BehaviorSubject, observable, prelude::Observable};
extern crate mdns;

extern crate libmdns;
struct BrokerAnnounce {
    service: Option<Service>,
}
impl BrokerAnnounce {
    fn new() -> Self {
        BrokerAnnounce { service: None }
    }
    fn start(&mut self) {
        let responder = Responder::new().expect("noBueno");
        self.service = Some(responder.register(
            "_mqtt._tcp".into(),
            "item broker".into(),
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
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("libmdns=debug");
    builder.init();
    let mut broker_announce = BrokerAnnounce::new();
    broker_announce.start();
    loop {
        sleep(Duration::from_secs(15));
    }
    broker_announce.stop();
}

#[test]
fn empty_local_subject_can_convert_into_shared() {
    let pool = ThreadPool::new().unwrap();
    use std::sync::{Arc, Mutex};
    let value = Arc::new(Mutex::new(0));
    let c_v = value.clone();
    let subject = SharedBehaviorSubject::new(42);
    let mut subject_c = subject.clone();
    let stamp = Instant::now();
    pool.schedule(
        move |_| {
            subject_c.complete();
        },
        Some(Duration::from_millis(25)),
        (),
    );
    subject
        .clone()
        .into_shared()
        .observe_on(pool)
        .into_shared()
        .subscribe_blocking(move |v: i32| {
            *value.lock().unwrap() = v;
        });
    assert!(stamp.elapsed() > Duration::from_millis(25));
    assert_eq!(*c_v.lock().unwrap(), 42);
}
