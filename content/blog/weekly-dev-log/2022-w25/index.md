+++
title = "Weekly Dev Log - 2022-W25"
date  = 2022-06-27
+++

I'm working on removing the `Shape` data structure ([#697]), which is going to make the kernel much simpler, and thus all following work much easier. I already finished decoupling the validation infrastructure from `Shape` ([#696]), which is a necessary precondition, but also turned out to be a nice improvement in its own right.

I've started to rewrite the sweep algorithm ([#723]), which has been a thorn in my side for a while, and is now blocking further progress on removing `Shape`.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- None this week. Busy working on the kernel!


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Decouple validation infrastructure from `Shape` ([#705], [#706], [#707], [#709], [#710], [#711], [#714], [#715])
- Derive `Copy` for `Vertex` ([#712])
- Make Anyhow a `dev-dependency` of `fj-kernel` ([#713])
- Break `triangulate`'s dependency on `Shape` ([#716])
- Don't check uniqueness of edges during validation ([#718])
- Relax builder API ([#719])
- Add `Curve::line_from_points` ([#721])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#698], [#699], [#700], [#701], [#703], [#704])
- Clean up `Shape` tests ([#708])
- Clean up module structure within `objects` ([#717])
- Make minor cleanups in approximation code ([#720])
- Increase variation in test model ([#722])


### Issue of the Week

Fornjot uses [wgpu-rs](https://crates.io/crates/wgpu) to display 3D graphics. This is a great library, and it supports all major desktop and mobile platforms, as well as browsers. However, wgpu-rs is also fairly low-level. If there were a higher-level library we could use, without limiting Fornjot's potential for portability, that could make development of the graphical aspects much easier going forward.

Enter [rend3](https://crates.io/crates/rend3). It is built on top of wgpu-rs and makes a great impression overall, but I haven't had the time to take a closer look at it. If you're interested in 3D graphics code, why not look into [#657 - Evaluate rend3](https://github.com/hannobraun/Fornjot/issues/657)?


### Outlook

I hope to wrap up the rewrite of the sweep algorithm this week ([#723]), and then see what's missing to remove `Shape` ([#697]). Progress so far has been promising, so I'm optimistic.


[#698]: https://github.com/hannobraun/Fornjot/pull/698
[#699]: https://github.com/hannobraun/Fornjot/pull/699
[#700]: https://github.com/hannobraun/Fornjot/pull/700
[#701]: https://github.com/hannobraun/Fornjot/pull/701
[#703]: https://github.com/hannobraun/Fornjot/pull/703
[#704]: https://github.com/hannobraun/Fornjot/pull/704
[#705]: https://github.com/hannobraun/Fornjot/pull/705
[#706]: https://github.com/hannobraun/Fornjot/pull/706
[#707]: https://github.com/hannobraun/Fornjot/pull/707
[#708]: https://github.com/hannobraun/Fornjot/pull/708
[#709]: https://github.com/hannobraun/Fornjot/pull/709
[#710]: https://github.com/hannobraun/Fornjot/pull/710
[#711]: https://github.com/hannobraun/Fornjot/pull/711
[#712]: https://github.com/hannobraun/Fornjot/pull/712
[#713]: https://github.com/hannobraun/Fornjot/pull/713
[#714]: https://github.com/hannobraun/Fornjot/pull/714
[#715]: https://github.com/hannobraun/Fornjot/pull/715
[#716]: https://github.com/hannobraun/Fornjot/pull/716
[#717]: https://github.com/hannobraun/Fornjot/pull/717
[#718]: https://github.com/hannobraun/Fornjot/pull/718
[#719]: https://github.com/hannobraun/Fornjot/pull/719
[#720]: https://github.com/hannobraun/Fornjot/pull/720
[#721]: https://github.com/hannobraun/Fornjot/pull/721
[#722]: https://github.com/hannobraun/Fornjot/pull/722
[#723]: https://github.com/hannobraun/Fornjot/pull/723

[#696]: https://github.com/hannobraun/Fornjot/issues/696
[#697]: https://github.com/hannobraun/Fornjot/issues/697
