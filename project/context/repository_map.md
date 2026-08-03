---
title: Repository Map
scope: project
status: active
---

# Repository Map

Derived navigation guide for agents and contributors. Not authoritative — see the
`project/` control plane for authoritative project state. Review and update when
the dependency surface or project structure changes significantly.

---

## Project Entry Points

| Subsystem | Primary file(s) | Notes |
|---|---|---|
| Game logic (Rust/WASM) | `replication_vector/src/lib.rs` | All public exports; WASM entry point via `wasm_bindgen` |
| Web harness | `replication_vector/web/index.html`, `replication_vector/web/package.json` | Vite-based; minimal HTML wrapper loading the WASM module |
| Cargo manifest | `replication_vector/Cargo.toml` | Declares `webgpu_vector_lib` path dep and `wasm-bindgen` |
| Validation scripts | `scripts/` | Canonical entry points for format, lint, test, baseline, validate |
| CI | `.github/workflows/validate.yml` | Runs `scripts/develop` then `scripts/validate` |

---

## Local Dependency: Velumin (`webgpu_vector_lib`)

**Location:** `.deps/velumin/webgpu_vector_lib/`  
**Crate path dep:** `webgpu_vector_lib = { path = "../.deps/velumin/webgpu_vector_lib" }`  
**Setup:** Managed by `scripts/develop`. Do not commit the `.deps/` directory.

### Where to start

Read `.deps/velumin/webgpu_vector_lib/src/lib.rs` — this single file contains
the entire public API surface and the rendering implementation.

### Public types (all platform-agnostic, safe to use in non-WASM code)

| Type | Purpose |
|---|---|
| `Vec2` | 2D point in normalized device coordinates. Range roughly −1.0 to 1.0 on each axis. |
| `Color` | RGBA floats 0.0–1.0. `red`, `green`, `blue`, `alpha`. |
| `StrokeStyle` | `width` (NDC units), `color`, `intensity` (multiplier; >1.0 contributes glow). |
| `Line` | `start: Vec2`, `end: Vec2`, `style: StrokeStyle`. |
| `Polyline` | `points: Vec<Vec2>`, `style: StrokeStyle`. Prefer over multiple `Line`s for connected shapes. |
| `VectorCommand` | Enum: `VectorCommand::Line(Line)` or `VectorCommand::Polyline(Polyline)`. |

A scene is a `Vec<VectorCommand>`. Building and returning this vec is
platform-agnostic Rust and does not require WASM compilation.

### WASM browser API (target_arch = "wasm32" only)

`WebGPU::create(canvas_id: &str) -> Result<WebGPU, JsValue>` (async)  
`WebGPU::render(&mut self) -> Result<(), JsValue>`  
`WebGPU::render_blasterites_tester(&mut self, time_ms: f64) -> Result<(), JsValue>`

**Current state:** `WebGPU::render()` calls Velumin's internal `smoke_scene()` —
it does not yet accept external `VectorCommand` slices. Replication Vector does
not yet wire its own scenes into the browser renderer. This is the next integration
step (rendering spike).

### Coordinate system and style conventions

- Origin (0, 0) is screen center. X increases right; Y increases up.
- Typical gameplay elements fit within ±0.8 on each axis.
- `width` values: thin projectile lines ~0.01–0.015; entity outlines ~0.014–0.02;
  thick structural lines ~0.04.
- `intensity` 1.0 = normal brightness. Values above 1.0 feed the glow pass; the
  renderer handles dual-pass compositing automatically.
- Colors are RGBA floats. Common palette: white `(0.92, 0.96, 1.0)`, cyan
  `(0.2, 0.95, 1.0)`, amber `(1.0, 0.68, 0.18)`, blue `(0.55, 0.8, 1.0)`.

### Existing usage in this project

`replication_vector/src/lib.rs` — `bootstrap_scene()` shows the minimal pattern:
construct a `Color` constant, define a helper `line(start, end) -> VectorCommand`,
return a `Vec<VectorCommand>`. The three-line triangle is a working, tested example.

### Known unknowns

- The external API for passing scene commands into `WebGPU::render` is not yet
  designed. Do not assume or invent it; watch for a future work item or design
  proposal.
- `Polyline` is implemented and tested in Velumin but not yet used in Replication
  Vector. Prefer it for entity outlines (ship hull, asteroid outline, shield arc).
- WebGL2 fallback is proposed but not adopted. Assume WebGPU is required.

### Velumin's own project artifacts

Velumin is also an LRH project. Its control plane is at `.deps/velumin/project/`.
Read `.deps/velumin/project/context/agents.md` for a compact orientation.
Do not modify Velumin's files; treat it as a read-only upstream dependency.

---

## Key Conventions

- **Coordinate units:** NDC (−1.0 to 1.0). All position values are in this space.
- **Scene construction is pure Rust:** no WASM, no browser APIs needed to build or
  test a `Vec<VectorCommand>`. Tests can run with `cargo test` on any platform.
- **Validation entry points are in `scripts/`:** do not invoke `cargo`, `wasm-pack`,
  `npm`, or `rustfmt` directly. Use the canonical scripts.
- **Velumin is a path dep, not a registry crate:** changes to `.deps/velumin/` take
  effect on the next `cargo build` with no version bump needed, but `.deps/` must
  not be committed.
