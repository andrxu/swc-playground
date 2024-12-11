use std::fs;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_visit::Visit;

struct AstDumper;

impl Visit for AstDumper {
    fn visit_module(&mut self, module: &Module) {
        // Here you can implement how you want to dump the AST
        println!("{:#?}", module); // Print the AST in a debug format
                                   // You can also write to a file or format it differently if needed
    }
}

fn main() {
    let file_path = "./test.js"; // Replace with your actual file path
    let code = fs::read_to_string(file_path).expect("Unable to read file");

    // Create a SourceMap
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(Lrc::new(FileName::Custom(file_path.into())), code.into());

    let syntax = Syntax::Es(Default::default());
    let lexer = swc_ecma_parser::lexer::Lexer::new(
        syntax,
        EsVersion::Es2022,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    // Parse the code into an AST
    let module = parser.parse_module().expect("Failed to parse module");

    // Dump the AST for debugging
    let mut dumper = AstDumper;
    dumper.visit_module(&module);
}
