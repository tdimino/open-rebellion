---
title: "GitHub Pages Documentation Proof"
description: "Deployment and public smoke-test evidence for P40"
category: qa
created: 2026-09-08
updated: 2026-09-10
tags: [qa, documentation, github-pages, deployment]
---

# GitHub Pages Documentation Proof

P40 passes for the current documentation. GitHub Pages deployment run
[`34505121163`](https://github.com/tdimino/open-rebellion/actions/runs/34505121163)
built, uploaded, and deployed documentation from `main` at commit
`bcc224fc1bf947e0679982bed68dcc41f827d464`.

Public HTTP smoke tests returned 200 for:

- [Project landing page](https://tdimino.github.io/open-rebellion/)
- [Universal documentation index](https://tdimino.github.io/open-rebellion/docs/INDEX.html)
- [Campaign-history index](https://tdimino.github.io/open-rebellion/docs/reference/campaign-history/)
- [Official campaign contract](https://tdimino.github.io/open-rebellion/docs/reference/campaign-history/official-campaign-contract.html)
- [Machine-readable source ledger](https://tdimino.github.io/open-rebellion/docs/reference/campaign-history/sources.json)
- [Preserved original manual](https://tdimino.github.io/open-rebellion/docs/reference/campaign-history/archive/star-wars-rebellion-manual.pdf)
- [Full-functionality audit index](https://tdimino.github.io/open-rebellion/docs/qa/2026-09-08-full-functionality-audit/)

The audit entry point is deliberately named lowercase `index.md`; this makes
the directory URL above resolve on GitHub Pages instead of requiring the
case-sensitive `INDEX.html` path. The recorded run completed all build,
artifact, status-reporting, and deploy jobs. The only annotation was GitHub's
platform notice that the bundled
`actions/upload-artifact@v4` Node 20 action was forced onto Node 24; it did not
affect the deployment.

The earlier run `34306480934` remains the historical 2026-09-08 baseline. Run
`34505121163` supersedes it for current documentation acceptance.
