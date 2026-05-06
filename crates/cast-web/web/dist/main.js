// cast-web frontend entry. Plain TypeScript, no framework, no
// bundler — `tsc` compiles `web/src/*.ts` to `web/dist/*.js`
// one-to-one, and the browser loads them as native ES modules
// via `<script type="module" src="/static/main.js">`.
//
// Three planes:
//  - Concept list (left)         — WebSocket to cast-watch via `/ws`,
//                                   `query from:concepts dim:full`.
//  - Concept detail (middle)     — HTTP `/watcher/query` on click;
//                                   `walk` for one node + radius
//                                   neighbours, or `tree` for the
//                                   canonical concept tree when no
//                                   concept is selected.
//  - Right column (two stacked sections):
//      • user/assistant chat     — `/mailbox/{messages,meta,stream,send,clear}`.
//      • watcher log             — `/watcher/log/{entries,stream}`.
//
// Top-level functions are intentionally anchor-friendly: each
// `init*()` and the `refresh*()` / `render*()` helpers are
// addressable from Rust via
//   `cast::io::continues_in! { target: "crates/cast-web/web/src/main.ts",
//                               lang: typescript, anchor: "<name>", … }`
// (the `typescript` backend resolves `function` / `interface` /
// lexical-`const` declarations).
const STATE = {
    selected: null,
    radius: 2,
    dim: 'full',
    conceptNames: [],
    concepts: {},
    unresolvedByFileLine: {},
    unresolvedAnchorTextsByFileLine: {},
    errorsByFileLineText: {},
};
// ── Entry ─────────────────────────────────────────────────────────────
export function main() {
    initConcepts();
    initDetailControls();
    initMailbox();
    initWatcherLog();
}
// ── DOM helpers ───────────────────────────────────────────────────────
function byId(id) {
    return document.getElementById(id);
}
function setEmpty(list, text) {
    list.replaceChildren();
    list.appendChild(emptyEl(text));
}
function emptyEl(text) {
    const li = document.createElement('li');
    li.className = 'empty';
    li.textContent = text;
    return li;
}
function fieldLabel(text) {
    const div = document.createElement('div');
    div.className = 'field-label';
    div.textContent = text;
    return div;
}
function shortFile(path) {
    if (!path)
        return '';
    const i = path.indexOf('crates/');
    if (i >= 0)
        return path.slice(i);
    const j = path.indexOf('apps/');
    if (j >= 0)
        return path.slice(j);
    return path;
}
function anchorPath(a) {
    return typeof a === 'string' ? a : a.path;
}
function anchorRole(a) {
    return typeof a === 'string' ? null : a.role;
}
// ── Concepts (WebSocket → cast-watch via /ws) ─────────────────────────
function initConcepts() {
    const list = byId('concepts');
    if (!list)
        return;
    const url = new URL('/ws', window.location.href);
    url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new WebSocket(url.toString());
    ws.addEventListener('open', () => {
        ws.send(JSON.stringify({ kind: 'help' }));
        ws.send(JSON.stringify({ kind: 'status' }));
        ws.send(JSON.stringify({ kind: 'manual' }));
        // dim: full because we need anchors (for the per-anchor outcome
        // markers) and edges (for the tree view's resolution).
        // Costs ~25KB on a ~20-concept workspace; acceptable for one-shot.
        ws.send(JSON.stringify({ kind: 'query', from: 'concepts', dim: 'full' }));
    });
    ws.addEventListener('message', (ev) => {
        let frame;
        try {
            frame = JSON.parse(ev.data);
        }
        catch {
            return;
        }
        const body = frame.body;
        if (!body || body.kind !== 'query_result')
            return;
        const qr = body;
        if (qr.stream !== 'concepts' || !qr.concepts)
            return;
        STATE.concepts = qr.concepts;
        renderConcepts(list, Object.keys(qr.concepts).sort());
        void refreshConceptStatus(list).then(() => {
            // First load: show the canonical concept tree in the middle pane.
            if (!STATE.selected)
                void showCanonicalTree();
        });
    });
    ws.addEventListener('close', () => {
        setEmpty(list, 'connection closed');
    });
}
function renderConcepts(list, names) {
    STATE.conceptNames = names;
    list.replaceChildren();
    if (names.length === 0) {
        setEmpty(list, 'no concepts');
        return;
    }
    for (const name of names) {
        const li = document.createElement('li');
        li.className = 'clickable';
        li.dataset.concept = name;
        if (name === STATE.selected)
            li.classList.add('selected');
        const pip = document.createElement('span');
        pip.className = 'status loading';
        li.appendChild(pip);
        li.appendChild(document.createTextNode(name));
        li.addEventListener('click', () => selectConcept(name));
        list.appendChild(li);
    }
}
async function refreshConceptStatus(list) {
    let body;
    try {
        const res = await fetch('/watcher/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                kind: 'query',
                from: 'paths',
                where: { outcome: 'unresolved' },
            }),
        });
        const env = (await res.json());
        body = env.body;
    }
    catch {
        return;
    }
    const paths = body?.paths ?? [];
    const counts = {};
    const texts = {};
    const errors = {};
    for (const p of paths) {
        const key = `${p.file}:${p.line}`;
        counts[key] = (counts[key] ?? 0) + 1;
        let textSet = texts[key];
        if (!textSet) {
            textSet = new Set();
            texts[key] = textSet;
        }
        textSet.add(p.text);
        let errMap = errors[key];
        if (!errMap) {
            errMap = {};
            errors[key] = errMap;
        }
        if (p.error)
            errMap[p.text] = p.error;
    }
    STATE.unresolvedByFileLine = counts;
    STATE.unresolvedAnchorTextsByFileLine = texts;
    STATE.errorsByFileLineText = errors;
    for (const li of Array.from(list.querySelectorAll('li.clickable'))) {
        const name = li.dataset.concept;
        if (!name)
            continue;
        const entry = STATE.concepts[name];
        let total = 0;
        if (entry?.declarations) {
            for (const d of entry.declarations) {
                const key = `${d.file}:${d.line}`;
                total += counts[key] ?? 0;
            }
        }
        const pip = li.querySelector('.status');
        if (!pip)
            continue;
        pip.classList.remove('loading');
        if (total > 0) {
            pip.classList.add('unresolved');
            pip.classList.remove('ok');
            let badge = li.querySelector('.status-count');
            if (!badge) {
                badge = document.createElement('span');
                badge.className = 'status-count';
                li.appendChild(badge);
            }
            badge.textContent = `${total} unresolved`;
        }
        else {
            pip.classList.add('ok');
            pip.classList.remove('unresolved');
            const badge = li.querySelector('.status-count');
            if (badge)
                badge.remove();
        }
    }
}
// ── Concept detail (HTTP → cast-watch via /watcher/query) ─────────────
function initDetailControls() {
    const slider = byId('radius-slider');
    const value = byId('radius-value');
    const dim = byId('dim-select');
    if (!slider || !value || !dim)
        return;
    let radiusTimer;
    slider.addEventListener('input', () => {
        STATE.radius = parseFloat(slider.value);
        value.textContent = STATE.radius.toFixed(1);
        if (radiusTimer !== undefined)
            clearTimeout(radiusTimer);
        radiusTimer = window.setTimeout(() => {
            if (STATE.selected)
                void renderDetail(STATE.selected);
        }, 200);
    });
    dim.addEventListener('change', () => {
        STATE.dim = dim.value;
        if (STATE.selected)
            void renderDetail(STATE.selected);
    });
}
function selectConcept(name) {
    STATE.selected = name;
    for (const li of Array.from(document.querySelectorAll('#concepts li.clickable'))) {
        li.classList.toggle('selected', li.dataset.concept === name);
    }
    void renderDetail(name);
}
function deselectConcept() {
    STATE.selected = null;
    for (const li of Array.from(document.querySelectorAll('#concepts li.clickable'))) {
        li.classList.remove('selected');
    }
    void showCanonicalTree();
}
async function renderDetail(name) {
    const pane = byId('detail');
    if (!pane)
        return;
    pane.replaceChildren();
    const back = document.createElement('button');
    back.className = 'back-button';
    back.textContent = '← back to tree';
    back.addEventListener('click', deselectConcept);
    pane.appendChild(back);
    pane.appendChild(emptyEl(`loading ${name}…`));
    let body;
    try {
        const res = await fetch('/watcher/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                kind: 'walk',
                from: [name],
                hops: 0,
                radius: STATE.radius,
                dim: STATE.dim,
            }),
        });
        const env = (await res.json());
        body = env.body;
    }
    catch (e) {
        pane.replaceChildren(back, emptyEl(`request failed: ${String(e)}`));
        return;
    }
    if (!body || body.kind !== 'walk_result') {
        pane.replaceChildren(back, emptyEl(`unexpected response: ${body?.kind ?? 'none'}`));
        return;
    }
    pane.replaceChildren();
    pane.appendChild(back);
    const heading = document.createElement('h3');
    heading.textContent = name;
    pane.appendChild(heading);
    const visited = body.visited ?? {};
    const seed = visited[name];
    if (seed) {
        const decls = seed.declarations ?? [];
        const first = decls[0];
        if (first && first.summary) {
            const summary = document.createElement('div');
            summary.className = 'summary';
            summary.textContent = first.summary;
            pane.appendChild(summary);
        }
        if (decls.length) {
            pane.appendChild(fieldLabel('declarations'));
            const ul = document.createElement('ul');
            ul.className = 'bare';
            for (const d of decls) {
                const li = document.createElement('li');
                const fileSpan = document.createElement('span');
                fileSpan.textContent = `${shortFile(d.file)}:${d.line}`;
                li.appendChild(fileSpan);
                const anchors = d.anchors ?? [];
                if (anchors.length) {
                    const key = `${d.file}:${d.line}`;
                    const unresolvedTexts = STATE.unresolvedAnchorTextsByFileLine[key];
                    const errors = STATE.errorsByFileLineText[key] ?? {};
                    li.appendChild(document.createTextNode(' — '));
                    for (let i = 0; i < anchors.length; i++) {
                        const a = anchors[i];
                        if (a === undefined)
                            continue;
                        const path = anchorPath(a);
                        const role = anchorRole(a);
                        const isUnresolved = !!unresolvedTexts && unresolvedTexts.has(path);
                        const span = document.createElement('span');
                        const roleClass = role === 'primitive' ? ' anchor-primitive' : '';
                        span.className =
                            (isUnresolved ? 'anchor anchor-unresolved' : 'anchor anchor-ok') + roleClass;
                        const rolePrefix = role === 'primitive' ? '◇ ' : '';
                        span.textContent = `${isUnresolved ? '✗' : '✓'} ${rolePrefix}${path}`;
                        const errMsg = errors[path];
                        if (isUnresolved && errMsg) {
                            span.title = errMsg;
                        }
                        else {
                            span.title =
                                role === 'primitive' ? `resolved (primitive): ${path}` : `resolved: ${path}`;
                        }
                        li.appendChild(span);
                        if (i < anchors.length - 1)
                            li.appendChild(document.createTextNode(', '));
                    }
                }
                ul.appendChild(li);
            }
            pane.appendChild(ul);
        }
        const edges = seed.edges ?? [];
        if (edges.length) {
            pane.appendChild(fieldLabel('edges'));
            const ul = document.createElement('ul');
            ul.className = 'bare';
            for (const e of edges) {
                const li = document.createElement('li');
                li.appendChild(document.createTextNode(`${e.kind} → `));
                const targetSpan = document.createElement('span');
                targetSpan.textContent = e.target;
                const owners = ownersOfAnchor(e.target, e.kind);
                const firstOwner = owners[0];
                if (owners.length === 1 && firstOwner) {
                    targetSpan.className = 'neighbor';
                    targetSpan.title = `walk to ${firstOwner}`;
                    targetSpan.addEventListener('click', () => selectConcept(firstOwner));
                }
                else if (owners.length > 1 && firstOwner) {
                    targetSpan.className = 'neighbor';
                    targetSpan.title = `${owners.length} concepts share this anchor: ${owners.join(', ')}`;
                    targetSpan.addEventListener('click', () => selectConcept(firstOwner));
                }
                else {
                    // Foreign file (io_continues_in to a non-cast file) or
                    // unresolved continues_in — leave as plain text.
                    targetSpan.style.color = 'var(--fg-muted)';
                }
                li.appendChild(targetSpan);
                if (e.lang)
                    li.appendChild(document.createTextNode(` (${e.lang})`));
                ul.appendChild(li);
            }
            pane.appendChild(ul);
        }
    }
    const neighborNames = Object.keys(visited)
        .filter((n) => n !== name)
        .sort();
    if (neighborNames.length) {
        pane.appendChild(fieldLabel(`neighbors (radius=${STATE.radius.toFixed(1)})`));
        const ul = document.createElement('ul');
        ul.className = 'bare';
        for (const nb of neighborNames) {
            const li = document.createElement('li');
            const span = document.createElement('span');
            span.className = 'neighbor';
            span.textContent = nb;
            span.addEventListener('click', () => selectConcept(nb));
            li.appendChild(span);
            const decls = visited[nb]?.declarations ?? [];
            const sum = decls[0]?.summary;
            if (sum) {
                const t = document.createElement('span');
                t.className = 'tag';
                t.textContent = sum.length > 60 ? sum.slice(0, 60) + '…' : sum;
                li.appendChild(document.createTextNode(' '));
                li.appendChild(t);
            }
            ul.appendChild(li);
        }
        pane.appendChild(ul);
    }
    else if (STATE.radius > 0) {
        pane.appendChild(fieldLabel('neighbors'));
        const empty = document.createElement('div');
        empty.className = 'empty';
        empty.textContent = 'no graph neighbors at this radius';
        pane.appendChild(empty);
    }
}
// Resolve an edge target back to the concept(s) that own it as an
// anchor. Uses STATE.concepts (which has dim: full data) — for
// continues_in we match against d.anchors; for io_continues_in we
// match the target against d.file (foreign-file targets that ARE
// cast-annotated).
function ownersOfAnchor(target, kind) {
    const owners = [];
    for (const [name, entry] of Object.entries(STATE.concepts)) {
        for (const d of entry.declarations ?? []) {
            const anchorPaths = (d.anchors ?? []).map(anchorPath);
            if (kind === 'continues_in' && anchorPaths.includes(target)) {
                owners.push(name);
                break;
            }
            if (kind === 'io_continues_in' && d.file && d.file.endsWith(target)) {
                owners.push(name);
                break;
            }
        }
    }
    return owners;
}
// ── Canonical concept tree (replaces the old force-directed graph) ────
//
// One tree per workspace, rooted at the `cast::concept!` declared in
// `Cast.cast`. Concepts nest by longest-prefix-match on embodied
// anchors; primitive anchors are leaves with optional `→ embodied_by`
// cross-references; non-concept macros (rules / anti-patterns / etc.)
// are surfaced as per-concept count badges and fetched lazily via
// `tree_expand` on click.
async function showCanonicalTree() {
    const pane = byId('detail');
    if (!pane)
        return;
    pane.replaceChildren();
    pane.appendChild(emptyEl('loading tree…'));
    let body;
    try {
        const res = await fetch('/watcher/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ kind: 'tree' }),
        });
        const env = (await res.json());
        body = env.body;
    }
    catch {
        pane.replaceChildren(emptyEl('failed to load tree'));
        return;
    }
    if (!body || body.kind !== 'tree' || !body.tree) {
        pane.replaceChildren(emptyEl('unexpected tree response'));
        return;
    }
    pane.replaceChildren();
    const header = document.createElement('div');
    header.className = 'graph-header';
    const conceptCount = countConcepts(body.tree.root);
    const edgeCount = (body.tree.edges ?? []).length;
    header.textContent = `canonical tree — ${conceptCount} concepts, ${edgeCount} edges. click a name to dive in; click a count badge to expand macros.`;
    pane.appendChild(header);
    const treeRoot = document.createElement('div');
    treeRoot.className = 'tree';
    // Root node starts expanded so the user sees the top of the tree
    // immediately. All deeper nodes start collapsed — the expander glyph
    // reveals one level at a time on click.
    treeRoot.appendChild(renderTreeNode(groupStdlibChildren(body.tree.root), /* startCollapsed */ false));
    pane.appendChild(treeRoot);
}
function countConcepts(node) {
    let n = 1;
    for (const c of node.children ?? [])
        n += countConcepts(c);
    return n;
}
// In a non-cast workspace, every concept whose anchors live entirely
// under `cast_stdlib::*` / `cast_os_stdlib::*` ends up as a peer of
// the workspace's own concepts at the tree root — the placer's strict-
// prefix-match rule has nothing local to nest them under, so it falls
// back to "top". That swamps the workspace's own concepts under
// thousands of stdlib leaves.
//
// Fold them under a single collapsed parent: keep the existing
// `cast_stdlib` / `cast_os_stdlib` umbrella node if it's already a
// top-level child (cast-stdlib's lib.rs declares one), otherwise
// synthesize one. Either way the workspace's own concepts stay at the
// top level next to a small fold-out per stdlib.
function groupStdlibChildren(root) {
    const stdlibKids = [];
    const osStdlibKids = [];
    const otherKids = [];
    let stdlibUmbrella = null;
    let osStdlibUmbrella = null;
    for (const c of root.children ?? []) {
        if (c.name === 'cast_stdlib') {
            stdlibUmbrella = c;
            continue;
        }
        if (c.name === 'cast_os_stdlib') {
            osStdlibUmbrella = c;
            continue;
        }
        switch (classifyStdlibOrigin(c)) {
            case 'stdlib':
                stdlibKids.push(c);
                break;
            case 'os_stdlib':
                osStdlibKids.push(c);
                break;
            default:
                otherKids.push(c);
        }
    }
    // Nothing to fold — return the original tree unchanged.
    if (stdlibKids.length === 0 &&
        osStdlibKids.length === 0 &&
        !stdlibUmbrella &&
        !osStdlibUmbrella) {
        return root;
    }
    const newChildren = [...otherKids];
    if (stdlibKids.length > 0 || stdlibUmbrella) {
        newChildren.push(makeStdlibParent('cast_stdlib', stdlibUmbrella, stdlibKids));
    }
    if (osStdlibKids.length > 0 || osStdlibUmbrella) {
        newChildren.push(makeStdlibParent('cast_os_stdlib', osStdlibUmbrella, osStdlibKids));
    }
    return { ...root, children: newChildren };
}
function classifyStdlibOrigin(node) {
    const anchors = node.anchors ?? [];
    if (anchors.length === 0)
        return 'workspace';
    let stdlib = 0;
    let osStdlib = 0;
    for (const a of anchors) {
        if (a.path.startsWith('cast_stdlib::') || a.path === 'cast_stdlib')
            stdlib++;
        else if (a.path.startsWith('cast_os_stdlib::') || a.path === 'cast_os_stdlib')
            osStdlib++;
    }
    if (stdlib === anchors.length)
        return 'stdlib';
    if (osStdlib === anchors.length)
        return 'os_stdlib';
    return 'workspace';
}
function makeStdlibParent(syntheticName, existingUmbrella, orphans) {
    if (existingUmbrella) {
        return {
            ...existingUmbrella,
            children: [...(existingUmbrella.children ?? []), ...orphans],
        };
    }
    return {
        name: syntheticName,
        tags: ['stdlib'],
        children: orphans,
    };
}
const COUNT_LABELS = {
    rules: 'rules',
    anti_patterns: 'anti',
    comparisons: 'compare',
    pipelines: 'pipe',
    tiers: 'tier',
    matrices: 'matrix',
    gut_checks: 'gut',
    io_targets: 'io',
};
function renderTreeNode(node, startCollapsed = true) {
    const wrap = document.createElement('div');
    wrap.className = 'tree-node';
    wrap.dataset.concept = node.name;
    const head = document.createElement('div');
    head.className = 'tree-head';
    const expander = document.createElement('span');
    expander.className = 'tree-expander';
    const hasChildren = (node.children ?? []).length > 0 || (node.anchors ?? []).length > 0;
    expander.textContent = hasChildren ? '▾' : '·';
    head.appendChild(expander);
    const nameEl = document.createElement('span');
    nameEl.className = 'tree-name';
    if (node.name === STATE.selected)
        nameEl.classList.add('selected');
    nameEl.textContent = node.name;
    nameEl.addEventListener('click', () => selectConcept(node.name));
    head.appendChild(nameEl);
    // Tags (small chips after the name).
    for (const t of node.tags ?? []) {
        const chip = document.createElement('span');
        chip.className = 'tag-chip';
        chip.textContent = t;
        head.appendChild(chip);
    }
    // Graph warnings (orphan / undeclared / target_not_in_anchors). One
    // chip per warning kind with an attribute count and the kind label;
    // hover tooltip carries the messages.
    const warningChips = {};
    for (const w of node.warnings ?? []) {
        let bucket = warningChips[w.kind];
        if (!bucket) {
            bucket = [];
            warningChips[w.kind] = bucket;
        }
        bucket.push(w.message);
    }
    for (const [kind, msgs] of Object.entries(warningChips)) {
        const chip = document.createElement('span');
        chip.className = `warning-chip warning-${kind}`;
        const label = kind === 'orphan'
            ? '⚠ orphan'
            : kind === 'undeclared'
                ? '⚠ undeclared'
                : kind === 'target_not_in_anchors'
                    ? '⚠ target-mismatch'
                    : `⚠ ${kind}`;
        chip.textContent = msgs.length > 1 ? `${label}×${msgs.length}` : label;
        chip.title = msgs.join('\n');
        head.appendChild(chip);
    }
    // Count badges — clickable to lazy-expand macros under this node.
    const counts = node.counts ?? {};
    const totalCounts = Object.values(counts).reduce((a, b) => a + (b ?? 0), 0);
    let macrosPanel = null;
    if (totalCounts > 0) {
        for (const key of Object.keys(COUNT_LABELS)) {
            const v = counts[key] ?? 0;
            if (!v)
                continue;
            const badge = document.createElement('span');
            badge.className = 'tree-count';
            badge.textContent = `${COUNT_LABELS[key]}:${v}`;
            badge.addEventListener('click', (ev) => {
                ev.stopPropagation();
                if (!macrosPanel) {
                    macrosPanel = document.createElement('div');
                    macrosPanel.className = 'tree-expanded-macros';
                    macrosPanel.appendChild(emptyEl('loading macros…'));
                    wrap.appendChild(macrosPanel);
                    void loadExpand(node.name, macrosPanel);
                }
                else {
                    macrosPanel.classList.toggle('collapsed');
                    if (macrosPanel.classList.contains('collapsed')) {
                        macrosPanel.style.display = 'none';
                    }
                    else {
                        macrosPanel.style.display = '';
                    }
                }
            });
            head.appendChild(badge);
        }
    }
    wrap.appendChild(head);
    // Embodied anchors covered by a child concept (`leaf: false`) are
    // hidden — the child concept node IS the deeper view of that code.
    // Primitives always render as leaves.
    const visibleAnchors = (node.anchors ?? []).filter((a) => {
        if (a.role === 'embodied' && a.leaf === false)
            return false;
        return true;
    });
    if (visibleAnchors.length > 0) {
        const anchorsList = document.createElement('ul');
        anchorsList.className = 'tree-anchors';
        for (const a of visibleAnchors) {
            const li = document.createElement('li');
            li.className = `role-${a.role}`;
            const code = document.createElement('code');
            code.textContent = a.path;
            li.appendChild(code);
            if (a.role === 'primitive' && a.embodied_by) {
                const xref = document.createElement('span');
                xref.className = 'tree-xref';
                xref.textContent = `→ ${a.embodied_by}`;
                const embodiedBy = a.embodied_by;
                xref.addEventListener('click', (ev) => {
                    ev.stopPropagation();
                    selectConcept(embodiedBy);
                });
                li.appendChild(xref);
            }
            anchorsList.appendChild(li);
        }
        wrap.appendChild(anchorsList);
    }
    // Recursively render children. Each child starts collapsed — the
    // user reveals one level at a time by clicking expanders.
    if ((node.children ?? []).length > 0) {
        const childList = document.createElement('div');
        childList.className = 'tree-children';
        for (const c of node.children ?? []) {
            childList.appendChild(renderTreeNode(c, /* startCollapsed */ true));
        }
        wrap.appendChild(childList);
    }
    // Apply initial collapsed state. Whichever inner panels exist
    // (children, anchors) get hidden; the expander glyph flips to ▸.
    if (hasChildren) {
        expander.classList.add('has-children');
        if (startCollapsed) {
            const sections = wrap.querySelectorAll(':scope > .tree-children, :scope > .tree-anchors');
            sections.forEach((s) => {
                s.classList.add('collapsed');
                s.style.display = 'none';
            });
            expander.textContent = '▸';
        }
        expander.addEventListener('click', () => {
            // Per-level expand: toggle THIS node's direct child panels only.
            // Grandchildren stay in their own collapsed state — clicking ▸ on
            // a level-1 node reveals its direct children (each rendered with
            // their own children collapsed), and the user clicks again to go
            // deeper.
            const sections = wrap.querySelectorAll(':scope > .tree-children, :scope > .tree-anchors, :scope > .tree-expanded-macros');
            let anyCollapsed = false;
            sections.forEach((s) => {
                if (s.classList.contains('collapsed'))
                    anyCollapsed = true;
            });
            sections.forEach((s) => {
                if (anyCollapsed) {
                    s.classList.remove('collapsed');
                    s.style.display = '';
                }
                else {
                    s.classList.add('collapsed');
                    s.style.display = 'none';
                }
            });
            expander.textContent = anyCollapsed ? '▾' : '▸';
        });
    }
    return wrap;
}
async function loadExpand(conceptName, panel) {
    let body;
    try {
        const res = await fetch('/watcher/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ kind: 'tree_expand', concept: conceptName }),
        });
        const env = (await res.json());
        body = env.body;
    }
    catch {
        panel.replaceChildren(emptyEl('failed to expand'));
        return;
    }
    if (!body || body.kind !== 'tree_expand_result') {
        panel.replaceChildren(emptyEl('unexpected expand response'));
        return;
    }
    panel.replaceChildren();
    const children = body.children ?? [];
    if (children.length === 0) {
        panel.appendChild(emptyEl('no macros attributed'));
        return;
    }
    for (const c of children) {
        const row = document.createElement('div');
        row.className = 'macro-row';
        const kind = document.createElement('span');
        kind.className = 'macro-kind';
        kind.textContent = c.kind;
        row.appendChild(kind);
        const loc = document.createElement('span');
        loc.className = 'macro-loc';
        loc.textContent = `${shortFile(c.file)}:${c.line}`;
        row.appendChild(loc);
        panel.appendChild(row);
    }
}
// ── Mailbox (user ↔ assistant) ────────────────────────────────────────
function initMailbox() {
    const messages = byId('messages');
    const meta = byId('meta');
    const form = byId('send');
    const body = byId('body');
    const button = byId('send-button');
    const clearButton = byId('clear-button');
    if (!messages || !meta || !form || !body || !button)
        return;
    void refreshMessages(messages);
    void refreshMeta(meta, button);
    window.setInterval(() => void refreshMeta(meta, button), 15_000);
    const sse = new EventSource('/mailbox/stream');
    sse.addEventListener('file', () => {
        void refreshMessages(messages);
        void refreshMeta(meta, button);
    });
    body.addEventListener('keydown', (ev) => {
        if (ev.key === 'Enter' && (ev.ctrlKey || ev.metaKey)) {
            ev.preventDefault();
            form.requestSubmit();
        }
    });
    form.addEventListener('submit', async (ev) => {
        ev.preventDefault();
        const text = body.value.trim();
        if (!text)
            return;
        button.disabled = true;
        try {
            const res = await fetch('/mailbox/send', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ role: 'user', body: text }),
            });
            if (!res.ok) {
                meta.textContent = `send failed: ${res.status} ${await res.text()}`;
                return;
            }
            body.value = '';
            void refreshMessages(messages);
        }
        finally {
            void refreshMeta(meta, button);
        }
    });
    if (clearButton) {
        clearButton.addEventListener('click', async () => {
            if (!confirm('Remove every message in the mailbox? meta.json is preserved.'))
                return;
            clearButton.disabled = true;
            try {
                const res = await fetch('/mailbox/clear', { method: 'POST' });
                if (!res.ok) {
                    meta.textContent = `clear failed: ${res.status} ${await res.text()}`;
                    return;
                }
                // The directory watcher only emits create / rename events, not
                // remove — so the SSE won't fire. Refresh locally.
                void refreshMessages(messages);
            }
            finally {
                clearButton.disabled = false;
            }
        });
    }
}
async function refreshMessages(list) {
    const res = await fetch('/mailbox/messages');
    if (!res.ok) {
        setEmpty(list, `failed: ${res.status}`);
        return;
    }
    const data = (await res.json());
    list.replaceChildren();
    if (!data.messages || data.messages.length === 0) {
        setEmpty(list, 'no messages');
        return;
    }
    for (const m of data.messages) {
        const li = document.createElement('li');
        li.className = m.role === 'user' ? 'msg-user' : 'msg-assistant';
        const head = document.createElement('div');
        head.className = m.role === 'user' ? 'role-user' : 'role-assistant';
        head.textContent = `${m.id} · ${m.role}${m.in_reply_to ? ` (reply to ${m.in_reply_to})` : ''}`;
        li.appendChild(head);
        const bodyDiv = document.createElement('div');
        bodyDiv.className = 'body';
        bodyDiv.textContent = m.body;
        li.appendChild(bodyDiv);
        if (m.payloads && m.payloads.length > 0) {
            const tags = document.createElement('div');
            tags.className = 'tag';
            tags.textContent = `payloads: ${m.payloads.map((p) => p.tag).join(', ')}`;
            li.appendChild(tags);
        }
        list.appendChild(li);
    }
    list.scrollTop = list.scrollHeight;
}
async function refreshMeta(meta, button) {
    const res = await fetch('/mailbox/meta');
    if (!res.ok) {
        meta.className = 'meta-line detached';
        meta.textContent = `meta failed: ${res.status}`;
        button.disabled = true;
        return;
    }
    const data = (await res.json());
    if (data.attached) {
        meta.className = 'meta-line';
        const age = data.age_seconds ?? 0;
        meta.textContent = `claude session attached (heartbeat ${age}s ago)`;
        button.disabled = false;
    }
    else {
        meta.className = 'meta-line detached';
        meta.textContent = data.last_heartbeat_at
            ? `no claude session attached (last heartbeat ${data.last_heartbeat_at})`
            : 'no claude session attached';
        button.disabled = false;
    }
}
// ── Watcher log (assistant ↔ cast-watch) ──────────────────────────────
function initWatcherLog() {
    const list = byId('watcher-log');
    if (!list)
        return;
    void refreshWatcherLog(list);
    const sse = new EventSource('/watcher/log/stream');
    sse.addEventListener('file', () => void refreshWatcherLog(list));
}
async function refreshWatcherLog(list) {
    let res;
    try {
        res = await fetch('/watcher/log/entries');
    }
    catch (e) {
        setEmpty(list, `request failed: ${String(e)}`);
        return;
    }
    if (!res.ok) {
        setEmpty(list, `failed: ${res.status}`);
        return;
    }
    const data = (await res.json());
    const turns = data.turns ?? [];
    list.replaceChildren();
    if (turns.length === 0) {
        setEmpty(list, 'no queries yet');
        return;
    }
    for (const t of turns) {
        const li = document.createElement('li');
        li.className = 'turn';
        const head = document.createElement('div');
        head.className = 'head';
        head.textContent = `turn ${t.turn}`;
        li.appendChild(head);
        const q = document.createElement('div');
        q.className = 'q';
        q.textContent = querySummary(t.query);
        li.appendChild(q);
        const r = document.createElement('div');
        r.className = 'r';
        r.textContent = responseSummary(t.response);
        li.appendChild(r);
        list.appendChild(li);
    }
    list.scrollTop = list.scrollHeight;
}
function querySummary(q) {
    if (!q || typeof q !== 'object')
        return JSON.stringify(q);
    const kind = q['kind'] ?? '?';
    const extras = [];
    const from = q['from'];
    if (from !== undefined) {
        extras.push(`from=${Array.isArray(from) ? from.join(',') : String(from)}`);
    }
    if (q['dim'] !== undefined)
        extras.push(`dim=${String(q['dim'])}`);
    if (q['radius'] !== undefined && q['radius'] !== null) {
        extras.push(`radius=${String(q['radius'])}`);
    }
    if (q['format'] !== undefined)
        extras.push(`format=${String(q['format'])}`);
    if (q['hops'] !== undefined && q['hops'] !== null) {
        extras.push(`hops=${String(q['hops'])}`);
    }
    return `${kind} ${extras.join(' ')}`.trim();
}
function responseSummary(r) {
    if (!r || typeof r !== 'object')
        return JSON.stringify(r);
    const body = r.body ?? r;
    const kind = body['kind'] ?? '?';
    let summary = kind;
    if (body['count'] !== undefined && body['count'] !== null) {
        summary += ` count=${String(body['count'])}`;
    }
    if (body['format'] !== undefined)
        summary += ` format=${String(body['format'])}`;
    const visited = body['visited'];
    if (visited && typeof visited === 'object') {
        summary += ` visited=${Object.keys(visited).length}`;
    }
    if (body['phase'] !== undefined)
        summary += ` phase=${String(body['phase'])}`;
    return summary;
}
main();
//# sourceMappingURL=main.js.map