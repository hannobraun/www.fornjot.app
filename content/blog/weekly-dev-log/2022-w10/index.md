+++
title = "Weekly Dev Log - 2022-W10"
date  = 2022-03-14
+++

The previous week felt like a mad rush to improve the shape representation within the CAD kernel, making it internally consistent ([#280]). That work had started the week before, and I managed to wrap it up. This should be a real boon going forward, as the new representation is much more reliable, and can detect a lot of bugs before they ever have a chance to affect a user.

There were a few other improvements too, like improvements to the terminal output and a fix for a problem on some Nvidia graphics hardware. Read all about it below!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### User-visible improvements

- Improve terminal output ([#297])
- Update list of sponsors in README ([#305])
- Fix vertex validation warnings in sweep algorithm ([#318])
- Fix crashes on some graphics hardware ([#323])


### Internal improvements

- Fix Clippy warnings ([#292]; special thanks to first-time contributor [@danieleades]!)
- Update dependencies ([#293], [#294], [#295], [#296])
- Prepare for implementation of vertex validation ([#298], [#300])
- Make shape representation internally consistent ([#299], [#301], [#302], [#303], [#304], [#308], [#309], [#310], [#312], [#321], [#322], [#329], [#330], [#331], [#332], [#333], [#334], [#336])
- Clean up 2D difference and union code ([#311], [#317], [#328], [#335])
- Ignore IntelliJ IDEA files ([#313]; special thanks to first-time contributor [@anwentec]!)
- Trigger CI build for all paths ([#314], [#316])
- Update documentation for contributors ([#315])
- Clean up transform code ([#319])
- Clean up topology types ([#320], [#327])
- Update release guidelines ([#326])


### Issue of the week

The Fornjot repository contains a few example models that are used for testing. Sometimes, when changes to the Fornjot kernel are made, those models need to be updated. And sometimes, these updates are forgotten!

If you're looking for a way to get involved with Fornjot, why not check out [#51 - Build models in repository as part of CI build](https://github.com/hannobraun/Fornjot/issues/51)? This would ensure that some idiot (I mean myself) can no longer forget to update the models, when that is necessary.


### Outlook

With [#280] now finished, my focus is back on vertex validation ([#242]). I *think* that everything is in place to wrap that up now. With that done, I can continue working on eliminating triangle representation from the kernel ([#97]), which is blocking work on constructive solid geometry ([CSG](https://en.wikipedia.org/wiki/Constructive_solid_geometry)).


[#292]: https://github.com/hannobraun/Fornjot/pull/292
[#293]: https://github.com/hannobraun/Fornjot/pull/293
[#294]: https://github.com/hannobraun/Fornjot/pull/294
[#295]: https://github.com/hannobraun/Fornjot/pull/295
[#296]: https://github.com/hannobraun/Fornjot/pull/296
[#297]: https://github.com/hannobraun/Fornjot/pull/297
[#298]: https://github.com/hannobraun/Fornjot/pull/298
[#299]: https://github.com/hannobraun/Fornjot/pull/299
[#300]: https://github.com/hannobraun/Fornjot/pull/300
[#301]: https://github.com/hannobraun/Fornjot/pull/301
[#302]: https://github.com/hannobraun/Fornjot/pull/302
[#303]: https://github.com/hannobraun/Fornjot/pull/303
[#304]: https://github.com/hannobraun/Fornjot/pull/304
[#305]: https://github.com/hannobraun/Fornjot/pull/305
[#306]: https://github.com/hannobraun/Fornjot/pull/306
[#307]: https://github.com/hannobraun/Fornjot/pull/307
[#308]: https://github.com/hannobraun/Fornjot/pull/308
[#309]: https://github.com/hannobraun/Fornjot/pull/309
[#310]: https://github.com/hannobraun/Fornjot/pull/310
[#311]: https://github.com/hannobraun/Fornjot/pull/311
[#312]: https://github.com/hannobraun/Fornjot/pull/312
[#313]: https://github.com/hannobraun/Fornjot/pull/313
[#314]: https://github.com/hannobraun/Fornjot/pull/314
[#315]: https://github.com/hannobraun/Fornjot/pull/315
[#316]: https://github.com/hannobraun/Fornjot/pull/316
[#317]: https://github.com/hannobraun/Fornjot/pull/317
[#318]: https://github.com/hannobraun/Fornjot/pull/318
[#319]: https://github.com/hannobraun/Fornjot/pull/319
[#320]: https://github.com/hannobraun/Fornjot/pull/320
[#321]: https://github.com/hannobraun/Fornjot/pull/321
[#322]: https://github.com/hannobraun/Fornjot/pull/322
[#323]: https://github.com/hannobraun/Fornjot/pull/323
[#324]: https://github.com/hannobraun/Fornjot/pull/324
[#325]: https://github.com/hannobraun/Fornjot/pull/325
[#326]: https://github.com/hannobraun/Fornjot/pull/326
[#327]: https://github.com/hannobraun/Fornjot/pull/327
[#328]: https://github.com/hannobraun/Fornjot/pull/328
[#329]: https://github.com/hannobraun/Fornjot/pull/329
[#330]: https://github.com/hannobraun/Fornjot/pull/330
[#331]: https://github.com/hannobraun/Fornjot/pull/331
[#332]: https://github.com/hannobraun/Fornjot/pull/332
[#333]: https://github.com/hannobraun/Fornjot/pull/333
[#334]: https://github.com/hannobraun/Fornjot/pull/334
[#335]: https://github.com/hannobraun/Fornjot/pull/335
[#336]: https://github.com/hannobraun/Fornjot/pull/336

[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#242]: https://github.com/hannobraun/Fornjot/issues/242
[#280]: https://github.com/hannobraun/Fornjot/issues/280

[@danieleades]: https://github.com/danieleades
[@anwentec]: https://github.com/anwentec
