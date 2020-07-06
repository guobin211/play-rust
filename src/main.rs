use web_view::*;

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn main() {
    let html_content = format!(
        r#"
        <!DOCTYPE html>
            <html lang="zh-CH">
            <head>
                <meta charset="UTF-8">
                <title>WebApp</title>
                {styles}
            </head>
            <body>
            <div id="app">
            </div>
            {scripts}
            </body>
            </html>
        "#,
        styles = inline_style(include_str!("../web/dist/app.cd080b21.css")),
        scripts = inline_script(include_str!("../web/dist/main.172c23a0.js"))
    );

    let window= web_view::builder()
        .title("WebApp")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
