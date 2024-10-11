extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};
use rcdom::RcDom;

fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let data = "<!DOCTYPE html><html><body><a href=\"foo\"></a></body></html>".to_string();
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut data.as_bytes())
        .unwrap();

    println!("{:?}", dom.document);
}
