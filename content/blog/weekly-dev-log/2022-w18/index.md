+++
title = "Weekly Dev Log - 2022-W18"
date  = 2022-05-09
+++

My efforts last week were dominated by [publishing version 0.6](https://www.fornjot.app/blog/fornjot-0.6/). It took quite a while to write the changelog, then the release announcement. Finally, I needed to fix some issues with the new release automation (which was to be expected; very happy with the results).

I want to release new versions more regularly in the future, which should make the process smoother and more efficient.

With the release done, I finally returned my attention to implementing [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry), which the cleanup work of the last months was preparing for. I've started with implementing the union operation ([#42]).

In other news, [@connor-lennox] has started contributing by expanding the capabilities of `fj-math`, and cleaning up some of the code that uses it.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Update README ([#516], [#520])
- Update crate metadata ([#519])
- Update top-level documentation of all crates ([#521])
- Publish version 0.6.0 ([#524])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Simplify `Shape` ([#541])
- Expand and update constructors of `Curve`/`Surface` ([#542])
- Implement plane-plane intersection ([#543])

#### `fj-math`

- Expand API, use it in more places ([#523], [#545], [#547]; special thanks to first-time contributor [@connor-lennox]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#509], [#510], [#511], [#512], [#513], [#514], [#517])
- Fix issues with release automation ([#527], [#529], [#530], [#531], [#532], [#534])
- Update release procedure ([#535], [#536])
- Make some small code cleanups ([#540])


### Issue of the Week

Computer tend to be much better at doing tedious stuff than I am. That's why I really appreciate the release automation that we have so far. If you're interested in that kind of thing, why not take a look at [#479 - `release-operator` should automatically determine which crates to publish](https://github.com/hannobraun/Fornjot/issues/479)?


### Outlook

My priority this week is to continue the work on the union operation ([#42]). I have a pretty good understanding of the next few steps, and I hope to get those done relatively quickly. After that, there are a few more details to figure out, before I can finish implementing the algorithm.


[#509]: https://github.com/hannobraun/Fornjot/pull/509
[#510]: https://github.com/hannobraun/Fornjot/pull/510
[#511]: https://github.com/hannobraun/Fornjot/pull/511
[#512]: https://github.com/hannobraun/Fornjot/pull/512
[#513]: https://github.com/hannobraun/Fornjot/pull/513
[#514]: https://github.com/hannobraun/Fornjot/pull/514
[#516]: https://github.com/hannobraun/Fornjot/pull/516
[#517]: https://github.com/hannobraun/Fornjot/pull/517
[#519]: https://github.com/hannobraun/Fornjot/pull/519
[#520]: https://github.com/hannobraun/Fornjot/pull/520
[#521]: https://github.com/hannobraun/Fornjot/pull/521
[#523]: https://github.com/hannobraun/Fornjot/pull/523
[#524]: https://github.com/hannobraun/Fornjot/pull/524
[#527]: https://github.com/hannobraun/Fornjot/pull/527
[#529]: https://github.com/hannobraun/Fornjot/pull/529
[#530]: https://github.com/hannobraun/Fornjot/pull/530
[#531]: https://github.com/hannobraun/Fornjot/pull/531
[#532]: https://github.com/hannobraun/Fornjot/pull/532
[#534]: https://github.com/hannobraun/Fornjot/pull/534
[#535]: https://github.com/hannobraun/Fornjot/pull/535
[#536]: https://github.com/hannobraun/Fornjot/pull/536
[#540]: https://github.com/hannobraun/Fornjot/pull/540
[#541]: https://github.com/hannobraun/Fornjot/pull/541
[#542]: https://github.com/hannobraun/Fornjot/pull/542
[#543]: https://github.com/hannobraun/Fornjot/pull/543
[#545]: https://github.com/hannobraun/Fornjot/pull/545
[#547]: https://github.com/hannobraun/Fornjot/pull/547

[#42]: https://github.com/hannobraun/Fornjot/issues/42

[@connor-lennox]: https://github.com/connor-lennox
