#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use test_engine::{
    App,
    refs::{Own, Weak},
    ui::{Label, Setup, View, ViewData, view},
};

#[view]
struct MobileView {
    #[init]
    label: Label,
}

impl Setup for MobileView {
    fn setup(self: Weak<Self>) {
        self.label.set_text("Hello").place().back();
    }
}

#[derive(Default)]
struct MobileApp {}

impl App for MobileApp {
    fn make_root_view(&self) -> Own<dyn View> {
        MobileView::new()
    }
}

test_engine::register_app!(MobileApp);
