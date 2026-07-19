//! Offline locator generator + parse-error explainer for javagui-spy.
//!
//! This module ports the reference algorithm in `python/JavaGui/spy/generator.py` into
//! Rust and exposes it via PyO3. Instead of one RPC per candidate, every candidate is
//! verified OFFLINE against the parsed component tree using the *production* matcher
//! (`crate::locator::matcher::find_matching_components`), guaranteeing parity with the
//! live locator engine while avoiding round-trips.
//!
//! Two PyO3 functions are exported (registered in `src/lib.rs`):
//!
//! - [`suggest_locators`] — ranked, verified locator candidates for a target node.
//! - [`explain_locator`] — parse a locator, returning `{ok, position, hint, message}`.
//!
//! The get_ui_tree JSON payload is the nested/serde form:
//! `{roots:[node]}`; `node.id.hash_code`, `node.component_type.simple_name`,
//! `node.identity.{name,text,title,tooltip}`, `node.geometry.bounds.{x,y,width,height}`,
//! `node.accessibility.accessible_name`, `node.children`.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;

use crate::model::{
    AccessibilityInfo, Bounds, ComponentGeometry, ComponentId, ComponentIdentity,
    ComponentProperties, ComponentState, ComponentType, SwingBaseType, TraversalMetadata,
    UIComponent,
};

use super::matcher::{find_matching_components, Evaluator};
use super::parser::parse_locator;

// ---------------------------------------------------------------------------
// Stability weights (the attribute-priority ladder) — parity with Python
// ---------------------------------------------------------------------------
fn stability_of(attr: &str) -> f64 {
    match attr {
        "name" => 1.00,
        "accessiblename" => 0.90,
        "text" => 0.75,
        "tooltip" => 0.65,
        "nth-of-type" => 0.40,
        "index" => 0.25,
        "geometry" => 0.15,
        _ => 0.10,
    }
}

// The default attribute-priority order (name > accessiblename > text > tooltip).
const QUALIFIER_ORDER: [&str; 4] = ["name", "accessiblename", "text", "tooltip"];

// ---------------------------------------------------------------------------
// Nested tree accessors (mirror the Python node_* helpers)
// ---------------------------------------------------------------------------
fn node_type(n: &Value) -> String {
    n.get("component_type")
        .and_then(|c| c.get("simple_name"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("Component")
        .to_string()
}

fn node_class(n: &Value) -> Option<String> {
    n.get("component_type")
        .and_then(|c| c.get("class_name"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

fn node_name(n: &Value) -> Option<String> {
    n.get("identity")
        .and_then(|i| i.get("name"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

fn node_text(n: &Value) -> Option<String> {
    let idy = n.get("identity")?;
    let raw = idy
        .get("text")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .or_else(|| idy.get("title").and_then(|v| v.as_str()).filter(|s| !s.is_empty()))?;
    // Strip mnemonics ('&') and collapse whitespace, matching the Python normalizer.
    let cleaned = raw.replace('&', "");
    let collapsed = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        None
    } else {
        Some(collapsed)
    }
}

fn node_tooltip(n: &Value) -> Option<String> {
    n.get("identity")
        .and_then(|i| i.get("tooltip"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

fn node_accessible_name(n: &Value) -> Option<String> {
    n.get("accessibility")
        .and_then(|a| a.get("accessible_name"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

fn node_geometry(n: &Value) -> Option<(i64, i64, i64, i64)> {
    let g = n.get("geometry")?;
    // Tolerate both nested {bounds:{...}} and flat {x,y,...}.
    let b = g.get("bounds").unwrap_or(g);
    let x = b.get("x")?.as_i64()?;
    let y = b.get("y")?.as_i64()?;
    let w = b.get("width")?.as_i64()?;
    let h = b.get("height")?.as_i64()?;
    Some((x, y, w, h))
}

fn node_id(n: &Value) -> Option<i64> {
    n.get("id").and_then(|i| i.get("hash_code")).and_then(|v| v.as_i64())
}

fn node_children(n: &Value) -> &[Value] {
    match n.get("children").and_then(|c| c.as_array()) {
        Some(arr) => arr.as_slice(),
        None => &[],
    }
}

/// Attribute qualifiers for a node in ladder order, honoring `strip_names`.
fn qualifiers(n: &Value, strip_names: bool) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for attr in QUALIFIER_ORDER {
        let val = match attr {
            "name" => {
                if strip_names {
                    None
                } else {
                    node_name(n)
                }
            }
            "accessiblename" => node_accessible_name(n),
            "text" => node_text(n),
            "tooltip" => node_tooltip(n),
            _ => None,
        };
        if let Some(v) = val {
            out.push((attr, v));
        }
    }
    out
}

fn esc(v: &str) -> String {
    v.replace('\'', "\\'")
}

fn round_to(x: f64, places: i32) -> f64 {
    let f = 10f64.powi(places);
    (x * f).round() / f
}

fn score_of(locator: &str, stability: f64, has_semantic: bool, depth_gap: usize) -> f64 {
    let readability = if has_semantic { 1.0 } else { 0.4 };
    let brevity = 1.0 / (1.0 + locator.len() as f64 / 40.0);
    let anchor_locality = 1.0 / (1.0 + depth_gap as f64);
    round_to(
        0.45 * stability + 0.25 * readability + 0.20 * brevity + 0.10 * anchor_locality,
        4,
    )
}

// ---------------------------------------------------------------------------
// Candidate model
// ---------------------------------------------------------------------------
#[derive(Clone)]
struct Candidate {
    locator: String,
    strategy: &'static str,
    match_count: Option<i64>,
    unique: Option<bool>,
    stability: f64,
    score: f64,
    brittle_flags: Vec<String>,
    preconditions: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
fn make_candidate(
    locator: String,
    strategy: &'static str,
    stability: f64,
    brittle: &[&str],
    has_semantic: bool,
    depth_gap: usize,
    preconditions: &[&str],
) -> Candidate {
    let score = score_of(&locator, stability, has_semantic, depth_gap);
    Candidate {
        locator,
        strategy,
        match_count: None,
        unique: None,
        stability: round_to(stability, 3),
        score,
        brittle_flags: brittle.iter().map(|s| s.to_string()).collect(),
        preconditions: preconditions.iter().map(|s| s.to_string()).collect(),
    }
}

// ---------------------------------------------------------------------------
// A flattened record carrying ancestor chain + same-type sibling index.
// ---------------------------------------------------------------------------
struct FlatRecord<'a> {
    node: &'a Value,
    ancestors: Vec<&'a Value>,
    type_index: usize,
    node_id: Option<i64>,
}

fn flatten<'a>(
    node: &'a Value,
    parent: Option<&'a Value>,
    ancestors: &[&'a Value],
    out: &mut Vec<FlatRecord<'a>>,
) {
    let ty = node_type(node);
    let type_index = match parent {
        Some(p) => {
            let mut idx = 0usize;
            for sib in node_children(p) {
                if std::ptr::eq(sib, node) {
                    break;
                }
                if node_type(sib) == ty {
                    idx += 1;
                }
            }
            idx
        }
        None => 0,
    };
    out.push(FlatRecord {
        node,
        ancestors: ancestors.to_vec(),
        type_index,
        node_id: node_id(node),
    });
    let mut child_ancestors = ancestors.to_vec();
    child_ancestors.push(node);
    for child in node_children(node) {
        flatten(child, Some(node), &child_ancestors, out);
    }
}

// ---------------------------------------------------------------------------
// Candidate enumeration (unverified) — ordered best-first, parity w/ Python
// ---------------------------------------------------------------------------
fn enumerate_candidates(rec: &FlatRecord, strip_names: bool) -> Vec<Candidate> {
    let n = rec.node;
    let t = node_type(n);
    let mut out: Vec<Candidate> = Vec::new();

    let target_quals = qualifiers(n, strip_names);

    // Tier 1 — global single segment
    for (attr, val) in &target_quals {
        out.push(make_candidate(
            format!("{}[{}='{}']", t, attr, esc(val)),
            "single",
            stability_of(attr),
            &[],
            true,
            0,
            &[],
        ));
    }

    // Tier 2 — nearest-stable-ancestor anchored >> chain
    for (i, anc) in rec.ancestors.iter().rev().enumerate() {
        let depth_gap = i + 1;
        let anc_quals = qualifiers(anc, strip_names);
        if anc_quals.is_empty() {
            continue;
        }
        let at = node_type(anc);
        for (aa, av) in &anc_quals {
            let anchor = format!("{}[{}='{}']", at, aa, esc(av));
            let anc_stab = stability_of(aa);
            // anchored + a semantic target qualifier (preferred)
            for (ta, tv) in &target_quals {
                let stab = anc_stab.min(stability_of(ta));
                out.push(make_candidate(
                    format!("{} >> {}[{}='{}']", anchor, t, ta, esc(tv)),
                    "anchored",
                    stab,
                    &[],
                    true,
                    depth_gap,
                    &[],
                ));
            }
            // anchor + bare type
            out.push(make_candidate(
                format!("{} >> {}", anchor, t),
                "anchored-bare",
                anc_stab.min(0.55),
                &[],
                !target_quals.is_empty(),
                depth_gap,
                &[],
            ));
            // anchor + nth-of-type (structural; brittle)
            out.push(make_candidate(
                format!("{} >> {}:nth-of-type({})", anchor, t, rec.type_index + 1),
                "anchored-nth",
                anc_stab.min(stability_of("nth-of-type")),
                &["sibling-index"],
                false,
                depth_gap,
                &[],
            ));
        }
        break; // nearest stable anchor only
    }

    // Tier 3 — geometry fallback (always flagged brittle)
    if let Some((x, y, w, h)) = node_geometry(n) {
        out.push(make_candidate(
            format!("{}[width='{}'][height='{}'][x='{}'][y='{}']", t, w, h, x, y),
            "geometry",
            stability_of("geometry"),
            &["resize/relayout will break this"],
            false,
            0,
            &[],
        ));
    }

    out
}

// ---------------------------------------------------------------------------
// Offline oracle: build owned UIComponents and verify with the production matcher
// ---------------------------------------------------------------------------
fn build_component(json: &Value) -> Option<UIComponent> {
    let simple_name = node_type(json);
    let class_name = node_class(json).unwrap_or_else(|| simple_name.clone());
    let hash_code = node_id(json).unwrap_or(0);
    let (x, y, w, h) = node_geometry(json).unwrap_or((0, 0, 0, 0));

    let children: Option<Vec<UIComponent>> = {
        let kids: Vec<UIComponent> = node_children(json).iter().filter_map(build_component).collect();
        if kids.is_empty() {
            None
        } else {
            Some(kids)
        }
    };

    let idy = json.get("identity");
    let get_id_str = |key: &str| {
        idy.and_then(|i| i.get(key))
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
    };

    Some(UIComponent {
        id: ComponentId {
            hash_code,
            tree_path: hash_code.to_string(),
            depth: 0,
        },
        component_type: ComponentType {
            class_name: class_name.clone(),
            simple_name,
            base_type: SwingBaseType::Unknown,
            interfaces: Vec::new(),
            class_hierarchy: vec![class_name],
        },
        identity: ComponentIdentity {
            name: get_id_str("name"),
            text: get_id_str("text"),
            internal_name: get_id_str("internal_name"),
            title: get_id_str("title"),
            tooltip: get_id_str("tooltip"),
            label_text: get_id_str("label_text"),
            action_command: get_id_str("action_command"),
        },
        geometry: ComponentGeometry {
            bounds: Bounds {
                x: x as i32,
                y: y as i32,
                width: w as i32,
                height: h as i32,
            },
            local_bounds: None,
            preferred_size: None,
            minimum_size: None,
            maximum_size: None,
        },
        state: ComponentState::default(),
        properties: ComponentProperties::default(),
        accessibility: AccessibilityInfo {
            accessible_name: node_accessible_name(json),
            ..AccessibilityInfo::default()
        },
        children,
        parent_id: None,
        metadata: TraversalMetadata::default(),
    })
}

/// Resolve a candidate offline. Returns the matched node ids (hash codes), or
/// `None` if the candidate string does not parse (candidate is then skipped).
fn resolve(locator_str: &str, roots: &[UIComponent], evaluator: &Evaluator) -> Option<Vec<i64>> {
    let locator = parse_locator(locator_str).ok()?;
    let mut ids: Vec<i64> = Vec::new();
    for root in roots {
        for m in find_matching_components(&locator, root, evaluator) {
            let id = m.id.hash_code;
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    Some(ids)
}

fn suggest(
    tree: &Value,
    target_id: i64,
    strip_names: bool,
    top: usize,
) -> Vec<Candidate> {
    // Build the flat record list across the whole forest.
    let mut flat: Vec<FlatRecord> = Vec::new();
    if let Some(roots) = tree.get("roots").and_then(|r| r.as_array()) {
        for root in roots {
            flatten(root, None, &[], &mut flat);
        }
    }

    // Locate the target record.
    let target = match flat.iter().find(|r| r.node_id == Some(target_id)) {
        Some(t) => t,
        None => return Vec::new(),
    };

    // Build owned components for the offline oracle (once).
    let owned_roots: Vec<UIComponent> = tree
        .get("roots")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(build_component).collect())
        .unwrap_or_default();
    let evaluator = Evaluator::new();

    let mut unique_ok: Vec<Candidate> = Vec::new();
    let mut best_effort: Option<Candidate> = None;

    for mut cand in enumerate_candidates(target, strip_names) {
        let ids = match resolve(&cand.locator, &owned_roots, &evaluator) {
            Some(v) => v,
            None => continue,
        };
        cand.match_count = Some(ids.len() as i64);
        let is_unique = ids.len() == 1 && ids[0] == target_id;
        cand.unique = Some(is_unique);

        if is_unique {
            unique_ok.push(cand);
            if unique_ok.len() >= top * 2 {
                break;
            }
        } else {
            let better = best_effort
                .as_ref()
                .map(|b| cand.score > b.score)
                .unwrap_or(true);
            if better && !cand.brittle_flags.iter().any(|f| f == "ambiguous") {
                let mut be = cand.clone();
                be.brittle_flags.push("not-unique".to_string());
                best_effort = Some(be);
            }
        }
    }

    unique_ok.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.locator.len().cmp(&b.locator.len()))
    });

    if !unique_ok.is_empty() {
        unique_ok.truncate(top);
        unique_ok
    } else {
        best_effort.into_iter().collect()
    }
}

fn candidate_to_dict(py: Python<'_>, c: &Candidate) -> PyResult<PyObject> {
    let d = PyDict::new(py);
    d.set_item("locator", &c.locator)?;
    d.set_item("strategy", c.strategy)?;
    match c.match_count {
        Some(n) => d.set_item("match_count", n)?,
        None => d.set_item("match_count", py.None())?,
    }
    match c.unique {
        Some(b) => d.set_item("unique", b)?,
        None => d.set_item("unique", py.None())?,
    }
    d.set_item("stability", c.stability)?;
    d.set_item("score", c.score)?;
    d.set_item("brittle_flags", PyList::new(py, &c.brittle_flags))?;
    d.set_item("preconditions", PyList::new(py, &c.preconditions))?;
    Ok(d.into())
}

// ---------------------------------------------------------------------------
// PyO3 exports
// ---------------------------------------------------------------------------

/// Suggest ranked, offline-verified Robot Framework locators for a target node.
///
/// `tree_json` is a get_ui_tree JSON payload (`{roots:[...]}`). `node_id` is the
/// target node's `id.hash_code`. When `strip_names` is true, `name=` qualifiers are
/// omitted (useful to test resilience against unstable programmatic names). Returns up
/// to `top` candidate dicts, best first. Each candidate is verified OFFLINE against the
/// parsed tree using the production matcher, so `unique`/`match_count` reflect the exact
/// behavior of the live locator engine — no RPC per candidate.
#[pyfunction]
#[pyo3(signature = (tree_json, node_id, strip_names = false, top = 3))]
pub fn suggest_locators(
    py: Python<'_>,
    tree_json: &str,
    node_id: i64,
    strip_names: bool,
    top: usize,
) -> PyResult<PyObject> {
    let tree: Value = serde_json::from_str(tree_json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("invalid tree JSON: {}", e)))?;
    let candidates = suggest(&tree, node_id, strip_names, top);
    let list = PyList::empty(py);
    for c in &candidates {
        list.append(candidate_to_dict(py, c)?)?;
    }
    Ok(list.into())
}

/// Parse a locator and explain the result.
///
/// Returns a dict `{ok, position, hint, message}`:
/// - `ok=true` for a parseable locator (`position`/`hint`/`message` are `None`).
/// - `ok=false` otherwise: `position` is the byte offset of the failure, `hint` is a
///   machine-readable "expected: ..." list of valid tokens, and `message` is the full
///   human-readable error.
#[pyfunction]
pub fn explain_locator(py: Python<'_>, locator: &str) -> PyResult<PyObject> {
    let d = PyDict::new(py);
    match parse_locator(locator) {
        Ok(_) => {
            d.set_item("ok", true)?;
            d.set_item("position", py.None())?;
            d.set_item("hint", py.None())?;
            d.set_item("message", py.None())?;
        }
        Err(e) => {
            d.set_item("ok", false)?;
            d.set_item("position", e.position)?;
            let hint = if e.expected.is_empty() {
                format!("unknown token; expected: {}", "name,text,tooltip,type,>,>>,[")
            } else {
                format!("unknown token; expected: {}", e.expected.join(","))
            };
            d.set_item("hint", hint)?;
            d.set_item("message", e.to_string())?;
        }
    }
    Ok(d.into())
}
