# README: Library Architecture Documentation

## Purpose

This directory (`doc/app/`) contains **high-level architecture documentation** for the `cdx_file_rs` library. These documents explain the design philosophy, structure, and extension points to help new developers and AI agents quickly understand the codebase.

## Target Audience

- **New Contributors**: Developers wanting to contribute to the library
- **Library Users**: Teams building applications on top of `cdx_file_rs`
- **AI Agents**: Automated systems that need to understand the codebase structure
- **Maintainers**: Future maintainers who need to grasp design decisions

## Document Overview

### [00-overview.md](00-overview.md)
**What you'll learn**:
- Library purpose and use cases
- High-level capabilities (implemented, partial, planned)
- Directory structure guide
- Quick start for understanding the codebase
- Typical usage patterns

**Read this first** if you're new to the project.

### [01-architecture.md](01-architecture.md)
**What you'll learn**:
- Layered architecture diagram
- Responsibilities of each layer (Application → Rendering → Domain → Parsing → Binary → Tags)
- Data flow examples (reading files, user interactions, writing files)
- Dependency rules and rationale
- Key architectural decisions

**Read this** to understand how the pieces fit together.

### [02-data-model.md](02-data-model.md)
**What you'll learn**:
- Two-layer abstraction pattern (raw binary vs. typed structs)
- TaggedObject trait and conversion patterns
- Tree structure using dendron
- NodePayload enum design
- Roundtrip fidelity implementation

**Read this** to understand the data representation strategy.

### [03-extensibility.md](03-extensibility.md)
**What you'll learn**:
- Extension point 1: Rendering backends (AbstractPainter trait)
- Extension point 2: Interactive modes (ModeHandler trait)
- Extension point 3: New CDX object types (step-by-step guide)
- Extension point 4: Custom property codecs
- Extension point 5: Export formats

**Read this** when adding new features or backends.

### [04-design-principles.md](04-design-principles.md)
**What you'll learn**:
- Core principle 1: Separation of concerns
- Core principle 2: Type safety over flexibility
- Core principle 3: Roundtrip fidelity
- Core principle 4: Extensibility through abstraction
- Core principle 5: Progressive enhancement
- Core principle 6: Documentation as code
- Design trade-offs and rationale
- Anti-patterns to avoid

**Read this** to understand the "why" behind design decisions.

### [05-file-format.md](05-file-format.md)
**What you'll learn**:
- CDX binary format structure
- Tag system (object tags vs. property tags)
- Data type encodings (primitives, coordinates, strings, colors)
- Object hierarchy rules
- Coordinate systems and units
- Special object types (ColorTable, Bonds, etc.)
- Parsing strategy

**Read this** when working with file format details or adding new object types.

## Reading Paths

### Path 1: "I want to understand the library quickly"
1. [00-overview.md](00-overview.md) - Purpose and structure
2. [01-architecture.md](01-architecture.md) - How it's organized
3. [04-design-principles.md](04-design-principles.md) - Why it's designed this way

### Path 2: "I want to add a new feature"
1. [00-overview.md](00-overview.md) - Get oriented
2. [03-extensibility.md](03-extensibility.md) - Find the right extension point
3. [02-data-model.md](02-data-model.md) - Understand data structures (if adding objects)
4. [05-file-format.md](05-file-format.md) - Understand format (if adding CDX object types)

### Path 3: "I want to add a new rendering backend (SVG/PDF)"
1. [01-architecture.md](01-architecture.md) - Understand rendering layer
2. [03-extensibility.md](03-extensibility.md) - AbstractPainter trait guide
3. [04-design-principles.md](04-design-principles.md) - Understand separation of concerns

### Path 4: "I want to add a new editing mode"
1. [01-architecture.md](01-architecture.md) - Understand mode handler layer
2. [03-extensibility.md](03-extensibility.md) - ModeHandler trait guide
3. Look at `src/mode_handlers/select_mode.rs` for reference

### Path 5: "I need to parse/understand a CDX file"
1. [05-file-format.md](05-file-format.md) - Format specification
2. [02-data-model.md](02-data-model.md) - How it's represented in code
3. Look at `src/cdx_parse_impl/` for implementation

## Relationship to Other Documentation

### This Directory (`doc/app/`)
- **Focus**: Architecture, design principles, high-level concepts
- **Audience**: Developers and agents understanding the system
- **Style**: Explanatory prose, diagrams, examples

### `doc/md/` Directory
- **Focus**: Detailed tag specifications (converted from HTML spec)
- **Audience**: Developers implementing specific CDX objects
- **Style**: Reference documentation, tables

### Markdown Files in Root
- `IMPLEMENTATION_GUIDE.md`: Step-by-step guide for adding objects
- `TAG_VERIFICATION_REPORT.md`: Which tags are implemented
- `RENDERER_REVIEW_ISSUES.md`: Known renderer issues
- `Z_ORDER_IMPLEMENTATION_GUIDE.md`: Planned z-order feature
- etc.

### Code Documentation
- **Inline doc comments**: Explain specific APIs and methods
- **Module-level comments**: Explain module purpose
- **Examples in docstrings**: Show usage patterns

## Maintenance

### Keeping Documentation Updated

**When to update these docs**:
- Major architectural changes (e.g., backend abstraction)
- New extension points added
- Design principles evolve
- File format changes (rare)

**Who updates**:
- Core maintainers during design reviews
- Contributors adding major features
- AI agents when prompted

**Review process**:
- Documentation changes reviewed alongside code changes
- Ensure examples remain valid
- Update "Current Status" sections

### Documentation Standards

**Writing style**:
- Clear, concise prose
- Use examples and diagrams
- Explain "why" not just "what"
- Assume reader is competent developer but unfamiliar with codebase

**Code examples**:
- Must compile (or be clearly marked as pseudocode)
- Include imports where needed
- Show complete context

**Diagrams**:
- ASCII art for simple diagrams
- Clear layering and flow
- Annotated with explanations

## Contributing

If you find errors, ambiguities, or gaps in this documentation:

1. **File an issue**: Describe the problem
2. **Submit a PR**: Fix the documentation directly
3. **Ask in discussions**: If you need clarification

Good documentation is a collaborative effort!

## License

These documentation files are part of the `cdx_file_rs` project and are distributed under the same license as the project (see LICENSE file in repository root).

---

**Document Status**: Created February 2026
**Last Updated**: February 2, 2026
**Maintained By**: `cdx_file_rs` core team
