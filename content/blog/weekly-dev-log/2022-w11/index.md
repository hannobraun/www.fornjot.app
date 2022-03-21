+++
title = "Weekly Dev Log - 2022-W11"
date  = 2022-03-21
+++

For weeks now, I've been focused on an internal CAD kernel cleanup: reducing the use of the obsolete triangle representation, replacing it with full boundary representation ([#97]), as well as various sub-issues that were in the way of addressing that (most recently [#242]). This work has been blocking the implementation of [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG).

I'm happy to report, that while this work is not fully completed, it's complete enough to start working on CSG for bodies with straight edges and flat faces. Since supporting CSG for those is part of [the milestone I'm currently working on](/blog/straight-edges-flat-faces-simple-sketches-full-csg/), this is good enough to wrap this work up for now and move on to other things.

Before tackling CSG, I decided to go on a bit of a rampage to celebrate, fixing some issues that have been open a bit too long for my taste. One of them, splitting the Fornjot code into smaller libraries ([#141]), has been a bit more substantial, as I took the opportunity to clean up and document each of those new libraries. Work on that is still ongoing.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### App and Documentation

Improvements to the Fornjot application and documentation, that are relevant to end users.

- Fix warning about glyph size on startup ([#337]; thanks to [@therealprof]!)
- Add support for coloring models ([#338], [#343]; thanks to [@therealprof]!)
- Update list of sponsors in README ([#348])
- Support manually specifying triangulation tolerance ([#352], [#359]; thanks to [@mxdamien]!)
- Enable vertex validation ([#354], [#278])
- Rename application to `fj-app` ([#356], [#357])
- Fix errors when running app outside the repository ([#361], [#362], [#364])
- Rename `fj::Union` to `fj::Group` ([#366])
- Add convenient syntax for 2D difference operation ([#372])
- Fix tolerance not being updated on model reload ([#379])
- Fix race condition when loading model initially ([#380])


### Ecosystem

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

- Extract `fj-math` from `fj-app` ([#374], [#375])
- Extract `fj-debug` from `fj-app` ([#376])
- Extract `fj-kernel` from `fj-app` ([#377])
- Extract `fj-operations` from `fj-app` ([#378])
- Extract `fj-host` from `fj-app` ([#381], [#382])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#344], [#345], [#346], [#347], [#349])
- Start automating release procedure ([#325])
- Improve `Shape` API ([#351], [#353])
- Further replace use of triangle representation ([#355])
- Clean up `Model` ([#363])
- Add Cargo workspace ([#368], [#370], [#373]; thanks to [@hendrikmaus]!)


### Issue of the Week

The Fornjot repository contains different models, that each test different features. Often, a bug in the Fornjot kernel will cause a crash, if some of those models are loaded. But this isn't always noticed, allowing bugs to slip through.

If you're looking to get involved in Fornjot, maybe take a look at [#307 - Smoke testing](https://github.com/hannobraun/Fornjot/issues/307). It involves loading all models as part of the regular CI build, so these kinds of bugs no longer have a chance to go unnoticed.


### Outlook

I'm still working on [#141], which I expect to be wrapping up this week. Once that is completed, I want to publish a new release, which is overdue. Then, I'd like to address some long-standing triangulation bugs ([#105], [#143], [#145]). After that, the next big topic is going to be CSG ([#42], [#43], [#44])


[#278]: https://github.com/hannobraun/Fornjot/pull/278
[#325]: https://github.com/hannobraun/Fornjot/pull/325
[#337]: https://github.com/hannobraun/Fornjot/pull/337
[#338]: https://github.com/hannobraun/Fornjot/pull/338
[#343]: https://github.com/hannobraun/Fornjot/pull/343
[#344]: https://github.com/hannobraun/Fornjot/pull/344
[#345]: https://github.com/hannobraun/Fornjot/pull/345
[#346]: https://github.com/hannobraun/Fornjot/pull/346
[#347]: https://github.com/hannobraun/Fornjot/pull/347
[#348]: https://github.com/hannobraun/Fornjot/pull/348
[#349]: https://github.com/hannobraun/Fornjot/pull/349
[#351]: https://github.com/hannobraun/Fornjot/pull/351
[#352]: https://github.com/hannobraun/Fornjot/pull/352
[#353]: https://github.com/hannobraun/Fornjot/pull/353
[#354]: https://github.com/hannobraun/Fornjot/pull/354
[#355]: https://github.com/hannobraun/Fornjot/pull/355
[#356]: https://github.com/hannobraun/Fornjot/pull/356
[#357]: https://github.com/hannobraun/Fornjot/pull/357
[#359]: https://github.com/hannobraun/Fornjot/pull/359
[#361]: https://github.com/hannobraun/Fornjot/pull/361
[#362]: https://github.com/hannobraun/Fornjot/pull/362
[#363]: https://github.com/hannobraun/Fornjot/pull/363
[#364]: https://github.com/hannobraun/Fornjot/pull/364
[#366]: https://github.com/hannobraun/Fornjot/pull/366
[#368]: https://github.com/hannobraun/Fornjot/pull/368
[#370]: https://github.com/hannobraun/Fornjot/pull/370
[#372]: https://github.com/hannobraun/Fornjot/pull/372
[#373]: https://github.com/hannobraun/Fornjot/pull/373
[#374]: https://github.com/hannobraun/Fornjot/pull/374
[#375]: https://github.com/hannobraun/Fornjot/pull/375
[#376]: https://github.com/hannobraun/Fornjot/pull/376
[#377]: https://github.com/hannobraun/Fornjot/pull/377
[#378]: https://github.com/hannobraun/Fornjot/pull/378
[#379]: https://github.com/hannobraun/Fornjot/pull/379
[#380]: https://github.com/hannobraun/Fornjot/pull/380
[#381]: https://github.com/hannobraun/Fornjot/pull/381
[#382]: https://github.com/hannobraun/Fornjot/pull/382

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#105]: https://github.com/hannobraun/Fornjot/issues/105
[#141]: https://github.com/hannobraun/Fornjot/issues/141
[#143]: https://github.com/hannobraun/Fornjot/issues/143
[#145]: https://github.com/hannobraun/Fornjot/issues/145
[#242]: https://github.com/hannobraun/Fornjot/issues/242

[@hendrikmaus]: https://github.com/hendrikmaus
[@mxdamien]: https://github.com/mxdamien
[@therealprof]: https://github.com/therealprof
