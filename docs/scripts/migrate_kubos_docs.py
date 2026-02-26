#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SRC_ROOT = REPO_ROOT / "kubos" / "docs"
DST_ROOT = REPO_ROOT / "docs" / "content" / "docs" / "kubos"

SPECIAL_DEST_NAMES = {
    # Preserve the hand-authored Kubos section landing page at kubos/index.mdx.
    Path("index.rst"): "legacy-sphinx-home.mdx",
}

ANCHOR_MAP: dict[str, str] = {}

TOCTREE_BLOCK_RE = re.compile(r"\n*<div class=\"toctree\"[^>]*>.*?</div>\n*", re.S)
UML_BLOCK_RE = re.compile(r"\n*<div class=\"uml\">\s*(.*?)\s*</div>\n*", re.S)
SPHINX_INLINE_LINK_RE = re.compile(r"`([^`<>]+?)\s*<([^`<>]+?)>`")
BARE_PATH_CODE_RE = re.compile(r"`((?:\.\./)?[A-Za-z0-9._-]+(?:/[A-Za-z0-9._-]+)+)`")
ANCHOR_DEF_RE = re.compile(r"^\s*\.\.\s+_([A-Za-z0-9._:-]+):\s*$")
FRONTMATTER_TITLE_RE = re.compile(r'^title:\s*"?(.+?)"?\s*$')
FENCED_BLOCK_RE = re.compile(r"(^```[^\n]*\n.*?^```\s*$)", re.M | re.S)
FENCE_OPEN_RE = re.compile(r"^```[ \t]*([^\s`]+)?[ \t]*$")
ANGLE_TOKEN_RE = re.compile(r"<([^>\n]+)>")

HTML_TAG_WHITELIST = {
    "a",
    "abbr",
    "b",
    "blockquote",
    "br",
    "code",
    "div",
    "em",
    "figcaption",
    "figure",
    "i",
    "img",
    "kbd",
    "li",
    "ol",
    "p",
    "pre",
    "span",
    "strong",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "th",
    "thead",
    "tr",
    "ul",
}


def run_pandoc(src: Path) -> str:
    rst_text = src.read_text(encoding="utf-8")
    rst_text, uml_blocks = preprocess_rst_uml_blocks(rst_text)
    proc = subprocess.run(
        ["pandoc", "--wrap=none", "-f", "rst", "-t", "gfm", "-"],
        check=True,
        capture_output=True,
        text=True,
        input=rst_text,
        cwd=src.parent,
    )
    return restore_uml_placeholders(proc.stdout, uml_blocks)


def normalize_link(target: str) -> str:
    target = target.strip().replace("\\<", "<").replace("\\>", ">")
    if target.startswith(("http://", "https://", "mailto:", "#")):
        return target
    if target in ANCHOR_MAP:
        return ANCHOR_MAP[target]
    if target.endswith(".rst"):
        target = target[:-4]
    if target in ANCHOR_MAP:
        return ANCHOR_MAP[target]
    return target


def rewrite_image_paths(text: str) -> str:
    def repl_html(match: re.Match[str]) -> str:
        prefix, src, suffix = match.groups()
        src = normalize_image_src(src)
        return f'{prefix}{src}{suffix}'

    def repl_md(match: re.Match[str]) -> str:
        alt, src = match.groups()
        src = normalize_image_src(src)
        return f"![{alt}]({src})"

    text = re.sub(r'(<img\s+[^>]*src=")([^"]+)(")', repl_html, text)
    text = re.sub(r"!\[([^\]]*)\]\(([^)]+)\)", repl_md, text)
    return text


def normalize_image_src(src: str) -> str:
    cleaned = src.strip()
    marker = "images/"
    idx = cleaned.find(marker)
    if idx >= 0:
        return f"/kubos/images/{cleaned[idx + len(marker):]}"
    return cleaned


def clean_markdown(md: str) -> str:
    md = TOCTREE_BLOCK_RE.sub("\n\n", md)
    md = UML_BLOCK_RE.sub(lambda m: "\n\n```text\n" + m.group(1).strip() + "\n```\n\n", md)
    md = md.replace("\\<", "<").replace("\\>", ">")

    def inline_link_repl(match: re.Match[str]) -> str:
        text, target = match.groups()
        text = " ".join(text.split())
        return f"[{text}]({normalize_link(target)})"

    md = SPHINX_INLINE_LINK_RE.sub(inline_link_repl, md)

    def bare_path_repl(match: re.Match[str]) -> str:
        target = match.group(1)
        return f"[{target}]({normalize_link(target)})"

    md = BARE_PATH_CODE_RE.sub(bare_path_repl, md)

    for anchor, href in sorted(ANCHOR_MAP.items()):
        md = md.replace(f"`{anchor}`", f"[{anchor}]({href})")

    # Clean up common pandoc Sphinx artifacts that are noisy in MDX.
    md = re.sub(r"^\s*<div class=\"contents\".*?</div>\s*$", "", md, flags=re.M | re.S)
    md = rewrite_image_paths(md)
    md = sanitize_for_mdx(md)
    md = re.sub(r"\n{3,}", "\n\n", md).strip() + "\n"
    return md


def extract_title_and_body(md: str) -> tuple[str, str]:
    lines = md.splitlines()
    if lines and lines[0].startswith("# "):
        title = lines[0][2:].strip()
        body = "\n".join(lines[1:]).lstrip("\n")
        return title, body
    return "KubOS Doc", md


def add_frontmatter(title: str, body: str) -> str:
    return f"---\ntitle: {json.dumps(title)}\n---\n\n{body.rstrip()}\n"


def destination_for(rel_src: str | Path) -> Path:
    rel = Path(rel_src)
    if rel in SPECIAL_DEST_NAMES:
        return DST_ROOT / SPECIAL_DEST_NAMES[rel]
    if rel.name == "index.rst":
        return DST_ROOT / rel.parent / "index.mdx"
    return DST_ROOT / rel.with_suffix(".mdx")


def route_for_rel_src(rel_src: str | Path) -> str:
    dst = destination_for(rel_src)
    rel = dst.relative_to(DST_ROOT).with_suffix("")
    parts = list(rel.parts)
    if parts and parts[-1] == "index":
        parts = parts[:-1]
    route = "/docs/kubos"
    if parts:
        route += "/" + "/".join(parts)
    return route


def all_rst_sources() -> list[Path]:
    return sorted(path.relative_to(SRC_ROOT) for path in SRC_ROOT.rglob("*.rst"))


def parse_toctree_entries(src: Path) -> list[tuple[str, str]]:
    entries: list[tuple[str, str]] = []
    lines = src.read_text(encoding="utf-8").splitlines()
    i = 0
    while i < len(lines):
        if lines[i].strip().startswith(".. toctree::"):
            i += 1
            while i < len(lines):
                line = lines[i]
                stripped = line.strip()
                if not line.startswith((" ", "\t")) and stripped:
                    break
                if not stripped or stripped.startswith(":"):
                    i += 1
                    continue
                entry = stripped
                m = re.match(r"(.+?)\s*<([^>]+)>$", entry)
                if m:
                    label = " ".join(m.group(1).split())
                    target = normalize_link(m.group(2))
                else:
                    target = normalize_link(entry)
                    label = entry
                entries.append((label, target))
                i += 1
            continue
        i += 1
    return entries


def append_toctree_contents(body: str, entries: list[tuple[str, str]]) -> str:
    if not entries:
        return body
    lines = ["", "## Pages in This Section", ""]
    for label, target in entries:
        lines.append(f"- [{label}]({target})")
    return body.rstrip() + "\n" + "\n".join(lines) + "\n"


def preprocess_rst_uml_blocks(rst_text: str) -> tuple[str, list[str]]:
    lines = rst_text.splitlines()
    out: list[str] = []
    uml_blocks: list[str] = []
    i = 0

    while i < len(lines):
        line = lines[i]
        directive = re.match(r"^(\s*)\.\.\s+uml::\s*$", line)
        if not directive:
            out.append(line)
            i += 1
            continue

        base_indent = len(directive.group(1))
        j = i + 1
        block_lines: list[str] = []
        content_indent: int | None = None

        # Consume directive content (blank lines + indented UML text) until dedent.
        while j < len(lines):
            nxt = lines[j]
            if not nxt.strip():
                block_lines.append("")
                j += 1
                continue

            indent = len(nxt) - len(nxt.lstrip(" "))
            if indent <= base_indent:
                break

            if content_indent is None:
                content_indent = indent
            if indent < content_indent:
                # Keep at least the minimum indentation we discovered.
                content_indent = indent
            block_lines.append(nxt)
            j += 1

        if content_indent is None:
            # No content found; keep original line as fallback.
            out.append(line)
            i += 1
            continue

        normalized: list[str] = []
        for raw in block_lines:
            if raw == "":
                normalized.append("")
                continue
            normalized.append(raw[content_indent:])

        uml = "\n".join(normalized).strip("\n")
        placeholder = f"KUBOS_UML_PLACEHOLDER_{len(uml_blocks)}"
        uml_blocks.append(uml)

        # Surround with blank lines so pandoc preserves it as an isolated paragraph.
        if out and out[-1] != "":
            out.append("")
        out.append(directive.group(1) + placeholder)
        out.append("")

        i = j

    result = "\n".join(out)
    if rst_text.endswith("\n"):
        result += "\n"
    return result, uml_blocks


def restore_uml_placeholders(md: str, uml_blocks: list[str]) -> str:
    for idx, uml in enumerate(uml_blocks):
        placeholder = f"KUBOS_UML_PLACEHOLDER_{idx}"
        fenced = f"```text\n{uml}\n```"
        md = re.sub(
            rf"(?m)^[ \t]*{re.escape(placeholder)}[ \t]*$",
            lambda _m, fenced=fenced: fenced,
            md,
        )
    return md


def sanitize_for_mdx(text: str) -> str:
    parts = FENCED_BLOCK_RE.split(text)
    out: list[str] = []
    for part in parts:
        if not part:
            continue
        if part.startswith("```"):
            out.append(sanitize_fenced_block(part))
        else:
            out.append(sanitize_text_segment(convert_indented_code_blocks(part)))
    return "".join(out)


def sanitize_fenced_block(block: str) -> str:
    lines = block.splitlines()
    if not lines:
        return block
    m = FENCE_OPEN_RE.match(lines[0])
    if m:
        lang = (m.group(1) or "").strip().lower()
        if lang in {"plantuml", "none", "math"}:
            lines[0] = "```text"
    return "\n".join(lines) + ("\n" if block.endswith("\n") else "")


def convert_indented_code_blocks(segment: str) -> str:
    lines = segment.splitlines()
    if not lines:
        return segment

    out: list[str] = []
    i = 0

    def leading_spaces(s: str) -> int:
        return len(s) - len(s.lstrip(" "))

    def looks_like_list_item(s: str) -> bool:
        stripped = s.lstrip()
        return bool(re.match(r"(?:[-+*]\s+|\d+[.)]\s+)", stripped))

    while i < len(lines):
        line = lines[i]
        prev_line = lines[i - 1] if i > 0 else ""

        if (
            line.startswith("    ")
            and not looks_like_list_item(line)
            and (i == 0 or not prev_line.strip())
        ):
            j = i
            block_lines: list[str] = []
            indents: list[int] = []
            while j < len(lines):
                current = lines[j]
                if not current.strip():
                    block_lines.append(current)
                    j += 1
                    continue
                if not current.startswith("    "):
                    break
                if looks_like_list_item(current):
                    break
                block_lines.append(current)
                indents.append(leading_spaces(current))
                j += 1

            if indents:
                common = min(indents)
                if common >= 4:
                    context_indent = " " * (common - 4)
                    out.append(f"{context_indent}```")
                    for raw in block_lines:
                        if not raw.strip():
                            out.append(context_indent)
                        else:
                            out.append(context_indent + raw[common:])
                    out.append(f"{context_indent}```")
                    i = j
                    continue

        out.append(line)
        i += 1

    converted = "\n".join(out)
    if segment.endswith("\n"):
        converted += "\n"
    return converted


def sanitize_text_segment(segment: str) -> str:
    segment = rewrite_jsx_html_attrs(segment)
    segment = fix_malformed_markdown_links(segment)
    segment = ANGLE_TOKEN_RE.sub(replace_angle_token, segment)
    segment = segment.replace("<<<", "&lt;&lt;&lt;")
    segment = segment.replace("<>", "&lt;&gt;")
    segment = re.sub(r"<(?=[0-9-])", "&lt;", segment)
    # MDX treats braces as expressions even in some legacy markdown contexts (for example
    # indented code blocks), so escape them in prose segments.
    segment = segment.replace("{", r"\{").replace("}", r"\}")
    return segment


def replace_angle_token(match: re.Match[str]) -> str:
    token = match.group(1).strip()
    lower = token.lower()

    if lower.startswith(("http://", "https://")):
        return f"[{token}]({token})"
    if lower.startswith("mailto:"):
        email = token[len("mailto:") :]
        return f"[{email}]({token})"
    if "@" in token and " " not in token and "/" not in token and ":" not in token:
        return f"[{token}](mailto:{token})"

    tag_match = re.match(r"^/?([A-Za-z][A-Za-z0-9:-]*)(?:\s|/|$)", token)
    if tag_match and tag_match.group(1).lower() in HTML_TAG_WHITELIST:
        return match.group(0)

    return f"&lt;{token}&gt;"


def fix_malformed_markdown_links(segment: str) -> str:
    # Handle malformed link conversions like:
    # [Forcibly downgrading pip to](v10 ... <https://...>)
    pattern = re.compile(r"\[([^\]]+)\]\(([^)\n]*?)\s+<?((?:https?|http)://[^)\s>]+)>?\)")

    def repl(match: re.Match[str]) -> str:
        text, extra, url = match.groups()
        extra = " ".join(extra.split())
        merged_text = f"{text} {extra}".strip()
        return f"[{merged_text}]({url})"

    return pattern.sub(repl, segment)


def rewrite_jsx_html_attrs(segment: str) -> str:
    replacements = [
        (r"(<[A-Za-z][^>]*?)\bclass=", r"\1className="),
        (r"(<[A-Za-z][^>]*?)\bfor=", r"\1htmlFor="),
        (r"(<[A-Za-z][^>]*?)\bcolspan=", r"\1colSpan="),
        (r"(<[A-Za-z][^>]*?)\browspan=", r"\1rowSpan="),
        (r"(<[A-Za-z][^>]*?)\btabindex=", r"\1tabIndex="),
    ]
    for pattern, repl in replacements:
        segment = re.sub(pattern, repl, segment)
    return segment


def parse_anchor_labels(src: Path) -> list[str]:
    labels: list[str] = []
    for line in src.read_text(encoding="utf-8").splitlines():
        m = ANCHOR_DEF_RE.match(line)
        if m:
            labels.append(m.group(1))
    return labels


def build_anchor_map(rel_sources: list[Path]) -> dict[str, str]:
    anchor_map: dict[str, str] = {}
    for rel_src in rel_sources:
        src = SRC_ROOT / rel_src
        route = route_for_rel_src(rel_src)
        for label in parse_anchor_labels(src):
            anchor_map[label] = f"{route}#{label}"
    return anchor_map


def titleize_slug(slug: str) -> str:
    return " ".join(part.upper() if part.isupper() else part.capitalize() for part in slug.replace("_", "-").split("-"))


def read_existing_meta(path: Path) -> dict | None:
    if not path.exists():
        return None
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return None


def read_frontmatter_title(path: Path) -> str | None:
    if not path.exists():
        return None
    lines = path.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].strip() != "---":
        return None
    for line in lines[1:20]:
        if line.strip() == "---":
            break
        m = FRONTMATTER_TITLE_RE.match(line)
        if m:
            return m.group(1)
    return None


def immediate_doc_children(dir_path: Path) -> list[str]:
    names: list[str] = []
    for child in sorted(dir_path.iterdir(), key=lambda p: p.name):
        if child.name == "meta.json":
            continue
        if child.is_file() and child.suffix == ".mdx":
            names.append(child.stem)
        elif child.is_dir():
            has_docs = any(
                p.suffix == ".mdx" and p.name != "meta.json"
                for p in child.rglob("*.mdx")
            )
            if has_docs:
                names.append(child.name)
    # de-duplicate while preserving order
    seen = set()
    result = []
    for name in names:
        if name not in seen:
            seen.add(name)
            result.append(name)
    return result


def source_dir_for_dst_dir(dst_dir: Path) -> Path:
    rel = dst_dir.relative_to(DST_ROOT)
    return SRC_ROOT / rel


def toctree_immediate_order(dst_dir: Path, child_names: list[str]) -> list[str]:
    src_dir = source_dir_for_dst_dir(dst_dir)
    src_index = src_dir / "index.rst"
    if not src_index.exists():
        return []
    valid = set(child_names)
    order: list[str] = []
    for _, target in parse_toctree_entries(src_index):
        parts = [p for p in Path(target).parts if p not in (".", "")]
        if not parts:
            continue
        first = parts[0]
        name = Path(first).stem
        if name in valid and name not in order:
            order.append(name)
    return order


def merge_page_order(existing_pages: list[str], preferred: list[str], available: list[str]) -> list[str]:
    available_set = set(available)
    out: list[str] = []
    for name in existing_pages:
        if name in available_set and name not in out:
            out.append(name)
    for name in preferred:
        if name in available_set and name not in out:
            out.append(name)
    for name in sorted(available):
        if name not in out:
            out.append(name)
    return out


def write_meta_files() -> list[Path]:
    written: list[Path] = []
    dirs = [DST_ROOT] + sorted([p for p in DST_ROOT.rglob("*") if p.is_dir()])
    for dst_dir in dirs:
        child_names = immediate_doc_children(dst_dir)
        if not child_names:
            continue

        preferred = []
        if "index" in child_names:
            preferred.append("index")
        preferred.extend(toctree_immediate_order(dst_dir, child_names))

        meta_path = dst_dir / "meta.json"
        existing = read_existing_meta(meta_path) or {}
        pages = merge_page_order(existing.get("pages", []), preferred, child_names)

        meta = dict(existing)
        meta["pages"] = pages

        if "title" not in meta:
            if dst_dir == DST_ROOT:
                meta["title"] = "Kubos Docs"
            else:
                meta["title"] = read_frontmatter_title(dst_dir / "index.mdx") or titleize_slug(dst_dir.name)

        meta_path.write_text(json.dumps(meta, indent=2) + "\n", encoding="utf-8")
        written.append(meta_path)
    return written


def main() -> None:
    rel_sources = all_rst_sources()
    global ANCHOR_MAP
    ANCHOR_MAP = build_anchor_map(rel_sources)

    generated: list[Path] = []
    failed: list[tuple[Path, str]] = []

    for rel_src in rel_sources:
        src = SRC_ROOT / rel_src
        dst = destination_for(rel_src)
        dst.parent.mkdir(parents=True, exist_ok=True)
        try:
            raw_md = run_pandoc(src)
            clean_md = clean_markdown(raw_md)
            title, body = extract_title_and_body(clean_md)
            body = append_toctree_contents(body, parse_toctree_entries(src))
            content = add_frontmatter(title, body)
            dst.write_text(content, encoding="utf-8")
            generated.append(dst)
        except Exception as exc:  # pragma: no cover - migration helper
            failed.append((rel_src, str(exc)))

    meta_written = write_meta_files()

    print("Generated:")
    for path in generated[:200]:
        print(path.relative_to(REPO_ROOT))
    if len(generated) > 200:
        print(f"... and {len(generated) - 200} more")

    print(f"\nGenerated pages: {len(generated)}")
    print(f"Updated meta files: {len(meta_written)}")

    if failed:
        print("\nFailed:")
        for rel_src, err in failed:
            print(f"- {rel_src}: {err}")
        sys.exit(1)


if __name__ == "__main__":
    main()
