use fltk::{*, app, enums::FrameType, frame::Frame, image::PngImage, prelude::*, window::Window, button::Button};
use rand::Rng;
use std::str::FromStr;

pub const MEMBERCOUNT: u32 = 6;
pub const QUESTIONCOUNT: u32 = 7;

pub fn find_displaytype(usergender: String, genderchoice: String) -> u32{
    let dtype:u32;
    if usergender.eq_ignore_ascii_case("f")
    {
        if genderchoice.eq_ignore_ascii_case("m")
        {dtype=2;}
        else{dtype=4;}
    }
    else{
        if genderchoice.eq_ignore_ascii_case("m")
        {dtype=3;}
        else{dtype=1;}}

    dtype
}

pub struct Answerstring{
   pub a1:u32,
   pub a2:u32,   
   pub a3:u32,   
   pub a4:u32,   
   pub a5:u32,   
   pub a6:u32,   
   pub a7:u32,
}

pub struct Profile{
   pub name: String,
   pub age: u32,
   pub gender: String,
   pub  genderchoice: String,
   pub  pfp: PngImage,
   pub displaytype: u32,
}
impl Profile{

pub fn new()
{
let app = app::App::default();
let mut wind = Window::new(500, 200, 700, 400, "RustyDating");

let mut frame = Frame::default().with_size(700, 390).center_of(&wind);
let mut bar = frame::Frame::new(0, 0, 700, 40, " Rusty Dating: Dating for beginners!")
.with_align(enums::Align::Left | enums::Align::Inside);
let mut image = PngImage::load("pic/date.png").unwrap();
image.scale(700, 400, true, true);
frame.set_image(Some(image));


bar.set_frame(enums::FrameType::FlatBox);
bar.set_label_size(22);
bar.set_label_color(enums::Color::White);
bar.set_label_font(enums::Font::ScreenBold);
bar.set_color(enums::Color::from_u32(0xCB5C0D));
bar.draw(|b| {
    draw::set_draw_rgb_color(211, 211, 211);
    draw::draw_rectf(0, b.height(), b.width(), 3);
});

let mut nam = input::Input::new(210,45,100,30,"What's your name?");
nam.set_text_color(enums::Color::from_u32(0xCB5C0D));
nam.set_label_size(15);
nam.set_label_color(enums::Color::White);
nam.set_label_font(enums::Font::ScreenBold);
nam.set_label_type(enums::LabelType::Shadow);

let mut ag = input::Input::new(210,145,100,30,"Age?");
ag.set_text_color(enums::Color::from_u32(0xCB5C0D));
ag.set_label_size(15);
ag.set_label_color(enums::Color::White);
ag.set_label_font(enums::Font::ScreenBold);
ag.set_label_type(enums::LabelType::Shadow);

let mut gend = input::Input::new(210,245,100,30,"Gender? (F/M)");
gend.set_text_color(enums::Color::from_u32(0xCB5C0D));
gend.set_label_size(15);
gend.set_label_color(enums::Color::White);
gend.set_label_font(enums::Font::ScreenBold);
gend.set_label_type(enums::LabelType::Shadow);


let mut genderc = input::Input::new(210,345,100,30,"Gender LF? (F/M)");
genderc.set_text_color(enums::Color::from_u32(0xCB5C0D));
genderc.set_label_size(15);
genderc.set_label_color(enums::Color::White);
genderc.set_label_font(enums::Font::ScreenBold);
genderc.set_label_type(enums::LabelType::Shadow);

let mut but = Button::new(410, 210, 80, 80, "Enter");
but.set_frame(enums::FrameType::RShadowBox);
but.set_color(enums::Color::from_u32(0xCB5C0D));

but.set_selection_color(enums::Color::from_u32(0xFFA500));
but.set_label_color(enums::Color::White);



but.set_callback(move |_| {

let na=nam.value().to_string();
let a:u32=FromStr::from_str(&ag.value().to_string()).unwrap();
let gen=gend.value().to_string();
let genc = genderc.value().to_string();

let dt = find_displaytype(gend.value().to_string(), genderc.value().to_string());

Self{
         name: nam.value().to_string(),
         age: FromStr::from_str(&ag.value().to_string()).unwrap(),
         gender: gend.value().to_string(),
         genderchoice: genderc.value().to_string(),
         pfp: PngImage::load("pic/date.png").unwrap(),
         displaytype: dt,
    };
    nam.hide();
    ag.hide();
    gend.hide();
    genderc.hide();
    bar.set_label(&(na + &" , ".to_owned() + &a.to_string() + &" , ".to_owned() + &gen + &" , LF: ".to_owned() + &genc));    });
wind.end();
wind.show();

app.run().unwrap();

}
}

impl Answerstring{

pub fn new(){
let mut rng = rand::thread_rng();
Self{
a1: rng.gen::<u32>()%2,
a2: rng.gen::<u32>()%2,
a3: rng.gen::<u32>()%2,
a4: rng.gen::<u32>()%2,
a5: rng.gen::<u32>()%2,
a6: rng.gen::<u32>()%2,
a7: rng.gen::<u32>()%2,
};
}

pub fn fill_answerset(){

let app = app::App::default();
let mut wind = Window::new(500, 200, 700, 400, "Some answers from you");

let mut frame = Frame::default().with_size(700, 390).center_of(&wind);
let mut bar = frame::Frame::new(0, 0, 700, 40, " Let's learn more about you. Answer 1 or 0")
    .with_align(enums::Align::Left | enums::Align::Inside);
let mut image = PngImage::load("pic/date.png").unwrap();
image.scale(700, 400, true, true);
frame.set_image(Some(image));


bar.set_frame(enums::FrameType::FlatBox);
bar.set_label_size(22);
bar.set_label_color(enums::Color::White);
bar.set_label_font(enums::Font::ScreenBold);
bar.set_color(enums::Color::from_u32(0xCB5C0D));
bar.draw(|b| {
    draw::set_draw_rgb_color(211, 211, 211);
    draw::draw_rectf(0, b.height(), b.width(), 3);
});

let mut q1 = input::Input::new(310,70,100,30,"Outdoors or Indoors (1/0)");
q1.set_text_color(enums::Color::from_u32(0xCB5C0D));
q1.set_label_size(15);
q1.set_label_color(enums::Color::White);
q1.set_label_font(enums::Font::ScreenBold);
q1.set_label_type(enums::LabelType::Shadow);
let mut q2 = input::Input::new(310,120,100,30,"Classical or Rap (1/0)");
q2.set_text_color(enums::Color::from_u32(0xCB5C0D));
q2.set_label_size(15);
q2.set_label_color(enums::Color::White);
q2.set_label_font(enums::Font::ScreenBold);
q2.set_label_type(enums::LabelType::Shadow);

let mut q3 = input::Input::new(310,170,100,30,"Basketball or Tennis (1/0)");
q3.set_text_color(enums::Color::from_u32(0xCB5C0D));
q3.set_label_size(15);
q3.set_label_color(enums::Color::White);
q3.set_label_font(enums::Font::ScreenBold);
q3.set_label_type(enums::LabelType::Shadow);

let mut q4 = input::Input::new(310,200,100,30,"Anime or Cartoons (1/0)");
q4.set_text_color(enums::Color::from_u32(0xCB5C0D));
q4.set_label_size(15);
q4.set_label_color(enums::Color::White);
q4.set_label_font(enums::Font::ScreenBold);
q4.set_label_type(enums::LabelType::Shadow);

let mut q5 = input::Input::new(310,270,100,30,"Rust or Fresh Steel (1/0)");
q5.set_text_color(enums::Color::from_u32(0xCB5C0D));
q5.set_label_size(15);
q5.set_label_color(enums::Color::White);
q5.set_label_font(enums::Font::ScreenBold);
q5.set_label_type(enums::LabelType::Shadow);

let mut q6 = input::Input::new(310,320,100,30,"Heads or Tails (1/0)");
q6.set_text_color(enums::Color::from_u32(0xCB5C0D));
q6.set_label_size(15);
q6.set_label_color(enums::Color::White);
q6.set_label_font(enums::Font::ScreenBold);
q6.set_label_type(enums::LabelType::Shadow);

let mut q7 = input::Input::new(310,370,100,30,"Right now or Forever (1/0)");
q7.set_text_color(enums::Color::from_u32(0xCB5C0D));
q7.set_label_size(15);
q7.set_label_color(enums::Color::White);
q7.set_label_font(enums::Font::ScreenBold);
q7.set_label_type(enums::LabelType::Shadow);


let mut but = Button::new(410, 210, 80, 80, "Enter");
but.set_frame(enums::FrameType::RShadowBox);
but.set_color(enums::Color::from_u32(0xCB5C0D));

but.set_selection_color(enums::Color::from_u32(0xFFA500));
but.set_label_color(enums::Color::White);

but.set_callback(move |_| {
Self{
a1: FromStr::from_str(&q1.value().to_string()).unwrap(),
a2: FromStr::from_str(&q2.value().to_string()).unwrap(),
a3: FromStr::from_str(&q3.value().to_string()).unwrap(),
a4: FromStr::from_str(&q4.value().to_string()).unwrap(),
a5: FromStr::from_str(&q5.value().to_string()).unwrap(),
a6: FromStr::from_str(&q6.value().to_string()).unwrap(),
a7: FromStr::from_str(&q7.value().to_string()).unwrap(),
};
});

wind.end();
wind.show();

app.run().unwrap();
}

}
