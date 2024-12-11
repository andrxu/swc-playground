use std::fs;
use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::{EsVersion, Module, ModuleDecl, ModuleItem, Stmt};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_visit::Visit;

struct AstDumper;

impl Visit for AstDumper {
    fn visit_module(&mut self, module: &Module) {
        // Visit each item in the module
        for item in &module.body {
            self.visit_module_item(item);
        }
    }

    fn visit_module_item(&mut self, item: &ModuleItem) {
        match item {
            ModuleItem::Stmt(stmt) => {
                // Handle statements (e.g., function declarations, class declarations)
                self.visit_stmt(stmt);
            }
            ModuleItem::ModuleDecl(module_decl) => {
                // Handle module declarations (imports and exports)
                self.visit_module_decl(module_decl);
            }
        }
    }

    fn visit_module_decl(&mut self, decl: &ModuleDecl) {
        match decl {
            ModuleDecl::Import(import_decl) => {
                println!("Import: {:?}", import_decl);
            }
            ModuleDecl::ExportNamed(export_decl) => {
                println!("Export: {:?}", export_decl);
            }
            ModuleDecl::ExportDefaultExpr(export_default_decl) => {
                println!("Export default: {:?}", export_default_decl);
            }
            _ => {} // Handle other types of module declarations if needed
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Decl(decl) => {
                // Handle declarations (functions, classes, etc.)
                self.visit_decl(decl);
            }
            _ => {} // Handle other types of statements if needed
        }
    }

    fn visit_decl(&mut self, decl: &swc_ecma_ast::Decl) {
        match decl {
            swc_ecma_ast::Decl::Fn(func_decl) => {
                println!("Function: {:?}", func_decl.ident);
            }
            swc_ecma_ast::Decl::Class(class_decl) => {
                println!("Class: {:?}", class_decl.ident);
            }
            _ => {} // Handle other types of declarations if needed
        }
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
