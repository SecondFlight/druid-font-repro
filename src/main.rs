use druid::piet::Text;
use druid::widget::*;
use druid::*;

pub const FONT: &[u8] = include_bytes!("NotoSansMono-Regular.ttf");
pub const FONT_KEY: Key<FontDescriptor> = Key::new("fonts.NotoSansMono-Regular");

struct MainWindow {
    content: WidgetPod<(), Box<dyn Widget<()>>>,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            content: WidgetPod::new(Box::new(SizedBox::empty())),
        }
    }
}

impl Widget<()> for MainWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (), env: &Env) {
        self.content.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(), env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            let mono_regular_family = ctx.text().load_font(FONT).unwrap();

            self.content = WidgetPod::new(Box::new(Padding::new(3., Child::new()).env_scope(
                move |env, _data| {
                    let mono_regular = FontDescriptor::new(mono_regular_family.clone())
                        .with_weight(FontWeight::REGULAR);

                    env.set(FONT_KEY, mono_regular.clone());
                },
            )));
        }

        self.content.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), data: &(), env: &Env) {
        self.content.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(), env: &Env) -> Size {
        self.content.layout(ctx, bc, data, env);
        self.content.set_origin(ctx, data, env, Point::new(0., 0.));

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(), env: &Env) {
        let size = ctx.size();
        ctx.fill(size.to_rect(), &Color::rgba(1., 1., 1., 0.07));

        self.content.paint(ctx, data, env);
    }
}

struct Child {
    content: WidgetPod<(), Box<dyn Widget<()>>>,
}

impl Child {
    pub fn new() -> Child {
        Child {
            content: WidgetPod::new(Box::new(SizedBox::empty())),
        }
    }
}

impl Widget<()> for Child {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (), env: &Env) {
        self.content.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(), env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.content = WidgetPod::new(Box::new(
                Flex::column()
                    .with_child(Label::new("This does have a font").with_font(env.get(FONT_KEY).with_size(15.)))
                    .with_spacer(10.)
                    .with_child(Label::new("This does not have a font")),
            ));
            ctx.children_changed();
        }

        self.content.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(), data: &(), env: &Env) {
        self.content.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(), env: &Env) -> Size {
        self.content.layout(ctx, bc, data, env);
        self.content.set_origin(ctx, data, env, Point::new(0., 0.));

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(), env: &Env) {
        self.content.paint(ctx, data, env);
    }
}

fn make_ui() -> impl Widget<()> {
    MainWindow::new()
}

fn main() {
    let main_window = WindowDesc::new(make_ui()).window_size((800., 600.));
    AppLauncher::with_window(main_window)
        .use_env_tracing()
        .launch(())
        .expect("Failed to start");
}
