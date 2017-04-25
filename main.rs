extern crate iron;
extern crate router;
extern crate liquid;

use std::error::Error;
use std::path::PathBuf;

use iron::prelude::*;
use iron::status;
use router::Router;

use liquid::{LiquidOptions, Renderable, Context, Value, Token};
use liquid::lexer::Element;

struct Paginate {}
impl Renderable for Paginate {
    fn render(&self, _context: &mut Context) -> Result<Option<String>, liquid::Error> {
        Ok(Some("PAGINATE HERE".to_string()))
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(_req: &mut Request) -> IronResult<Response> {
        let mut options = LiquidOptions::with_known_blocks();
        options.file_system = Some(PathBuf::from("test/fixtures/theme/snippets"));
        options.register_block("paginate", Box::new(|_tag_name: &str, _arguments: &[Token], tokens: Vec<Element>, options: &LiquidOptions| {
            Ok(Box::new(Paginate{}))
        }));

        let template = liquid::parse_file(PathBuf::from("test/fixtures/theme/templates/search.liquid").as_path(), options).unwrap();

        let mut context = Context::new();
        context.set_val("num", Value::Num(4f32));

        let output = template.render(&mut context).unwrap_or_else(|e: liquid::Error| Some(e.description().to_string()) );
        Ok(Response::with((status::Ok, output.unwrap_or("toto".to_string()))))
    }
}
