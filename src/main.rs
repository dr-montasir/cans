use cans::html::{do_html, do_text};

pub const HEAD: &str = r#"<head>
 <meta charset="UTF-8">
     <title>{{page_title}} Page</title>
 </head>"#;

pub const HOME_TEMPLATE: &str = r#"<!DOCTYPE html>
 <html>
   {{HEAD}}
   <body>
      Home Page
   </body>
 </html>"#;

pub fn do_home_page() -> String {
    do_html!(HOME_TEMPLATE, HEAD = HEAD, page_title = do_text("Home"))
}

pub const ABOUT_TEMPLATE: &str = r#"<!DOCTYPE html>
 <html>
   {{HEAD}}
   <body>
      <p>About Page</p>
      {{component_if}}
   </body>
 </html>"#;

pub fn do_about_page() -> String {
    let component_if: &str;
    let x = 3;

    if x == 1 {
        component_if = "<a href='#'><i>x = 1</i></a>";
    } else if x == 2 {
        component_if = r#"<a href='#'><i>x = {{x}}</i></a>"#;
    } else {
        component_if = "<a href=\"#\"><i>x ≠ 1 & x  ≠ 2. The 'x' value is ( {{x}} )</i></a>";
    };

    do_html!(
        ABOUT_TEMPLATE,
        HEAD = HEAD,
        page_title = do_text("About"),
        component_if = component_if,
        x = x // x must be defined after component_if.
    )
}

fn main() {
    let home_page = do_home_page();
    println!("{}", home_page);

    let about_page = do_about_page();
    println!("{}", about_page);
}
