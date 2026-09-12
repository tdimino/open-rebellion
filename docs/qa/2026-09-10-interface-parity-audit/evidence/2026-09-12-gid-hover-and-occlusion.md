# GID hover and system-window overlap checkpoint

Status: visual cleanup and source-pixel regression, not `CMD-02` acceptance.

The GID row renderer added a translucent green-gray hover rectangle with no
identified original bitmap or constructor rule. P51 removes that replacement
paint while retaining category hover, submenu opening, and leaf selection.
The test harness captures the root before and during hover and checks a clear
row pixel for accidental reintroduction of the wash. This does not establish
the original hover or focus presentation; those states still need an
authoritative original-game capture.

P50 checked complete root borders only when no detailed system window was in
front. P51 confirms the known window overlap and compares every remaining
visible frame-border pixel with STRATEGY `10100..10107`. In the two native-size
system-window cases, `453` pixels per faction match the source BMPs and `181`
pixels per faction are recorded as covered by the window. A mismatch in the
visible remainder now fails the bundle instead of silently skipping the
entire frame.

The full 38-scenario, two-faction, two-viewport muted browser gate passed
`152/152` cases. Seventy-four native captures matched complete `634`-pixel
root borders, while the two native system-window captures matched `453`
visible pixels each. This totals `47,822` exact source-BMP pixels; `362` pixels
were genuinely covered. All 76 responsive captures exercised routing and
letterboxed geometry without claiming native pixel identity. Every case made
four successful startup requests, reported no browser or missing-asset errors,
and closed its isolated browser process. The passing raw run is
`.artifacts/interface-parity/2026-09-12T15-50-49-729Z-46479` (ignored).
The production WASM SHA-256 is
`fe509f56e763376d49d94887a7be946c419f573a1785cac22e37a719dac97a7c`;
the unchanged runtime pack SHA-256 is
`1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`.

The workspace tests, native build, formatting, and strict Clippy all pass via
`make all` with the clean-repository Clippy configuration. Astra medium's
read-only review found no blocking issue and approved this bounded commit.
It independently checked the 152 muted result records and confirmed that the
browser matrix uses a separate test WASM. The production WASM has its own
packaged-build hash above; no production browser parity claim is inferred from
the fixture run.
No unscaled, lossless original-executable reference has been accepted for the
GID hover state or full window stacking composition. All strict `CMD-02`
cells remain pending.
