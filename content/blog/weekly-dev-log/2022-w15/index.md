+++
title = "Weekly Dev Log - 2022-W15 (Pre-Vacation Edition)"
date  = 2022-04-15
+++

It's been another good (if short) week! You might notice, I'm publishing this on a Friday, not as usual on a Monday. That's because as of now, I'm on vacation! I didn't want to delay the dev log for two weeks, so here we go with an earlier one.

I finally wrapped up splitting the Fornjot app into self-contained libraries ([#141]), an effort that has gone on for a few weeks now (with interruptions). After that, I initially planned to start preparing for the next release. But it came to my attention that exported 3MF files were not valid, so I decided to focus on that.

I managed to fix 3MF export ([#451]), as well as a display issue that had the same root cause ([#339]). I added validation of exported 3MF files in the CI build, making sure issues like this won't creep in again as easily ([#54]).

Meanwhile, [@hendrikmaus] has continued his tireless work on release automation ([#104]), which I'll hopefully get to try out for the first time in a few weeks.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix triangulation edge case ([#453])
- Fix 3MF export and display issues with transparent faces ([#484])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

- Extract `fj-export` from `fj-app` ([#472], [#473])
- Extract `fj-viewer` from `fj-app` ([#474])

#### `fj-host`

- Add dedicated type for model parameters ([#466])

#### `fj-interop`

- Rename `fj-debug` to `fj-interop` ([#454])
- Move `Mesh` to `fj-interop` and clean it up a little ([#456], [#457], [#459], [#460], [#462])

#### `fj-kernel`

- Add support for arcs ([#483])

#### `fj-math`

- Remove `color` field from `fj_math::Triangle` ([#458])
- Make `Triangle::from_points` more flexible ([#461])
- Implement `fmt::Display` for `Scalar` ([#464])
- Implement `Neg` for `Vector` ([#482])

#### `fj-operations`

- Move `ShapeProcessor` to `fj-operations` ([#467])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#449], [#450], [#452])
- Clean up triangulation code ([#455])
- Add `Tolerance` struct to enforce validity of tolerance values ([#465])
- Continue automating release process ([#468], [#469], [#470]; thanks to [@hendrikmaus]!)
- Clean up directory structure in repository ([#475])
- Validate exported 3MF files in CI build ([#476], [#480], [#481])


### Issue of the Week

People make mistakes. *I* make mistakes. Sometimes I don't notice them, and get bitten by them later. It would be nice to prevent that, by having a robot tell me how dumb I am, and that I need to be more careful.

If that kind of thing is interesting to you, why not take a look at [#477 - CI build should fail, if crates are not part of workspace][#477]? This is the kind of thing that's not a big task for a newcomer, but can be very helpful going forward. (And if you're looking for more along those lines, there's also [#478].)


### Outlook

I'll be on vacation for the next two weeks! I *might* do some work during that time, but that will be completely governed by what I happen to be in the mood for. If that's something noteworthy, I'm going to publish a post-vacation dev log on May 2nd. If not, I'll be back with a regular one on May 9th.

Unless I learn about more critical bugs, my immediate priority after the vacation is going to be the next release. That will require a tiny bit of cleanup (READMEs, crate metadata, etc.), as well as a changelog and release announcement that will probably take days to write (I've waited too long).


[#449]: https://github.com/hannobraun/Fornjot/pull/449
[#450]: https://github.com/hannobraun/Fornjot/pull/450
[#452]: https://github.com/hannobraun/Fornjot/pull/452
[#453]: https://github.com/hannobraun/Fornjot/pull/453
[#454]: https://github.com/hannobraun/Fornjot/pull/454
[#455]: https://github.com/hannobraun/Fornjot/pull/455
[#456]: https://github.com/hannobraun/Fornjot/pull/456
[#457]: https://github.com/hannobraun/Fornjot/pull/457
[#458]: https://github.com/hannobraun/Fornjot/pull/458
[#459]: https://github.com/hannobraun/Fornjot/pull/459
[#460]: https://github.com/hannobraun/Fornjot/pull/460
[#461]: https://github.com/hannobraun/Fornjot/pull/461
[#462]: https://github.com/hannobraun/Fornjot/pull/462
[#464]: https://github.com/hannobraun/Fornjot/pull/464
[#465]: https://github.com/hannobraun/Fornjot/pull/465
[#466]: https://github.com/hannobraun/Fornjot/pull/466
[#467]: https://github.com/hannobraun/Fornjot/pull/467
[#468]: https://github.com/hannobraun/Fornjot/pull/468
[#469]: https://github.com/hannobraun/Fornjot/pull/469
[#470]: https://github.com/hannobraun/Fornjot/pull/470
[#472]: https://github.com/hannobraun/Fornjot/pull/472
[#473]: https://github.com/hannobraun/Fornjot/pull/473
[#474]: https://github.com/hannobraun/Fornjot/pull/474
[#475]: https://github.com/hannobraun/Fornjot/pull/475
[#476]: https://github.com/hannobraun/Fornjot/pull/476
[#480]: https://github.com/hannobraun/Fornjot/pull/480
[#481]: https://github.com/hannobraun/Fornjot/pull/481
[#482]: https://github.com/hannobraun/Fornjot/pull/482
[#483]: https://github.com/hannobraun/Fornjot/pull/483
[#484]: https://github.com/hannobraun/Fornjot/pull/484

[#54]: https://github.com/hannobraun/Fornjot/issues/54
[#104]: https://github.com/hannobraun/Fornjot/issues/104
[#141]: https://github.com/hannobraun/Fornjot/issues/141
[#339]: https://github.com/hannobraun/Fornjot/issues/339
[#451]: https://github.com/hannobraun/Fornjot/issues/451
[#477]: https://github.com/hannobraun/Fornjot/issues/477
[#478]: https://github.com/hannobraun/Fornjot/issues/478

[@hendrikmaus]: https://github.com/hendrikmaus
