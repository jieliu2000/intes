use std::{cell::RefCell, rc::Rc};

use fltk::{
    app::{self, MouseButton, WidgetId},
    button::{self},
    draw::{self},
    enums::{self, Event},
    frame,
    group::{self, Flex, Tabs},
    input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
    widget::{self, Widget},
    widget_extends,
    window::Window,
};
use fltk_accesskit::{AccessibilityContext, Accessible, AccessibleApp};
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
    button: String,
}
impl MouseTestArea {
    // our constructor
    pub fn new(
        label: &str,
        width: i32,
        height: i32,
        mouse_action_context: Rc<RefCell<MouseKeyboardActionContext>>,
    ) -> Self {
        let mut inner = widget::Widget::default()
            .with_size(width - 40, height)
            .with_id("mouse_test_canvas")
            .with_label(label);

        inner.set_frame(enums::FrameType::DownBox);

        inner.draw(|i| {
            // we need a draw implementation
            draw::draw_box(i.frame(), i.x(), i.y(), i.w(), i.h(), i.color());
            draw::set_draw_color(enums::Color::Black); // for the text
            draw::set_font(enums::Font::Helvetica, app::font_size());
            draw::draw_text2(&i.label(), i.x(), i.y(), i.w(), i.h(), i.align());
        });
        inner.handle(move |_i, ev| match ev {
            enums::Event::Push => {
                let button: MouseButton = app::event_mouse_button();
                match button {
                    MouseButton::Left => {
                        mouse_action_context.borrow_mut().button = "Left".to_string();
                    }
                    MouseButton::Middle => {
                        mouse_action_context.borrow_mut().button = "Middle".to_string();
                    }
                    MouseButton::Right => {
                        mouse_action_context.borrow_mut().button = "Right".to_string();
                    }
                    _ => {
                        mouse_action_context.borrow_mut().button = "".to_string();
                    }
                }
                app::handle_main(IntesEvent::MOUSE_DOWN).unwrap();
                true
            }
            enums::Event::Released => {
                app::handle_main(IntesEvent::MOUSE_UP).unwrap();
                true
            }
            enums::Event::Enter => {
                app::handle_main(IntesEvent::MOUSE_IN).unwrap();
                true
            }
            enums::Event::Leave => {
                app::handle_main(IntesEvent::MOUSE_OUT).unwrap();
                true
            }
            enums::Event::Move => {
                let coords = app::event_coords();
                mouse_action_context.borrow_mut().mouse_x = coords.0;
                mouse_action_context.borrow_mut().mouse_y = coords.1;
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

fn draw_tabs() -> Vec<Box<dyn Accessible>> {
    let mut tab = Tabs::default_fill().center_of_parent();
    let mut controls = Vec::new();

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
        mouse_x: 0,
        mouse_y: 0,
        button: "".to_string(),
    }));

    let part_definitions: [(
        i32,
        fn(&group::Pack, Rc<RefCell<MouseKeyboardActionContext>>) -> Vec<Box<dyn Accessible>>,
    ); 4] = [
        (30, add_mouse_test_tab_first_line),
        (100, add_mouse_test_tab_second_line),
        (30, add_mouse_tab_mouse_action_row),
        (30, add_mouse_tab_mouse_info_row),
    ];

    for (height, part_definition) in part_definitions {
        controls.append(&mut add_controls_hpack(
            &mouse_test_vpack,
            height,
            part_definition,
            &mouse_test_vpack,
            Rc::clone(&mouse_handler),
        ));
    }
    mouse_test_vpack.end();
    mouse_test_flex.end();

    let keyboard_test_flex = Flex::default_fill().with_label("Keyboard Test\t\t").row();
    keyboard_test_flex.end();
    tab.end();
    tab.auto_layout();
    return controls;
}

fn add_mouse_test_tab_first_line(
    _pack: &group::Pack,
    _event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) -> Vec<Box<dyn Accessible>> {
    let mut button1 = button::Button::default()
        .with_id("buttonA")
        .with_size(70, 0)
        .with_label("Button A");

    button1.set_callback(|_| {
        app::handle_main(IntesEvent::BUTTON1_CLICK).unwrap();
    });
    let mut button2 = button::Button::default()
        .with_id("buttonB")
        .with_size(70, 0)
        .with_label("Button B");

    button2.set_callback(|_| {
        app::handle_main(IntesEvent::BUTTON2_CLICK).unwrap();
    });

    return vec![Box::new(button1), Box::new(button2)];
}

fn add_mouse_tab_mouse_action_row(
    _pack: &group::Pack,
    _event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) -> Vec<Box<dyn Accessible>> {
    let label = frame::Frame::default()
        .with_size(90, 0)
        .with_label("Mouse action:");
    let mut input = input::IntInput::default().with_size(200, 0);
    input.handle(move |i, ev| match ev {
        IntesEvent::BUTTON1_CLICK => {
            i.set_value("Button A clicked");
            true
        }
        IntesEvent::BUTTON2_CLICK => {
            i.set_value("Button B clicked");
            true
        }
        IntesEvent::MOUSE_UP => {
            i.set_value("Mouse up");
            false
        }
        IntesEvent::MOUSE_DOWN => {
            i.set_value("Mouse down");
            false
        }
        IntesEvent::MOUSE_OUT => {
            i.set_value("");
            false
        }
        IntesEvent::MOUSE_MOVE => {
            i.set_value("Mouse move");
            false
        }
        _ => false,
    });

    input.set_readonly(true);
    return vec![Box::new(label), Box::new(input)];
}

fn add_mouse_tab_mouse_info_row(
    _pack: &group::Pack,
    event_context: Rc<RefCell<MouseKeyboardActionContext>>,
) -> Vec<Box<dyn Accessible>> {
    let label = frame::Frame::default()
        .with_size(120, 0)
        .with_label("Mouse information:");
    let mut input = input::IntInput::default().with_size(200, 0);
    input.handle(move |i, ev| match ev {
        IntesEvent::MOUSE_UP => {
            let mouse_info = format!(
                "X: {}, y: {}, button: {}",
                event_context.borrow().mouse_x,
                event_context.borrow().mouse_y,
                event_context.borrow().button
            );
            i.set_value(&mouse_info);
            true
        }
        IntesEvent::MOUSE_DOWN => {
            let mouse_info = format!(
                "X: {}, y: {}, button: {}",
                event_context.borrow().mouse_x,
                event_context.borrow().mouse_y,
                event_context.borrow().button
            );
            i.set_value(&mouse_info);
            true
        }
        IntesEvent::MOUSE_MOVE => {
            let mouse_info = format!(
                "X: {}, y: {}",
                event_context.borrow().mouse_x,
                event_context.borrow().mouse_y
            );
            i.set_value(&mouse_info);
            true
        }
        _ => {
            i.set_value("");
            false
        }
    });
    input.set_readonly(true);
    return vec![Box::new(label), Box::new(input)];
}

fn add_mouse_test_tab_second_line(
    pack: &group::Pack,
    event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) -> Vec<Box<dyn Accessible>> {
    let mut btn: MouseTestArea =
        MouseTestArea::new("Mouse Test Canvas", pack.width(), 100, event_handler);
    // notice that set_color and set_callback are automatically implemented for us!
    btn.set_color(enums::Color::Cyan);
    btn.set_callback(|_| println!("Clicked"));

    return vec![];
}

fn add_controls_hpack(
    grp1: &group::Pack,
    height: i32,
    add_components: fn(
        &group::Pack,
        Rc<RefCell<MouseKeyboardActionContext>>,
    ) -> Vec<Box<dyn Accessible>>,
    pack: &group::Pack,
    event_handler: Rc<RefCell<MouseKeyboardActionContext>>,
) -> Vec<Box<dyn Accessible>> {
    let mut hpack = group::Pack::default()
        .with_size(grp1.width(), height)
        .center_of(grp1);

    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(10);

    let components = add_components(pack, event_handler);

    hpack.end();
    return components;
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply(); // Apply the theme to all widgets

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("INTES: A GUI testing application")
        .center_screen();

    let controls = draw_tabs();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let ac = AccessibilityContext::new(wind, controls);

    app.run_with_accessibility(ac).unwrap();
}
