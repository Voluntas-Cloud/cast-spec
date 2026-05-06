# Architectural Prose Style

A small vocabulary of structural blocks for writing dense architecture notes
without dropping into UML, ADR ceremony, or unstructured prose.

## When this style fits

When a paragraph would need to express multiple axes, side-by-side
comparisons, or "before vs after" transformations, and a diagram would be too
coarse to sit inline. These blocks live *between* prose and diagrams.

When this style does NOT fit:
- Implementation detail that belongs next to a function — that's what
  inline comments are for.
- Sequential tutorials — numbered steps in prose work better.
- Anything where the reader just needs one specific fact — prose gets there
  faster than a block.

---

## Block types

### Compare

Two named concepts side by side. Use when the concepts are easy to confuse
or are routinely smushed together.

```
intent_id   = stable identity, used for traceability across logs
intent_hash = replay-guard key, bound to exact server-signed bytes
```

### Pipeline

A sequence of transforms with arrows. Use when the transformation *chain*
is the point, not any single stage.

```
raw voluntlet facts
  → normalized node capabilities
    → cluster capabilities
      → stability assessment
        → upgrade opportunities
```

### Tier

Ordered discrete levels. Use when "how strict / how risky / how mature"
matters, and three or four named buckets are clearer than a continuum.

```
fragile           ← single point of failure exists
recoverable       ← failure causes downtime, data survives
resilient         ← failure tolerated with degraded service
highly_available  ← failure tolerated transparently
```

### Matrix

Several things sharing structure but differing in values. Use when multiple
domains, types, or cases all answer the same set of questions.

```
                 | derived?  | stored?      | scope          | example
HOTP-like TAN    | yes       | seed only    | any login      | RFC 4226
Static TAN list  | no        | finite bag   | any login      | recovery codes
Transaction TAN  | yes       | server-side  | bound to txn   | photoTAN
```

### Rule + reason

A short imperative rule followed by a one-line "why." Use when a principle
is load-bearing and easy to violate without thinking.

> **Unify at the task level, separate at the secret/lifecycle level.**
> Mixing storage models for derived vs stored secrets produces either
> silent replay (consume function not actually consuming) or breakage
> (derive function trying to mark used).

### Anti-pattern

Explicitly named "don't do it this way, because…" Use when the *wrong*
path is the tempting path.

```
Do NOT implement this as:
  if (x && y && z) then do_thing
```
> Spaghetti decisions are unmaintainable and unauditable. Use the pipeline:
> `inputs → evaluation → ranked options → chosen action`.

### Gut-check (use sparingly)

A single decision question that resolves ambiguity at the *writing* layer,
so the reader never has to re-derive it.

> If you can regenerate the code later → it's a key (TOTP/HOTP).
> If the code disappears after use forever → it's a stored artifact (recovery/TAN).

---

## Rules of use

- **Bridge, don't repeat.** Prose around a block bridges *into* it ("here is
  the distinction") and *out of* it ("which is why we keep two vaults").
  Never restate what the block already shows.
- **Density over decoration.** A block that is mostly whitespace has not
  earned its layout. If a one-liner works, write the one-liner.
- **Code blocks for shapes that aren't code.** Use fenced blocks for
  structural pseudo-text, not just literal code. The shape *is* the meaning;
  monospace preserves the alignment that carries it.
- **Name the block's role only when it helps.** Inline blocks rarely need a
  heading; a heading promotes a block to a section, which is heavier.
- **One block per idea.** If you find yourself reaching for two block types
  in a row, the idea probably needs splitting.
- **Block first, prose second.** When the structure is clear, write the
  block, then write the bridge prose around it. Doing it the other way
  tends to over-explain.

---

## What this style is *for*

Capturing abstract design ideas — pipelines, distinctions, classifications,
load-bearing rules — at a granularity that lives next to the type and
function names that implement them, without forcing the reader to expand
prose into structure in their head.

Blocks compress; prose connects. Use both.
