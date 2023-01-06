#[macro_use]
extern crate rocket;

use std::fs;
use neofiglet::FIGfont;
use rocket::response::content::RawHtml;

#[derive(Debug, PartialEq, FromForm)]
struct Options<'r> {
    font: &'r str,
}

#[get("/", rank = 3)]
fn index() -> RawHtml<String> {
    let html = fs::read_to_string("src/index.html").unwrap();
    let resp = html.to_string();

    return RawHtml(resp);
}

#[get("/?<text>", rank = 2)]
fn fig_rs(text: &str) -> String {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert(text);

    return figure.unwrap().to_string();
}

#[get("/?<text>&<options>")]
fn fig_rs_with_font(text: &str, options: Options<'_>) -> String {
    let mut font_str = String::from("fonts/");
    font_str.push_str(options.font);
    font_str.push_str(".flf");

    let font = match FIGfont::from_file(font_str.as_str()) {
        Ok(font) => font,
        Err(e) => return e.to_string(),
    };

    let figure = font.convert(text);

    return figure.unwrap().to_string();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, fig_rs, fig_rs_with_font])
}