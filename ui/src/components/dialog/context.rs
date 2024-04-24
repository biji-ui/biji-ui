use leptos::{
    html::{Button, Div},
    *,
};

#[derive(Copy, Clone)]
pub struct DialogContext {
    pub trigger_ref: NodeRef<Button>,
    pub open: RwSignal<bool>,
    pub root: RwSignal<RootContext>,
    pub prevent_scroll: bool,
}

impl Default for DialogContext {
    fn default() -> Self {
        Self {
            trigger_ref: NodeRef::default(),
            open: create_rw_signal(false),
            root: create_rw_signal(RootContext::default()),
            prevent_scroll: true,
        }
    }
}

impl DialogContext {
    pub fn open(&self) {
        self.root.set(RootContext::default());
        self.open.set(true);
    }

    pub fn close(&self) {
        self.open.set(false);
    }

    pub fn toggle(&self) {
        if self.open.get() {
            self.close();
        } else {
            self.open();
        }
    }
}

#[derive(Copy, Clone)]
pub struct RootContext {
    pub close_ref: NodeRef<Button>,
    pub overlay_ref: NodeRef<Div>,
}

impl Default for RootContext {
    fn default() -> Self {
        Self {
            overlay_ref: NodeRef::default(),
            close_ref: NodeRef::default(),
        }
    }
}
