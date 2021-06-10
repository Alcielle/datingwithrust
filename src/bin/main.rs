use lib::*;
use fltk::{*, app, enums::FrameType, frame::Frame, image::PngImage, prelude::*, window::Window, button::Button};
use rand::Rng;

pub const MEMBERCOUNT: u32 = 6;
pub const QUESTIONCOUNT: u32 = 7;

pub fn find_match(userstring: Answerstring, matchstring: Answerstring) -> u32 {

    let  mut matchp:u32 = 0;
    
    if userstring.a1==matchstring.a1 {matchp=matchp+1;}
    if userstring.a2==matchstring.a2 {matchp=matchp+1;}
    if userstring.a3==matchstring.a3 {matchp=matchp+1;}
    if userstring.a4==matchstring.a4 {matchp=matchp+1;}
    if userstring.a5==matchstring.a5 {matchp=matchp+1;}
    if userstring.a6==matchstring.a6 {matchp=matchp+1;}
    if userstring.a7==matchstring.a7 {matchp=matchp+1;}

    matchp=matchp*100;
    matchp=matchp/QUESTIONCOUNT;

    matchp
}




fn main() {

let mstring = "M";
let fstring = "F";

let mut sampleanswers: Vec<Answerstring>= Vec::new();
let mut sampleprofiles: Vec<Profile> = Vec::new();


let p1 = Profile{
    name: String::from("Jane"),
    age: 31,
    gender: fstring.to_string(),
    genderchoice: mstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 2,
};

let as1 = Answerstring::new();
sampleprofiles.push(p1);

let p2 = Profile{
    name: String::from("John"),
    age: 37,
    gender: mstring.to_string(),
    genderchoice: fstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 1,
};

let as2 = Answerstring::new();
sampleprofiles.push(p2);

let p3 = Profile{
    name: String::from("Jake"),
    age: 21,
    gender: mstring.to_string(),
    genderchoice: mstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 3,
};

let as3 = Answerstring::new();
sampleprofiles.push(p3);

let p4 = Profile{
    name: String::from("Jo"),
    age: 31,
    gender: fstring.to_string(),
    genderchoice: fstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 4,
};

let as4 = Answerstring::new();
sampleprofiles.push(p4);

let p5 = Profile{
    name: String::from("Jillian"),
    age: 25,
    gender: fstring.to_string(),
    genderchoice: fstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 4,
};

let as5 = Answerstring::new();
sampleprofiles.push(p5);

let p6 = Profile{
    name: String::from("Jack"),
    age: 23,
    gender: mstring.to_string(),
    genderchoice: mstring.to_string(),
    pfp: PngImage::load("pic/date.png").unwrap(),
    displaytype: 3,
};

let as6=Answerstring::new();
sampleprofiles.push(p6);

let puser=Profile::new();
let pastring=Answerstring::fill_answerset();



}
        




