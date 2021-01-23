
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
#[derive(Clone, Data, Lens)]
struct Stage {
    loggedIn: bool,
    email: String,
}

pub fn launch_gui() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("KLauncher")
        .window_size((400.0, 400.0));
    let initial_state = Stage {
        loggedIn: false,
        email: "".into()
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state).unwrap()
}

fn build_root_widget() -> impl Widget<Stage> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &Stage, _env: &Env| format!("Hello {}!", data.email));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(Stage::email);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}