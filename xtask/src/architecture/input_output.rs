//! Policy: the input and output a file of `domain/` or `use_cases/` names, found in its syntax tree.

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, ExprMethodCall, Ident, ItemUse, Macro, Path, PathSegment, Token, UseTree};

/// The paths that do input or output.
const INPUT_OUTPUT: [&[&str]; 8] = [
    &["std", "fs"],
    &["std", "io"],
    &["std", "process"],
    &["std", "env"],
    &["std", "net"],
    &["std", "os"],
    &["cargo_metadata"],
    &["toml"],
];

/// The methods of `Path` that open what it names, refused whatever they are called on.
const OPENING_METHODS: [&str; 7] = [
    "canonicalize",
    "exists",
    "metadata",
    "read_dir",
    "read_link",
    "symlink_metadata",
    "try_exists",
];

/// The macros that write to the terminal.
const PRINTING_MACROS: [&str; 5] = ["print", "println", "eprint", "eprintln", "dbg"];

/// Each use of input or output in `text`, as `line N: what it is`, or why `text` is not Rust.
pub(super) fn breaches(text: &str) -> Result<Vec<String>, String> {
    let file = syn::parse_file(text).map_err(not_rust)?;
    let mut finder = Finder::default();
    finder.visit_file(&file);
    Ok(finder.breaches)
}

fn not_rust(error: syn::Error) -> String {
    format!("line {}: {error}", error.span().start().line)
}

#[derive(Default)]
struct Finder {
    breaches: Vec<String>,
}

impl Finder {
    fn check(&mut self, segments: &[String], span: Span) {
        if let Some(found) = INPUT_OUTPUT.iter().find(|io| starts_with(segments, io)) {
            self.refuse(
                span,
                &format!("names {}, which does input or output", found.join("::")),
            );
        }
    }

    fn check_method(&mut self, method: &Ident) {
        if OPENING_METHODS.contains(&method.to_string().as_str()) {
            self.refuse(
                method.span(),
                &format!("calls {method}, which opens a path"),
            );
        }
    }

    fn refuse(&mut self, span: Span, what: &str) {
        self.breaches
            .push(format!("line {}: {what}.", span.start().line));
    }
}

impl<'ast> Visit<'ast> for Finder {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        let imports = imported(&item.tree, &[]);
        imports
            .iter()
            .filter(|(segments, _)| is_std_whole(segments))
            .for_each(|(_, span)| {
                self.refuse(*span, "imports std whole, which hides what it uses")
            });
        imports
            .iter()
            .for_each(|(segments, span)| self.check(segments, *span));
    }

    fn visit_path(&mut self, path: &'ast Path) {
        let segments: Vec<String> = path.segments.iter().map(name_of).collect();
        if segments.len() > 1 {
            self.check(&segments, path.span());
        }
        if let Some(method) = method_of_path(path) {
            self.check_method(method);
        }
        visit::visit_path(self, path);
    }

    fn visit_expr_method_call(&mut self, call: &'ast ExprMethodCall) {
        self.check_method(&call.method);
        visit::visit_expr_method_call(self, call);
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        if PRINTING_MACROS.iter().any(|name| mac.path.is_ident(name)) {
            self.refuse(mac.path.span(), "prints, which is output");
        }
        if let Ok(arguments) = mac.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        {
            arguments
                .iter()
                .for_each(|argument| self.visit_expr(argument));
        }
        visit::visit_macro(self, mac);
    }
}

fn name_of(segment: &PathSegment) -> String {
    segment.ident.to_string()
}

/// The method a path names through `Path` or `PathBuf`, such as `exists` in `Path::exists`.
fn method_of_path(path: &Path) -> Option<&Ident> {
    let mut segments = path.segments.iter().rev();
    let method = segments.next()?;
    let owner = segments.next()?;
    (owner.ident == "Path" || owner.ident == "PathBuf").then_some(&method.ident)
}

fn starts_with(segments: &[String], prefix: &[&str]) -> bool {
    segments.len() >= prefix.len()
        && segments
            .iter()
            .zip(prefix)
            .all(|(segment, name)| segment == name)
}

fn is_std_whole(segments: &[String]) -> bool {
    segments == ["std"]
}

/// Every path a `use` tree imports, each with where it is written; a glob or a rename imports its whole prefix.
fn imported(tree: &UseTree, prefix: &[String]) -> Vec<(Vec<String>, Span)> {
    match tree {
        UseTree::Path(path) => imported(&path.tree, &joined(prefix, &path.ident)),
        UseTree::Name(name) => vec![(joined(prefix, &name.ident), name.span())],
        UseTree::Rename(rename) => vec![(joined(prefix, &rename.ident), rename.span())],
        UseTree::Glob(glob) => vec![(prefix.to_vec(), glob.span())],
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| imported(item, prefix))
            .collect(),
    }
}

fn joined(prefix: &[String], name: &Ident) -> Vec<String> {
    [prefix, &[name.to_string()]].concat()
}
