# Test Metrics

Brainstorming what test metrics we might want if test metrics generation is
requested. Rough ideas prioritized with basic MoSCoW method. If we are going to
setup test metrics infra, the following priority listing might come in handy as
part of it.

TODO

## Must have

- Ignored test metrics.
    - Count: individual test suites + overall + by ignore reason.
    - Reason: collect why tests are ignored.
        - Basic cause: e.g. `//@ ignore-arch-x86_64`
        - (Optional) reason: e.g.
            - Remark: might be hard to do, reason can be a comment not included
              on the same directive line. Maybe a `//@ ignore-reason:`
              above/below the ignore directive?

## Should have

- Test running time metrics:
    - Per-test granularity: how long did each test take?
    - Per-test-suite granularity: how long did each test suite take?
    - Overall: how long did the entire invocation take?
    - Sort either lexicographically or by descending running time (i.e. longest
      tests first).

## Could have

- Test coverage:
    - Remark: this is harder to do and it's not my priority to figure out how to
      wire that up yet, I want to first make compiletest maintainable +
      extensible so this can be done in a sane manner in the first place.

## Won't have

TODO
