
use druid::{text::FontDescriptor, theme, widget::{Button, Container, Flex, Label}, AppLauncher, Color, Data, EventCtx, Lens, LocalizedString, Selector, Size, Widget, WidgetExt, WidgetId, WindowDesc};
use rhai::eval;

const UPDATE_SCREEN: Selector<String> = Selector::new("update_screen");

#[derive(Clone,Data,Lens,Debug)]
struct Calculator{
    screen:String,
}

impl Calculator{
    fn calc(&self) -> String{
        let screen:&str = &self.screen;
        let result:i64 = match eval(screen){
            Ok(val)=>val,
            Err(e)=>{
                println!("{e}");
                0
            }
        };

        println!("{}",result);
        result.to_string()
    }

    fn add(&mut self,val:&str){
        let screen: Vec<char> = self.screen.chars().collect();
        if &self.screen == "0"{
            self.screen = String::new();
        }
        match screen.last() {
            Some(el)=>{
                if val == "+" ||val == "-" ||val == "/" ||val == "*" ||val == "%"{
                    let last = el.to_string();

                    if last != "+" && last != "-" && last != "/" && last != "*" &&last != "%"{
                        self.screen.push_str(val);
                    }
                } else {
                    self.screen.push_str(val);
                }
            }
            None=>{println!("Not")}
        }
    }

    fn clean(&mut self){
        self.screen = String::from("0");
    }
}

struct MyTheme {
    button_color: Color,
    // Добавьте здесь другие цвета и параметры вашей темы
}

impl MyTheme {
    fn new(r:u8,g:u8,b:u8) -> Self {
        Self {
            button_color: Color::rgb8(r, g, b), // Красный цвет кнопки
            // Инициализируйте здесь другие цвета и параметры вашей темы
        }
    }
}


fn main() {
    let  intst = Calculator{
        screen:String::from("0")
    };

    let up = MyTheme::new(200,69,233);
    let down = MyTheme::new(242,81,81);

    let main_window = WindowDesc::new(build_ui()).title(LocalizedString::new("Calculator")).window_size(Size::new(280.0,330.0)).resizable(false);



    AppLauncher::with_window(main_window).configure_env(move|env,_|{
        env.set(theme::BUTTON_LIGHT,up.button_color);
        env.set(theme::BUTTON_DARK,down.button_color);
        env.set(theme::BACKGROUND_LIGHT,Color::rgb8(81, 242, 204));
        env.set(theme::BACKGROUND_DARK,Color::rgb8(81, 242, 204));
        env.set(theme::BORDER_LIGHT,Color::rgb8(81, 242, 204));
        env.set(theme::WINDOW_BACKGROUND_COLOR,Color::rgb8(54, 54, 54));
    }).launch(intst).expect("Failed to run");
}



fn build_ui() -> impl Widget<Calculator>{
    let bold_font = FontDescriptor::new(druid::FontFamily::SYSTEM_UI);
    let  screen = Label::dynamic(|data:&Calculator,_env:&_| format!("{}",data.screen)).with_id(WidgetId::next()).env_scope(move|env,_|{
        env.set(theme::TEXT_COLOR,Color::rgb8(255, 255, 255));
        env.set(theme::TEXT_SIZE_LARGE,60.0);
        env.set(theme::UI_FONT_BOLD,bold_font.clone());
    });

    let equal = Button::new("=").on_click(move|ctx:&mut EventCtx, data:&mut Calculator, _|{
        data.screen = data.calc();
        // Отправляем команду на обновление экрана
        ctx.submit_command(UPDATE_SCREEN.with(data.screen.clone()));
    });

    let C = Button::new("C").on_click(|_,data:&mut Calculator,_|{
        data.clean();
    });

    let seven = Button::new("7").on_click(|_,data:&mut Calculator,_|{
        data.add("7");
    });

    let eight = Button::new("8").on_click(|_,data:&mut Calculator,_|{
        data.add("8");
    });

    let nine = Button::new("9").on_click(|_,data:&mut Calculator,_|{
        data.add("9");
    });

    let four = Button::new("4").on_click(|_,data:&mut Calculator,_|{
        data.add("4");
    });

    let five = Button::new("5").on_click(|_,data:&mut Calculator,_|{
        data.add("5");
    });

    let six = Button::new("6").on_click(|_,data:&mut Calculator,_|{
        data.add("6");
    });

    let one = Button::new("1").on_click(|_,data:&mut Calculator,_|{
        data.add("1");
    });

    let two = Button::new("2").on_click(|_,data:&mut Calculator,_|{
        data.add("2");
    });

    let three = Button::new("3").on_click(|_,data:&mut Calculator,_|{
        data.add("3");
    });

    let zero = Button::new("0").on_click(|_,data:&mut Calculator,_|{
        data.add("0");
    });



    let plus = Button::new("+").on_click(|_,data:&mut Calculator,_|{
        data.add("+");
    });

    let div = Button::new("-").on_click(|_,data:&mut Calculator,_|{
        data.add("-");
    });

    let mult = Button::new("x").on_click(|_,data:&mut Calculator,_|{
        data.add("*");
    });

    let subdiv = Button::new("÷").on_click(|_,data:&mut Calculator,_|{
        data.add("/");
    });


    let layout = Flex::column()
        .with_flex_child(screen.fix_size(60.0, 60.0),1.0)
        .with_flex_child(Flex::row()
                    .with_flex_child(seven.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(eight.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(nine.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(subdiv.fix_size(60.0, 60.0),1.0),1.0
    )
        .with_flex_child(Flex::row()
                    .with_flex_child(four.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(five.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(six.fix_size(60.0, 60.0),1.0)
                    .with_flex_child(mult.fix_size(60.0, 60.0),1.0),1.0
    ).with_flex_child(Flex::row()
                .with_flex_child(one.fix_size(60.0, 60.0),1.0)
                .with_flex_child(two.fix_size(60.0, 60.0),1.0)
                .with_flex_child(three.fix_size(60.0, 60.0),1.0)
                .with_flex_child(div.fix_size(60.0, 60.0),1.0),1.0
    ).with_flex_child(Flex::row()
                .with_flex_child(zero.fix_size(60.0, 60.0),1.0)
                .with_flex_child(C.fix_size(60.0, 60.0),1.0)
                .with_flex_child(equal.fix_size(60.0, 60.0),1.0)
                .with_flex_child(plus.fix_size(60.0, 60.0),1.0),1.0
).expand_height();

    Container::new(layout)
        .padding(20.0)
        .expand_width()
        .expand_height()
        .center()
}