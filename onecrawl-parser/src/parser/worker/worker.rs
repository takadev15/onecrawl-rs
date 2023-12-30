use std::io::Read;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

pub fn parse_worker() {
    let html_content = r#"
        <html>
            <body>
                <div class="content">
                    <h1>Hello, world!</h1>
                    <p>This is a sample paragraph.</p>
                </div>
            </body>
        </html>
    "#;

}

// fn find_h1(handle: &Handle) -> Handle {
//     unimplemented!();
// } 
