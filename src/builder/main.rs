use director::Director;
use html_builder::HTMLBuilder;
use text_builder::TextBuilder;

mod builder;
mod director;
mod html_builder;
mod text_builder;
fn main() {
    {
        let mut builder = TextBuilder::new();
        let mut director = Director::new(&mut builder);
        director.construct();
        let result = builder.get_result();
        println!("{}", result);
    }
    {
        let mut builder = HTMLBuilder::new("index.html");
        let mut director = Director::new(&mut builder);
        director.construct();
        let filename = builder.get_result();
        println!("{}が作成されました。", filename);
    }
}
