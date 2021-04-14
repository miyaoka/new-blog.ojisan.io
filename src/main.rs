use pulldown_cmark::{html, Parser};
use std::{fs, io::Read, io::Write};
use std::{
    fs::{File, OpenOptions},
    path::Path,
};
use tera::{Context, Tera};

fn main() {
    let tera = match Tera::new("src/templates/*.html") {
        Ok(mut t) => {
            t.autoescape_on(vec![]); // html そのものを埋め込みたいから escape しない。
            t
        },
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();

    let markdown_str = r#"# Hello
人間は愚かな生物。

[俺のブログ](https://blog.himanoa.net)
"#;
    let parser = Parser::new(markdown_str);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("{}", html_buf);

    // 実行位置からの相対パス
    let dir = fs::read_dir("./src/contents");

    // https://doc.rust-jp.rs/rust-by-example-ja/std_misc/fs.html
    match dir {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let p = path.unwrap().path();
                let s = p.to_str().unwrap();
                let article_path = s.to_string() + "/index.md";
                let a_p = Path::new(&article_path);
                let mut f = File::open(a_p).unwrap();
                let mut s = String::new();
                f.read_to_string(&mut s);
                let parser = Parser::new(&s);
                let mut html_buf = String::new();
                html::push_html(&mut html_buf, parser);
                context.insert("content", &html_buf);
                let rendered = tera.render("post.html", &context);
                match rendered {
                    Ok(render) => {
                        let mut file = fs::File::create("foo.html").unwrap();
                        file.write_all(render.as_bytes()).unwrap();
                        println!("{:?}", render);
                    },
                    Err(why) => {
                        println!("{:?}", why)
                    }
                }
            }
        }
    }
}
