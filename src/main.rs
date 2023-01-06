#[macro_use] extern crate rocket;

use neofiglet::FIGfont;

#[derive(Debug, PartialEq, FromForm)]
struct Options<'r> {
    font: &'r str
}

#[get("/?<text>", rank=2)]
fn index(text: &str ) -> String {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert(text);

    return figure.unwrap().to_string();
}

#[get("/?<text>&<options>")]
fn index_with_font(text: &str, options: Options<'_>) -> String {
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
    rocket::build().mount("/", routes![index, index_with_font])
}