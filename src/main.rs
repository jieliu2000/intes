use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use fltk::{
    app,
    button::{self, *},
    draw::{self, height},
    enums::{self, Event, FrameType},
    frame,
    group::{self, Flex, Tabs},
    input,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    text, widget, widget_extends,
    window::Window,
};
use fltk_theme::{ThemeType, WidgetTheme};

struct MouseTestArea {
    inner: widget::Widget,
}

struct IntesEvent;

impl IntesEvent {
    pub const MOUSE_MOVE: Event = Event::from_i32(60);
    const MOUSE_DOWN: Event = Event::from_i32(61);
    const MOUSE_UP: Event = Event::from_i32(62);
    const MOUSE_OUT: Event = Event::from_i32(63);
    const MOUSE_IN: Event = Event::from_i32(65);
    const BUTTON1_CLICK: Event = Event::from_i32(66);
    const BUTTON2_CLICK: Event = Event::from_i32(67);
}

struct MouseKeyboardActionContext {
    mouse_x: i32,
    mouse_y: i32,
    mouse_action: String,
    mouse_button: String,
}
impl MouseTestArea {
    // our constructor
    pub fn new(
        label: &str,
        width: i32,
        height: i32,
        mouse_action_handler: Rc<RefCell<MouseKeyboardActionContext>>,
    ) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(width - 40, height)
            .with_label(label);

        inner.set_frame(enums::FrameType::DownBox);

        inner.draw(|i| {
            // we need a draw implementation
            draw::draw_box(i.frame(), i.x(), i.y(), i.w(), i.h(), i.color());
            draw::set_draw_color(enums::Color::Black); // for the text
            draw::set_font(enums::Font::Helvetica, app::font_size());
            draw::draw_text2(&i.label(), i.x(), i.y(), i.w(), i.h(), i.align());
        });
        inner.handle(move |i, ev| match ev {
            enums::Event::Push => {
                app::handle_main(IntesEvent::MOUSE_DOWN).unwrap();
                true
            }
            enums::Event::Released => {
                app::handle_main(IntesEvent::MOUSE_UP).unwrap();
                true
            }
            enums::Event::Enter => {
                mouse_action_handler.borrow_mut().mouse_action = "".to_string();
                app::handle_main(IntesEvent::MOUSE_IN).unwrap();
                true
            }
            enums::Event::Leave => {
                mouse_action_handler.borrow_mut().mouse_action = "".to_string();
                app::handle_main(IntesEvent::MOUSE_OUT).unwrap();
                true
            }
            enums::Event::Move => {
                mouse_action_handler.borrow_mut().mouse_action = "Mouse Move".to_string();
                app::handle_main(IntesEvent::MOUSE_MOVE).unwrap();
                true
            }
            _ => false,
        });
        Self {
            inner,
            // we need to set the callbacks
        }
    }
}

// Extend widget::Widget via the member `inner` and add other initializers and constructors
widget_extends!(MouseTestArea, widget::Widget, inner);

fn draw_tabs() {
    let mut tab = Tabs::default_fill().center_of_parent();

    let client_area = tab.client_area();

    let mut mouse_test_flex = group::Flex::default_fill()
        .with_size(client_area.2, client_area.3)
        .center_of_parent()
        .with_label("Mouse Test");

    mouse_test_flex.set_type(group::FlexType::Column);
    mouse_test_flex.set_margin(20);

    let mut mouse_test_vpack = group::Pack::default_fill()
        .with_size(client_area.2, client_area.3)
        .center_of_parent();

    mouse_test_vpack.set_type(group::PackType::Vertical);
    mouse_test_vpack.set_spacing(30);

    let mouse_handler = Rc::new(RefCell::new(MouseKeyboardActionContext {
        mouse_action: String::new(),
        mouse_button: String::new(),
        mouse_x: 0,
        mouse_y: 0,
    }));

    add_controls_hpack(
        &mouse_test_vpack,
        30,
        add_mouse_test_tab_first_line,
        &mouse_test_vpack,
        Rc::clone(&mouse_handler),
    );
    add_controls_hpack(
        &mouse_test_vpack,
        100,
        add_mouse_test_tab_second_line,
        &mouse_test_vpack,
        Rc::clone(&mouse_handler),
    );
    add_controls_hpack(
        &mouse_test_vpack,
        30,
        add_mouse_tab_mouse_action_row,
        &mouse_test_vpack,
        Rc::clone(&mouse_handler),
    );

    add_controls_hpack(
        &mouse_test_vpack,
        30,
        add_mouse_tab_mouse_info_row,
        &mouse_test_vpack,
        Rc::clone(&mouse_handler),
    );
    mouse_test_vpack.end();
    mouse_test_flex.end();

    let keyboard_test_flex = Flex::default_fill().with_label("Keyboard Test\t\t").row();
    keyboard_test_flex.end();
    tab.end();
    tab.auto_layout();
}

fn add_mouse_test_tab_first_line(
    _pack: &group::Pack,
    _event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) {
    let mut button1 = button::Button::default()
        .with_size(70, 0)
        .with_label("Button1");

    button1.set_callback(|_| {
        app::handle_main(IntesEvent::BUTTON1_CLICK).unwrap();
    });
    let mut button2 = button::Button::default()
        .with_size(70, 0)
        .with_label("Button2");

    button2.set_callback(|_| {
        app::handle_main(IntesEvent::BUTTON2_CLICK).unwrap();
    });
}

fn add_mouse_tab_mouse_action_row(
    _pack: &group::Pack,
    _event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) {
    let _label = frame::Frame::default()
        .with_size(90, 0)
        .with_label("Mouse action:");
    let mut input = input::IntInput::default().with_size(200, 0);
    input.handle(move |i, ev| match ev {
        IntesEvent::BUTTON1_CLICK => {
            i.set_value("Button 1 clicked");
            true
        }
        IntesEvent::BUTTON2_CLICK => {
            i.set_value("Button 2 clicked");
            true
        }
        _ => false,
    });

    input.set_readonly(true)
}

fn add_mouse_tab_mouse_info_row(
    _pack: &group::Pack,
    _event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) {
    let _label = frame::Frame::default()
        .with_size(90, 0)
        .with_label("Mouse information:");
    let mut input = input::IntInput::default().with_size(200, 0);
    input.set_readonly(true);
    input.set_value("aaaaa");
}

fn add_mouse_test_tab_second_line(
    pack: &group::Pack,
    event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) {
    let mut btn = MouseTestArea::new("Mouse Test Canvas", pack.width(), 100, event_handler);
    // notice that set_color and set_callback are automatically implemented for us!
    btn.set_color(enums::Color::Cyan);
    btn.set_callback(|_| println!("Clicked"));
}

fn add_controls_hpack(
    grp1: &group::Pack,
    height: i32,
    add_components: fn(&group::Pack, Rc<RefCell<MouseKeyboardActionContext>>),
    pack: &group::Pack,
    event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) {
    let mut hpack = group::Pack::default()
        .with_size(grp1.width(), height)
        .center_of(grp1);

    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(30);

    add_components(pack, event_handler);

    hpack.end();
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply(); // Apply the theme to all widgets

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("INTES: A GUI testing application")
        .center_screen();

    draw_tabs();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
