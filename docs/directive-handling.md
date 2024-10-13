# Directive handling sketches

## Session-global context and diagnostic context

... but without the translation infra. And don't use raw `print!`/`println!` to
not break when the pipes are broken. 

```rs
struct Ctx { // Session-global context
    cap: Capabilities,
    dcx: DiagCtx,
    filter: Filter, // ./x test [tests/ui/foo] <- tests/ui/foo is filter, what compiletest was
                    // invoked with
    suite: Suite,
    mode: Mode,     // test mode is a one-to-many (test mode -> [test suite])
                    // e.g. ([ui] => [ui, ui-fulldeps])
}

ctx.emit_err();
let _ = emit_err!(ctx, "revision {user} collides with builtin cfg {builtin}");
ctx.emit_warn();
ctx.emit_info();
ctx.debug(); // no need, compiletest can use tracing unlike bootstrap, easy clap :)
// If the printing gets routed in a centralized fashion, we can
// yoink how cargo does it to even allow compiletest output snapshot self-tests!
```

Instead of having scattered logic everywhere, collect things into a package if
their lifetime is session global.

Current compiletest error handling and reporting is, ngl, kinda bad and not very
helpful. We can potentially do something fancy like (haha) annotate-snippets and
not only show the offending directive, we can even show help and suggestion
messages. I've wanted to add diagnostics like "unrecognized directive `meow`,
did you mean `bark` instead?" I think this is important for UX because ngl it's
extremely not obvious currently if e.g. some compile-flags is incompatible with
some directives, or whatever.

## Capabilities

```rs
// if ctx.cap.can_symlink() { yeah!(); }

impl Capabilities {
    fn detect() -> Capabilities {
        Self {
            process: false /* check by spawning a process */,
            symlink: false /* by trying to create one */,
        }
    }
}
```

Three kinds of capabilities:

1. Provided: this is passed from bootstrap via cli args.
2. Queried: we ask `rustc --print=all-target-specs` or `rustc
   --print=builtin-cfg` or whatever that one is called.
3. External detection: `Command::new("make").spawn()` :). We can run these
   detections up-front, then cache the results into `Capabilities`. This is nice
   because we can test the detection separately from how things react to given
   capabilities. E.g. we can configure a `Capabilities` and test how some
   follow-up code paths react to it. *And* we can do integration testing still.

## Diagnostics

Associate directive line with positional info.

```rs
// 0. Diagnostics
struct Span { lo: u32, hi: u32 }
// Possibly use an arena for test `PathBuf`s then handout an id into the arena
// to make it easier to pass around by copy? No deletion so we don't need to
// deal with the ABA problem.
struct Loc { test_path: PathId, span: Span }
```

We can even show a test source snippet via `annotate-snippets` + `color_eyre`
and what not.

### Path normalizations

Probably see
<https://github.com/rust-lang/cargo/blob/82c489f1c612096c2dffc48a4091d21af4b8e046/crates/cargo-util/src/paths.rs#L84>
or use something like `normpath`. Notable these will not traverse symlinks and
probably should avoid accessing fs, which is probably what we want anyway.
`std::fs::canonicalize` even tries to create the file and accesses the fs which
makes it really annoying to use. On Windows that returns the UNC prefixed path
which is a PITA to work with.

## Directive parsing/validation phases and lowering to test config set

Why separate phases? So different phases can focus on catching different things
and make it easier to produce better diagnostics. Consider this a gradual
"tightening" or "lowering" of directive mini-IRs. Some lints / validation logic
are easier/cleaner to implement on different stages of such lowerings.

Also, phases are a logical separation to say "hey, running this phase resulted
in 3 errors" and we have the flexibility to choose whether to proceed to the
next phase or exit with error.

### Basic recognition into `(name)`, `(name, value)` forms

```rs
// 1. Basic recognition (fast reject, enforce basic "shape")
struct ParseDirectiveOutcome {
    loc: Loc,
    kind: ParseDirectiveOutcomeKind,
}
enum ParseDirectiveOutcomeKind<'a> {
    NameOnly { name: &'a },
    NameValue { name: &'a, value: String },
    MissingColon { name: &'a },
    MissingValue { name: &'a },
    DidNotExpectValue { value: String },
    UnrecognizedName { line: &'a str },
}
fn parse_directive(cx: &Ctx, loc: Loc, line: &str) -> ParseDirectiveOutcome<'a> {
    todo!()
}
```

Focused on restricting into a basic `(name, value?, comment?)` "shape". This
phase should reject and report on unknown directives, missing colon/value and
the likes.

> Question: how do we handle directive comments? Probably some directives can
> allow trailing comments but not others, like `revisions: xxx` cannot handle
> comments.

> Question: can LLVM FileCheck directives linting fit into this?

### Early validation into structured directive representation

Try to lower the basic `(name, value?, comment?)`

```rs
// 2. Early validation (raw directive -> structured directive repr)
//    on individual directive basis
enum ValidationError {
    YouWroteWeirdCharactersAndYouShouldFeelBad { first_unexpected_character: String },
    CollidesWithBuiltinCfg { name: &'static str },
}
fn early_validate_directive(
    loc: Loc,
    name: &str,
    raw_value: &str
) -> Result<Directive, Vec<ValidationError>> {
    fn validate_revisions() -> ControlFlow<ValidationError, ()> { todo!() }
}

enum DirectiveKind {
    /// I can has doc comments that get rendered in nightly-rustc tool docs!
    /// Or maybe even somehow generate docs dynamically(?!) based on directive declaration?
    CompileFlags(...),
    Revisions(...),
}
impl DirectiveKind {
    fn name() -> &'static str { todo!() }
    fn supports_test_suite(suite: Suite) -> bool { todo!() }
    fn can_be_revisioned() -> bool { todo!() }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Revision { Universal, Named(String) }
struct Directive {
    rev: Revision,
    kind: DirectiveKind,
}

// 3. Late validation and assemble into test config set on a per-test granularity.
struct TestConfigSet(BTreeMap<Revision, TestConfig>);
struct TestConfig { exec_env: ExecEnv, rustflags: Rustflags, /* ? */ }
// Note that we can have multiple instances of a directive kind, e.g. `//@ revisions: a` + `//@
// revisions: b`, or `//@ compile-flags`. Then this should also handle merging / linting.
// Probably want to report as many diagnostics as possible.
struct LateValidationError {
    loc: Loc,
    kind: LateValidationErrorKind
}
enum LateValidationErrorKind {
    IncompatibleFlags { /* specifics */ },
}
fn late_validation_assmble_set(directives: Vec<Directive>)
    -> Result<TestConfigSet, Vec<LateValidationError>>
{
    todo!()
}

// 4. Now we have the assembled test config, we need to hand that off to libtest.
// ??? how does this part work with runtest?
```
