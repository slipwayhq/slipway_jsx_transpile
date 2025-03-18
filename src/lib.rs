use serde::{Deserialize, Serialize};
use std::rc::Rc;
use swc_common::{FileName, GLOBALS, Globals, Mark, SourceMap, comments::SingleThreadedComments};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms::react::{Options as ReactOptions, jsx};
use swc_ecma_visit::VisitMutWith;

wit_bindgen::generate!({
    world: "slipway",
});

struct Component;

export!(Component);

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let js = jsx_to_js(&input.jsx)?;
        let output = Output { js };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

fn jsx_to_js(jsx_str: &str) -> Result<String, ComponentError> {
    // Create a SourceMap and load your JSX source.
    let cm = Rc::new(SourceMap::default());
    let fm = cm.new_source_file(
        Rc::new(FileName::Real("example.jsx".into())),
        jsx_str.into(),
    );

    // Parse as ES with JSX enabled.
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        StringInput::from(&*fm),
        None,
    );
    let mut module = parser.parse_module().expect("Failed to parse");

    let globals = Globals::new();
    let comments = SingleThreadedComments::default();

    // Transform JSX to JS.
    GLOBALS.set(&globals, || {
        let top_level_mark = Mark::fresh(Mark::root());
        let mut jsx_transform = jsx(
            cm.clone(),
            Some(&comments),
            ReactOptions::default(),
            top_level_mark,
            Default::default(),
        );
        module.visit_mut_with(&mut jsx_transform);
    });

    // Write out the final JavaScript.
    let mut buf = Vec::new();
    {
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
        };
        emitter.emit_module(&module).map_err(|e| ComponentError {
            message: "Failed to emit transpiled code.".to_string(),
            inner: vec![e.to_string()],
        })?;
    }

    let result = String::from_utf8(buf).expect("Emitted code should be valid UTF-8");
    Ok(result)
}

#[derive(Deserialize)]
struct Input {
    jsx: String,
}

#[derive(Serialize)]
struct Output {
    js: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_simple_jsx() {
        let jsx = r#"<h1>Hello World!</h1>"#;
        let js = jsx_to_js(jsx).unwrap();
        assert_eq!(
            js.trim(),
            r#"React.createElement("h1", null, "Hello World!");"#
        );
    }
}
