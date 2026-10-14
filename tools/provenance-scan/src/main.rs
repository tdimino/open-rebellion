//! Flags simulation code that has no recorded source in the original game.
//!
//! Every function, method, `const`, and `static` in the scanned files is an
//! item. An item is cited when its comments (the block directly above it and
//! any comment inside it) carry a source: a `FUN_`/`DAT_` address, a GNPRTB
//! parameter, a DAT file or table name, or an explicit `src:` tag. `port:` and
//! `Augmentation` mark port-owned design. `hyp:` and "Inferred" mark a guess
//! that still awaits recovery; a hypothesis is counted, never treated as
//! cited.
//!
//! An uncited item with hits (numeric literals other than 0 and 1, or roll
//! draws) is a finding. `check` fails when a finding is missing from the
//! baseline or has grown past it; `baseline` rewrites the baseline and refuses
//! to let it grow. The scan is item-level: it finds where to look, and the
//! per-system review decides whether each cited line matches its source.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use syn::spanned::Spanned;
use syn::visit::Visit;

/// Directories scanned recursively, relative to the workspace root.
const SCAN_DIRS: &[&str] = &["crates/rebellion-core/src"];
/// Single files scanned in addition to `SCAN_DIRS`.
const SCAN_FILES: &[&str] = &[
    "crates/rebellion-data/src/integrator.rs",
    "crates/rebellion-data/src/simulation.rs",
];
/// Paths skipped inside `SCAN_DIRS`: binary DAT layout, the wire format, and
/// serialization helpers, whose numbers are formats rather than rules.
const SKIP: &[&str] = &[
    "crates/rebellion-core/src/dat",
    "crates/rebellion-core/src/net_protocol.rs",
    "crates/rebellion-core/src/serde_ordered.rs",
];
const BASELINE: &str = "scripts/provenance-baseline.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum Provenance {
    Cited,
    Port,
    Hyp,
    Uncited,
}

#[derive(Debug, Serialize)]
struct Hit {
    line: usize,
    kind: &'static str,
    text: String,
}

#[derive(Debug, Serialize)]
struct ItemReport {
    key: String,
    start: usize,
    end: usize,
    provenance: Provenance,
    hits: Vec<Hit>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Baseline {
    /// `file::item` → number of hits the uncited item carried.
    items: BTreeMap<String, usize>,
}

fn classify(comments: &str) -> Provenance {
    let has = |needle: &str| comments.contains(needle);
    let cited = comments
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '.')
        .any(|word| {
            // Ghidra spells data globals read as floats `_DAT_…`.
            let address = word.trim_start_matches('_');
            (address.starts_with("FUN_") || address.starts_with("DAT_")) && address.len() >= 10
                || word.starts_with("GNPRTB")
                || word.ends_with(".DAT")
                || (word.len() >= 5
                    && word.ends_with("TB")
                    && word
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()))
        })
        || has("src:");
    if cited {
        Provenance::Cited
    } else if has("port:") || has("Augmentation") {
        Provenance::Port
    } else if has("hyp:") || has("Inferred") || has("inferred") {
        Provenance::Hyp
    } else {
        Provenance::Uncited
    }
}

/// Collects hits inside one item. Types, attributes, macro bodies, and
/// literal indices are skipped: they hold layout, not rules.
struct HitVisitor<'a> {
    lines: &'a [&'a str],
    hits: Vec<Hit>,
}

impl HitVisitor<'_> {
    fn push(&mut self, line: usize, kind: &'static str) {
        let text = self
            .lines
            .get(line.wrapping_sub(1))
            .map_or("", |l| l.trim())
            .to_string();
        self.hits.push(Hit { line, kind, text });
    }
}

fn is_trivial(lit: &syn::Lit) -> bool {
    match lit {
        syn::Lit::Int(i) => matches!(i.base10_digits(), "0" | "1"),
        syn::Lit::Float(f) => f.base10_parse::<f64>().is_ok_and(|v| v == 0.0 || v == 1.0),
        _ => true,
    }
}

impl<'ast> Visit<'ast> for HitVisitor<'_> {
    fn visit_expr_lit(&mut self, node: &'ast syn::ExprLit) {
        if !is_trivial(&node.lit) {
            self.push(node.span().start().line, "literal");
        }
    }

    fn visit_expr_index(&mut self, node: &'ast syn::ExprIndex) {
        if let syn::Expr::Path(path) = &*node.expr {
            if path.path.is_ident("rolls") {
                self.push(node.span().start().line, "roll");
            }
        }
        self.visit_expr(&node.expr);
        if !matches!(&*node.index, syn::Expr::Lit(_)) {
            self.visit_expr(&node.index);
        }
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let name = node.method.to_string();
        if matches!(
            name.as_str(),
            "gen_range" | "gen_bool" | "random" | "next_f64" | "next_f32" | "next_u32" | "next_u64"
        ) {
            self.push(node.span().start().line, "roll");
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_type(&mut self, _: &'ast syn::Type) {}
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) {}
    fn visit_macro(&mut self, _: &'ast syn::Macro) {}
    // Nested items are reported on their own.
    fn visit_item(&mut self, _: &'ast syn::Item) {}
}

fn is_test_gated(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("test")
            || (attr.path().is_ident("cfg")
                && attr
                    .meta
                    .require_list()
                    .is_ok_and(|list| list.tokens.to_string().contains("test")))
    })
}

struct FileScan<'a> {
    file: String,
    lines: Vec<&'a str>,
    items: Vec<ItemReport>,
    seen: BTreeMap<String, usize>,
}

impl<'a> FileScan<'a> {
    /// The comment text an item carries: the contiguous `//` block directly
    /// above it (attributes included) and every comment line inside it.
    fn comments(&self, start: usize, end: usize) -> String {
        let mut first = start;
        while first > 1 {
            let above = self.lines[first - 2].trim_start();
            if above.starts_with("//") || above.starts_with("#[") {
                first -= 1;
            } else {
                break;
            }
        }
        self.lines[first - 1..end.min(self.lines.len())]
            .iter()
            .filter_map(|line| line.find("//").map(|at| &line[at..]))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn report(
        &mut self,
        name: String,
        span: proc_macro2::Span,
        visit: impl FnOnce(&mut HitVisitor),
    ) {
        let (start, end) = (span.start().line, span.end().line);
        let mut visitor = HitVisitor {
            lines: &self.lines,
            hits: Vec::new(),
        };
        visit(&mut visitor);
        let count = self.seen.entry(name.clone()).or_insert(0);
        *count += 1;
        let key = if *count == 1 {
            format!("{}::{name}", self.file)
        } else {
            format!("{}::{name}#{count}", self.file)
        };
        let provenance = classify(&self.comments(start, end));
        let hits = visitor.hits;
        self.items.push(ItemReport {
            key,
            start,
            end,
            provenance,
            hits,
        });
    }

    fn items(&mut self, items: &[syn::Item], prefix: &str) {
        for item in items {
            match item {
                syn::Item::Fn(f) if !is_test_gated(&f.attrs) => {
                    self.report(format!("{prefix}{}", f.sig.ident), f.span(), |v| {
                        v.visit_block(&f.block)
                    });
                }
                syn::Item::Const(c) if !is_test_gated(&c.attrs) => {
                    self.report(format!("{prefix}{}", c.ident), c.span(), |v| {
                        v.visit_expr(&c.expr)
                    });
                }
                syn::Item::Static(s) if !is_test_gated(&s.attrs) => {
                    self.report(format!("{prefix}{}", s.ident), s.span(), |v| {
                        v.visit_expr(&s.expr)
                    });
                }
                syn::Item::Mod(m) if !is_test_gated(&m.attrs) => {
                    if let Some((_, inner)) = &m.content {
                        self.items(inner, &format!("{prefix}{}::", m.ident));
                    }
                }
                syn::Item::Impl(imp) if !is_test_gated(&imp.attrs) => {
                    let ty = match &*imp.self_ty {
                        syn::Type::Path(p) => p
                            .path
                            .segments
                            .last()
                            .map_or("_".into(), |s| s.ident.to_string()),
                        _ => "_".into(),
                    };
                    for member in &imp.items {
                        match member {
                            syn::ImplItem::Fn(f) if !is_test_gated(&f.attrs) => {
                                self.report(
                                    format!("{prefix}{ty}::{}", f.sig.ident),
                                    f.span(),
                                    |v| v.visit_block(&f.block),
                                );
                            }
                            syn::ImplItem::Const(c) => {
                                self.report(format!("{prefix}{ty}::{}", c.ident), c.span(), |v| {
                                    v.visit_expr(&c.expr)
                                });
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Trait(t) if !is_test_gated(&t.attrs) => {
                    for member in &t.items {
                        if let syn::TraitItem::Fn(f) = member {
                            if let Some(block) = &f.default {
                                self.report(
                                    format!("{prefix}{}::{}", t.ident, f.sig.ident),
                                    f.span(),
                                    |v| v.visit_block(block),
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn scan_source(file: &str, source: &str) -> Result<Vec<ItemReport>> {
    let parsed = syn::parse_file(source).with_context(|| format!("parsing {file}"))?;
    let mut scan = FileScan {
        file: file.to_string(),
        lines: source.lines().collect(),
        items: Vec::new(),
        seen: BTreeMap::new(),
    };
    scan.items(&parsed.items, "");
    Ok(scan.items)
}

/// What to scan, relative to the workspace root.
struct Scope<'a> {
    /// Directories scanned recursively.
    dirs: &'a [&'a str],
    /// Single files scanned in addition to `dirs`.
    files: &'a [&'a str],
    /// Paths skipped inside `dirs`, matched exactly or as a directory prefix.
    skip: &'a [&'a str],
}

const SCOPE: Scope<'static> = Scope {
    dirs: SCAN_DIRS,
    files: SCAN_FILES,
    skip: SKIP,
};

fn rust_files(root: &Path, scope: &Scope, dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        let rel = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        if scope
            .skip
            .iter()
            .any(|skip| rel == *skip || rel.starts_with(&format!("{skip}/")))
        {
            continue;
        }
        if path.is_dir() {
            rust_files(root, scope, &path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

fn scan_workspace(root: &Path, scope: &Scope) -> Result<Vec<ItemReport>> {
    let mut paths = Vec::new();
    for dir in scope.dirs {
        rust_files(root, scope, &root.join(dir), &mut paths)?;
    }
    paths.extend(scope.files.iter().map(|file| root.join(file)));
    paths.sort();
    let mut reports = Vec::new();
    for path in paths {
        let rel = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        let source = std::fs::read_to_string(&path).with_context(|| format!("reading {rel}"))?;
        reports.extend(scan_source(&rel, &source)?);
    }
    Ok(reports)
}

/// Uncited items that carry at least one hit, keyed as the baseline keys them.
fn findings(reports: &[ItemReport]) -> BTreeMap<String, usize> {
    reports
        .iter()
        .filter(|r| r.provenance == Provenance::Uncited && !r.hits.is_empty())
        .map(|r| (r.key.clone(), r.hits.len()))
        .collect()
}

/// Findings that are new or larger than the baseline allows.
fn regressions(baseline: &Baseline, current: &BTreeMap<String, usize>) -> Vec<String> {
    current
        .iter()
        .filter(|(key, count)| {
            baseline
                .items
                .get(*key)
                .is_none_or(|allowed| *count > allowed)
        })
        .map(|(key, count)| {
            format!(
                "{key}: {count} hits (baseline {})",
                baseline
                    .items
                    .get(key)
                    .map_or("none".into(), |n| n.to_string())
            )
        })
        .collect()
}

/// Items with hits per file, split by provenance: cited, port, hyp, uncited.
fn tally(reports: &[ItemReport]) -> BTreeMap<&str, [usize; 4]> {
    let mut per_file: BTreeMap<&str, [usize; 4]> = BTreeMap::new();
    for report in reports.iter().filter(|r| !r.hits.is_empty()) {
        let file = report.key.split("::").next().unwrap_or("");
        let slot = match report.provenance {
            Provenance::Cited => 0,
            Provenance::Port => 1,
            Provenance::Hyp => 2,
            Provenance::Uncited => 3,
        };
        per_file.entry(file).or_default()[slot] += 1;
    }
    per_file
}

fn summary(reports: &[ItemReport]) -> String {
    let row = |name: &str, c: &[usize; 4]| {
        format!(
            "{name:<52} {:>6} {:>5} {:>4} {:>8}\n",
            c[0], c[1], c[2], c[3]
        )
    };
    let mut out = format!(
        "{:<52} {:>6} {:>5} {:>4} {:>8}\n",
        "file (items with hits)", "cited", "port", "hyp", "uncited"
    );
    let mut totals = [0; 4];
    for (file, counts) in &tally(reports) {
        out += &row(file, counts);
        for (total, n) in totals.iter_mut().zip(counts) {
            *total += n;
        }
    }
    out + &row("total", &totals)
}

/// Runs one command and returns what it prints.
fn run(root: &Path, scope: &Scope, baseline_path: &Path, args: &[String]) -> Result<String> {
    let reports = scan_workspace(root, scope)?;
    let current = findings(&reports);
    let read_baseline = || -> Result<Option<Baseline>> {
        if !baseline_path.exists() {
            return Ok(None);
        }
        Ok(Some(serde_json::from_str(&std::fs::read_to_string(
            baseline_path,
        )?)?))
    };
    match args.first().map(String::as_str) {
        None | Some("report") => {
            let mut out = String::new();
            if let Some(path) = args
                .iter()
                .position(|a| a == "--json")
                .and_then(|at| args.get(at + 1))
            {
                std::fs::write(path, serde_json::to_string_pretty(&reports)?)?;
                out = format!("wrote {} items to {path}\n", reports.len());
            }
            Ok(out + &summary(&reports))
        }
        Some("check") => {
            let baseline =
                read_baseline()?.context("no baseline; run `provenance-scan baseline` first")?;
            let bad = regressions(&baseline, &current);
            if !bad.is_empty() {
                bail!(
                    "{} uncited item(s) outside the baseline:\n  {}",
                    bad.len(),
                    bad.join("\n  ")
                );
            }
            let cleared = baseline
                .items
                .keys()
                .filter(|k| !current.contains_key(*k))
                .count();
            Ok(format!(
                "ok: {} uncited items, all within the baseline ({cleared} cleared since)\n",
                current.len()
            ))
        }
        Some("baseline") => {
            if let Some(old) = read_baseline()? {
                let bad = regressions(&old, &current);
                if !bad.is_empty() {
                    bail!(
                        "the baseline may only shrink; cite or tag these first:\n  {}",
                        bad.join("\n  ")
                    );
                }
            }
            let baseline = Baseline { items: current };
            std::fs::write(
                baseline_path,
                serde_json::to_string_pretty(&baseline)? + "\n",
            )?;
            Ok(format!(
                "wrote {} items to {}\n",
                baseline.items.len(),
                baseline_path.display()
            ))
        }
        Some(other) => {
            bail!("unknown command `{other}`; use report [--json PATH], check, or baseline")
        }
    }
}

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    print!("{}", run(&root, &SCOPE, &root.join(BASELINE), &args)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Vec<ItemReport> {
        scan_source("x.rs", source).unwrap()
    }

    #[test]
    fn a_literal_in_an_uncommented_function_is_an_uncited_hit() {
        let r = scan("fn f() -> i32 { 35 }");
        assert_eq!(r[0].provenance, Provenance::Uncited);
        assert_eq!(r[0].hits.len(), 1);
    }

    #[test]
    fn zero_one_indices_types_and_macros_are_not_hits() {
        let r = scan("fn f(a: [u8; 4]) -> u8 { let b = a[3] + 1 - 0; assert_eq!(b, 7); b }");
        assert!(r[0].hits.is_empty(), "{:?}", r[0].hits);
    }

    #[test]
    fn a_ghidra_address_above_or_inside_an_item_cites_it() {
        let above = scan("// FUN_0058a020 decoy phase\nfn f() -> i32 { 35 }");
        let inside = scan("fn f() -> i32 {\n    // GNPRTB 3588\n    35\n}");
        assert_eq!(above[0].provenance, Provenance::Cited);
        assert_eq!(inside[0].provenance, Provenance::Cited);
    }

    #[test]
    fn a_hypothesis_tag_is_counted_but_not_cited() {
        let r = scan("// hyp: guessed until FUN_ recovery\nfn f() -> i32 { 35 }");
        assert_eq!(r[0].provenance, Provenance::Hyp);
        assert_eq!(findings(&r).len(), 0);
    }

    #[test]
    fn a_port_tag_marks_design_the_original_does_not_own() {
        let r = scan("/// port: UI layout spacing\nconst GAP: f32 = 12.0;");
        assert_eq!(r[0].provenance, Provenance::Port);
    }

    #[test]
    fn test_modules_and_test_functions_are_skipped() {
        let r =
            scan("#[cfg(test)]\nmod tests { fn g() -> i32 { 9 } }\n#[test]\nfn t() { let _ = 9; }");
        assert!(r.is_empty());
    }

    #[test]
    fn roll_draws_are_hits() {
        let r = scan("fn f(rolls: &[f64]) -> bool { rolls[i] < rolls[j] }");
        assert_eq!(r[0].hits.iter().filter(|h| h.kind == "roll").count(), 2);
    }

    #[test]
    fn methods_are_keyed_by_type_and_duplicates_are_numbered() {
        let r = scan("impl A { fn go() -> u8 { 2 } }\nimpl A { fn go() -> u8 { 3 } }");
        assert_eq!(r[0].key, "x.rs::A::go");
        assert_eq!(r[1].key, "x.rs::A::go#2");
    }

    #[test]
    fn the_baseline_rejects_a_new_or_grown_finding_and_accepts_a_shrunk_one() {
        let baseline = Baseline {
            items: BTreeMap::from([("a".into(), 3)]),
        };
        assert!(regressions(&baseline, &BTreeMap::from([("a".into(), 2)])).is_empty());
        assert_eq!(
            regressions(&baseline, &BTreeMap::from([("a".into(), 4)])).len(),
            1
        );
        assert_eq!(
            regressions(&baseline, &BTreeMap::from([("b".into(), 1)])).len(),
            1
        );
    }

    #[test]
    fn each_source_form_cites_and_a_bare_prefix_does_not() {
        for comment in [
            "// DAT_006bb3dc",
            "// TROOPSD.DAT field",
            "// UPRIS1TB",
            "// src: manual p. 40",
        ] {
            assert_eq!(classify(comment), Provenance::Cited, "{comment}");
        }
        for comment in [
            "// FUN_ later",
            "// DAT_ later",
            "// ATB",
            "// upris1TB",
            "// plain",
        ] {
            assert_eq!(classify(comment), Provenance::Uncited, "{comment}");
        }
    }

    #[test]
    fn augmentation_is_port_owned_and_inferred_is_a_hypothesis() {
        assert_eq!(classify("// Augmentation: port rule"), Provenance::Port);
        assert_eq!(classify("// Inferred from play"), Provenance::Hyp);
        assert_eq!(classify("// inferred from play"), Provenance::Hyp);
    }

    #[test]
    fn float_zero_and_one_are_trivial_and_other_floats_are_hits() {
        let r = scan("fn f() -> f64 { 0.0 + 1.0 }\nfn g() -> f64 { 0.5 + 2.0 }");
        assert!(r[0].hits.is_empty(), "{:?}", r[0].hits);
        assert_eq!(r[1].hits.len(), 2);
    }

    #[test]
    fn a_random_number_method_is_a_roll_and_other_methods_are_not() {
        let r = scan("fn f(rng: &mut R, v: &V) -> u8 { v.clone(); rng.gen_range(0..1) }");
        let kinds: Vec<_> = r[0].hits.iter().map(|h| h.kind).collect();
        assert_eq!(kinds, ["roll"]);
    }

    #[test]
    fn only_a_cfg_test_attribute_gates_an_item() {
        let r = scan("#[expect(unused, reason = \"test\")]\nfn f() -> u8 { 5 }\n#[cfg(feature = \"x\")]\nfn g() -> u8 { 5 }");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn every_item_kind_is_reported_unless_test_gated() {
        let source = "
            const C: u8 = 5;
            static S: u8 = 5;
            mod m { fn inner() -> u8 { 5 } }
            impl A { const K: u8 = 5; fn m() -> u8 { 5 } }
            trait T { fn d() -> u8 { 5 } }
            #[cfg(test)] const TC: u8 = 5;
            #[cfg(test)] static TS: u8 = 5;
            #[cfg(test)] impl B { fn m() -> u8 { 5 } }
            #[cfg(test)] trait U { fn d() -> u8 { 5 } }
            impl D { #[test] fn t() { let _ = 5; } }
        ";
        let keys: Vec<_> = scan(source).into_iter().map(|r| r.key).collect();
        assert_eq!(
            keys,
            [
                "x.rs::C",
                "x.rs::S",
                "x.rs::m::inner",
                "x.rs::A::K",
                "x.rs::A::m",
                "x.rs::T::d"
            ]
        );
    }

    #[test]
    fn findings_keep_only_uncited_items_that_carry_hits() {
        let r = scan("fn a() -> u8 { 5 }\nfn b() {}\n// FUN_0058a020\nfn c() -> u8 { 5 }");
        assert_eq!(findings(&r), BTreeMap::from([("x.rs::a".to_string(), 1)]));
    }

    #[test]
    fn a_finding_at_its_baseline_count_is_not_a_regression() {
        let baseline = Baseline {
            items: BTreeMap::from([("a".into(), 3)]),
        };
        assert!(regressions(&baseline, &BTreeMap::from([("a".into(), 3)])).is_empty());
    }

    #[test]
    fn the_summary_counts_items_with_hits_by_provenance_and_totals_them() {
        let r = scan("fn a() -> u8 { 5 }\nfn b() -> u8 { 5 }\n// FUN_0058a020\nfn c() -> u8 { 5 }\n// hyp: x\nfn d() -> u8 { 5 }\n// port: x\nfn e() -> u8 { 5 }\nfn f() {}");
        assert_eq!(tally(&r)["x.rs"], [1, 1, 1, 2]);
        assert!(summary(&r).ends_with(&format!(
            "{:<52} {:>6} {:>5} {:>4} {:>8}\n",
            "total", 1, 1, 1, 2
        )));
    }

    /// A scratch workspace under the system temp directory.
    fn workspace(name: &str, files: &[(&str, &str)]) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("provenance-scan-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (path, body) in files {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        root
    }

    const TEST_SCOPE: Scope<'static> = Scope {
        dirs: &["src"],
        files: &["extra.rs"],
        skip: &["src/skipdir", "src/skip.rs"],
    };

    #[test]
    fn the_walk_scans_rust_files_under_its_dirs_and_files_and_honours_skips() {
        let lit = "fn f() -> u8 { 5 }";
        let root = workspace(
            "walk",
            &[
                ("src/a.rs", lit),
                ("src/sub/c.rs", lit),
                ("src/skip.rs", lit),
                ("src/skip.rsx/e.rs", lit),
                ("src/skipdir/b.rs", lit),
                ("src/skipdirx/d.rs", lit),
                ("src/notes.txt", lit),
                ("extra.rs", lit),
            ],
        );
        let keys: Vec<_> = scan_workspace(&root, &TEST_SCOPE)
            .unwrap()
            .into_iter()
            .map(|r| r.key)
            .collect();
        assert_eq!(
            keys,
            [
                "extra.rs::f",
                "src/a.rs::f",
                "src/skip.rsx/e.rs::f",
                "src/skipdirx/d.rs::f",
                "src/sub/c.rs::f"
            ]
        );
    }

    #[test]
    fn the_commands_report_record_check_and_refuse_a_growing_baseline() {
        let root = workspace(
            "run",
            &[
                ("src/a.rs", "fn a() -> u8 { 5 }\nfn b() -> u8 { 6 }"),
                ("extra.rs", ""),
            ],
        );
        let baseline = root.join("baseline.json");
        let args = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let run_with = |list: &[&str]| run(&root, &TEST_SCOPE, &baseline, &args(list));

        let json = root.join("report.json");
        let out = run_with(&["report", "--json", json.to_str().unwrap()]).unwrap();
        assert!(out.starts_with("wrote 2 items to "), "{out}");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(&json).unwrap())
                .unwrap()
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert!(run_with(&[]).unwrap().starts_with("file (items with hits)"));

        assert!(run_with(&["check"]).is_err(), "check needs a baseline");
        assert!(run_with(&["baseline"])
            .unwrap()
            .starts_with("wrote 2 items to "));
        assert_eq!(
            run_with(&["check"]).unwrap(),
            "ok: 2 uncited items, all within the baseline (0 cleared since)\n"
        );

        std::fs::write(root.join("src/a.rs"), "fn a() -> u8 { 5 }").unwrap();
        assert_eq!(
            run_with(&["check"]).unwrap(),
            "ok: 1 uncited items, all within the baseline (1 cleared since)\n"
        );

        std::fs::write(root.join("src/a.rs"), "fn a() -> u8 { 5 + 7 }").unwrap();
        assert!(run_with(&["check"])
            .unwrap_err()
            .to_string()
            .contains("src/a.rs::a: 2 hits (baseline 1)"));
        assert!(run_with(&["baseline"])
            .unwrap_err()
            .to_string()
            .contains("may only shrink"));
        assert!(run_with(&["bogus"]).is_err());
    }
}
