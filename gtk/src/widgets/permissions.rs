use gtk::prelude::*;

#[derive(Shrinkwrap)]
pub struct PermissionDenied(gtk::Container);

impl PermissionDenied {
    pub fn new() -> Self {
        let container = cascade! {
            gtk::Box::new(gtk::Orientation::Horizontal, 24);
            ..set_halign(gtk::Align::Center);
            ..set_valign(gtk::Align::Center);
            ..add(
                &gtk::ImageBuilder::new()
                    .icon_name("system-lock-screen-symbolic")
                    .icon_size(gtk::IconSize::Dialog.into())
                    .pixel_size(64)
                    .build()
            );
            ..add(&cascade! {
                gtk::LabelBuilder::new()
                    .label("Permission Required\n\nOnly administrator accounts may upgrade the OS.")
                    .wrap(true)
                    .xalign(0.0)
                    .yalign(0.0)
                    .build();
                ..get_style_context().add_class(&gtk::STYLE_CLASS_DIM_LABEL);
            });
            ..show_all();
        };

        Self(container.upcast::<gtk::Container>())
    }
}
