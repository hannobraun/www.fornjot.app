+++
title = "Weekly Dev Log - 2022-W20"
date  = 2022-05-23
+++

My focus last week was the ongoing change to approximating faces in surface coordinates ([#568]), a cleanup that is necessary to make further progress on implementing the union operation ([#42]). This work necessitated some further cleanups ([#399], [#601], [#602]), which I've completed.

Meanwhile, [@chrisprice] added support for exporting to STL, while [@freylint] added optional Serde support to the `fj` crate.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Update list of sponsors ([#585])
- Add support for exporting to STL ([#594], [#599], [#604]; thanks to [@chrisprice]!)
- Remove restrictions from 2D difference operation ([#598])
- Add Serde support to `fj` ([#610]; thanks to [@freylint]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Close down `LocalForm` ([#588])
- Continue introduction of local coordinates ([#589], [#590], [#591], [#592], [#593])
- Extend `Shape` API ([#595], [#605], [#608])
- Make working with b-rep faces easier ([#597])
- Prefer returning cloned `Handle`s ([#606])
- Improve error message for structural validation failure ([#607])
- Add `Surface::plane_from_points` ([#611])

#### `fj-math`

- Add `Triangle::normal` ([#600]; thanks to [@chrisprice]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#586], [#587])
- Simplify names of `topology` modules ([#596])
- Make some cleanups within `shape` module ([#603])
- Clean up sweep algorithm ([#609], [#612])


### Issue of the Week

The `fj` library, which users use to define CAD models in Fornjot, uses straight floating point numbers to represent angles, and interprets those as radians. This has lead to confusion in the past, as users assumed those numbers represented degrees.

Let's fix this once and for all with [#263 - Use static typing to distinguish between degrees and radians](https://github.com/hannobraun/Fornjot/issues/263). Interested in getting an introduction to the `fj` library? This might be the issue for you!


### Outlook

The priority for the coming week remains [#568], and I have lots of unfinished code in local branches that I need to polish up and get merged. There are bugs though, and the lack of geometric validation ([#613]) is making it difficult to figure out what's wrong. That's what I will address first.


[#585]: https://github.com/hannobraun/Fornjot/pull/585
[#586]: https://github.com/hannobraun/Fornjot/pull/586
[#587]: https://github.com/hannobraun/Fornjot/pull/587
[#588]: https://github.com/hannobraun/Fornjot/pull/588
[#589]: https://github.com/hannobraun/Fornjot/pull/589
[#590]: https://github.com/hannobraun/Fornjot/pull/590
[#591]: https://github.com/hannobraun/Fornjot/pull/591
[#592]: https://github.com/hannobraun/Fornjot/pull/592
[#593]: https://github.com/hannobraun/Fornjot/pull/593
[#594]: https://github.com/hannobraun/Fornjot/pull/594
[#595]: https://github.com/hannobraun/Fornjot/pull/595
[#596]: https://github.com/hannobraun/Fornjot/pull/596
[#597]: https://github.com/hannobraun/Fornjot/pull/597
[#598]: https://github.com/hannobraun/Fornjot/pull/598
[#599]: https://github.com/hannobraun/Fornjot/pull/599
[#600]: https://github.com/hannobraun/Fornjot/pull/600
[#603]: https://github.com/hannobraun/Fornjot/pull/603
[#604]: https://github.com/hannobraun/Fornjot/pull/604
[#605]: https://github.com/hannobraun/Fornjot/pull/605
[#606]: https://github.com/hannobraun/Fornjot/pull/606
[#607]: https://github.com/hannobraun/Fornjot/pull/607
[#608]: https://github.com/hannobraun/Fornjot/pull/608
[#609]: https://github.com/hannobraun/Fornjot/pull/609
[#610]: https://github.com/hannobraun/Fornjot/pull/610
[#611]: https://github.com/hannobraun/Fornjot/pull/611
[#612]: https://github.com/hannobraun/Fornjot/pull/612

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#399]: https://github.com/hannobraun/Fornjot/issues/399
[#568]: https://github.com/hannobraun/Fornjot/issues/568
[#601]: https://github.com/hannobraun/Fornjot/issues/601
[#602]: https://github.com/hannobraun/Fornjot/issues/602
[#613]: https://github.com/hannobraun/Fornjot/issues/613

[@chrisprice]: https://github.com/chrisprice
[@freylint]: https://github.com/freylint
