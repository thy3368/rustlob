pub(crate) fn concat2(a: &str, b: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()));
    out.push_str(a);
    out.push_str(b);
    out
}

pub(crate) fn concat3(a: &str, b: &str, c: &str) -> String {
    let mut out = String::with_capacity(a.len().saturating_add(b.len()).saturating_add(c.len()));
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out
}

pub(crate) fn concat4(a: &str, b: &str, c: &str, d: &str) -> String {
    let mut out = String::with_capacity(
        a.len().saturating_add(b.len()).saturating_add(c.len()).saturating_add(d.len()),
    );
    out.push_str(a);
    out.push_str(b);
    out.push_str(c);
    out.push_str(d);
    out
}
