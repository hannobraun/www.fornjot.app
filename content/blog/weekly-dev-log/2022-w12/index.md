+++
title = "Weekly Dev Log - 2022-W12"
date  = 2022-03-29
+++

Sorry for being a bit late with this Weekly Dev Log! Yesterday was a crazy day, for multiple reasons (mostly good), and getting this done simply wasn't in the cards.

I've been working on [#141], which went pretty well until I started to extract the triangle mesh data structure into a separate crate. I realized that this code could use a good cleanup, which required changes to the triangulation code ([#105]), which required more clean-ups... the usual virtuous cycle of getting distracted by more and more productive tasks.

Unfortunately, that didn't lead to the same mad rush of activity that I've become accustomed to over the last few weeks. Some of the those cleanups required a lot of *thinking*, a really tedious activity that tends to distract from all the *doing* I'd prefer to get into instead.

Can't be helped! Things are moving in the right direction, which is what counts.

In other news, I got to merge a lot of contributions last week, some of those from first-time contributors, fixing warnings in Fornjot's output, cleaning up code, as well as improving the CI build and the CD infrastructure.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### App and Documentation

Improvements to the Fornjot application and documentation, that are relevant to end users.

- Fix warnings in output due to misuse of graphics API ([#397]; special thanks to first-time contributor [@liubog2008]!)


### Ecosystem

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Add `Curve::x_axis` ([#398])
- Distinguish between exterior and interior cycles ([#401])
- Add convenient APIs to build `Vertex` and `Edge` ([#403])

#### `fj-math`

- Add methods to construct unit vectors ([#390])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Consolidate platform-specific code behind trait ([#383], [#384]; special thanks to first-time contributor [@ozghimire]!)
- Update dependencies ([#385])
- Consolidate triangulation code in single module ([#386])
- Clean up approximation code ([#388])
- Run models as part of CI build ([#389]; special thanks to first-time contributor [@homersimpsons]!)
- Create GitHub Release via CD workflow ([#391]; thanks to [@hendrikmaus]!)
- Fix and improve CI/CD build ([#392], [#393], [#396]; thanks to [@hendrikmaus]!)
- Clean up internal implementation of `Shape` API ([#402], [#404])


### Issue of the Week

Fornjot support sweeping 2D sketches into 3D solids, along a straight line. Right now, sweeping is only possible along the z-axis, although on the kernel side, all code that's involved in that accepts an arbitrary vector.

If you want to get involved with Fornjot, why not give [#192 - Support sweeping in arbitrary directions](https://github.com/hannobraun/Fornjot/issues/192) a try?


### Outlook

My office computer broke! This is not catastrophic, my laptop can fill in for now, but it does throw all of my short-term plans into disarray. While the office PC is out for repair, I don't have access to all my recent work-in-progress.

Chances are, I'll have it back and in working order later this week, so I don't think it makes sense to re-create any work that's on there. I'll find other tasks to fill the time with, but at this time, I have no idea what those might be.


[#383]: https://github.com/hannobraun/Fornjot/pull/383
[#384]: https://github.com/hannobraun/Fornjot/pull/384
[#385]: https://github.com/hannobraun/Fornjot/pull/385
[#386]: https://github.com/hannobraun/Fornjot/pull/386
[#388]: https://github.com/hannobraun/Fornjot/pull/388
[#389]: https://github.com/hannobraun/Fornjot/pull/389
[#390]: https://github.com/hannobraun/Fornjot/pull/390
[#391]: https://github.com/hannobraun/Fornjot/pull/391
[#392]: https://github.com/hannobraun/Fornjot/pull/392
[#393]: https://github.com/hannobraun/Fornjot/pull/393
[#396]: https://github.com/hannobraun/Fornjot/pull/396
[#397]: https://github.com/hannobraun/Fornjot/pull/397
[#398]: https://github.com/hannobraun/Fornjot/pull/398
[#401]: https://github.com/hannobraun/Fornjot/pull/401
[#402]: https://github.com/hannobraun/Fornjot/pull/402
[#403]: https://github.com/hannobraun/Fornjot/pull/403
[#404]: https://github.com/hannobraun/Fornjot/pull/404

[#105]: https://github.com/hannobraun/Fornjot/issues/105
[#141]: https://github.com/hannobraun/Fornjot/issues/141

[@hendrikmaus]: https://github.com/hendrikmaus
[@liubog2008]: https://github.com/liubog2008
[@homersimpsons]: https://github.com/homersimpsons
[@ozghimire]: https://github.com/ozghimire
