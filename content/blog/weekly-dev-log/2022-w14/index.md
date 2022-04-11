+++
title = "Weekly Dev Log - 2022-W14"
date  = 2022-04-11
+++

It's been a good week! After all the distractions caused by my broken computer, I'm finally back to full productivity. The replacement arrived last week!

I managed to wrap up my work on improving the testing infrastructure for the triangulation algorithm, and managed to fix some of the most annoying bugs ([#105], [#143]). I also fixed a few bugs with the graphics code, that were surfaced by me having access to an AMD GPU now.

Meanwhile [@hendrikmaus] continued his work on release automation ([#104]). I'm still looking forward to trying that, but there are a few tasks I need to check off my list before I'm ready for a new release.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to the Fornjot application, the API for creating models, and documentation.

- Update README ([#419])
- Fix issues with an AMD GPU ([#437])
- Fix some triangulation issues ([#448])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-debug`

- Simplify debug info ([#447])

#### `fj-kernel`

- Expand topology builder API ([#425], [#428], [#432])
- Fix wrong word in doc comment ([#426])
- Improve representation of approximations ([#427], [#429], [#433])

#### `fj-math`

- Improve `Triangle` ([#434])
- Expand API of `Segment` ([#441], [#442])
- Add `PolyChain` ([#444], [#446])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#420], [#421], [#422], [#424])
- Improve testing infrastructure of triangulation ([#435], [#439], [#440], [#443], [#445])
- Upgrade to latest Rust version ([#438])
- Continue automation of release procedure ([#436]; thanks to [@hendrikmaus]!)


### Issue of the Week

If you open a model in the Fornjot application, then make changes to its source code, it automatically reloads. However, there's a longstanding issue about this not working on macOS, [#12 - Reloading the model doesn't work on macOS](https://github.com/hannobraun/Fornjot/issues/12).

If you're running macOS, maybe you'd be interested in taking a look? Your contribution could be as small as verifying that the issue is still present, as large as figuring out what's wrong and submitting a fix, or anything in between. Since I don't have access to macOS, any help in that area is especially appreciated!


### Outlook

With [#105] wrapped up, I'll be taking a look at the remaining open triangulation issues, to update them and figure out, if they're still present. I don't think I'll address any remaining ones at this point though. I think it's more prudent to wrap up my work on a modular Fornjot ecosystem ([#141]), then start working on getting a release out. It's long overdue!

Friday is a public holiday here, so it'll be a shorter work week for me. In addition, I'll be on vacation for two weeks, starting next week. For this reason, I plan to publish the next dev log on Thursday or Friday.


[#419]: https://github.com/hannobraun/Fornjot/pull/419
[#420]: https://github.com/hannobraun/Fornjot/pull/420
[#421]: https://github.com/hannobraun/Fornjot/pull/421
[#422]: https://github.com/hannobraun/Fornjot/pull/422
[#424]: https://github.com/hannobraun/Fornjot/pull/424
[#425]: https://github.com/hannobraun/Fornjot/pull/425
[#426]: https://github.com/hannobraun/Fornjot/pull/426
[#427]: https://github.com/hannobraun/Fornjot/pull/427
[#428]: https://github.com/hannobraun/Fornjot/pull/428
[#429]: https://github.com/hannobraun/Fornjot/pull/429
[#432]: https://github.com/hannobraun/Fornjot/pull/432
[#433]: https://github.com/hannobraun/Fornjot/pull/433
[#434]: https://github.com/hannobraun/Fornjot/pull/434
[#435]: https://github.com/hannobraun/Fornjot/pull/435
[#436]: https://github.com/hannobraun/Fornjot/pull/436
[#437]: https://github.com/hannobraun/Fornjot/pull/437
[#438]: https://github.com/hannobraun/Fornjot/pull/438
[#439]: https://github.com/hannobraun/Fornjot/pull/439
[#440]: https://github.com/hannobraun/Fornjot/pull/440
[#441]: https://github.com/hannobraun/Fornjot/pull/441
[#442]: https://github.com/hannobraun/Fornjot/pull/442
[#443]: https://github.com/hannobraun/Fornjot/pull/443
[#444]: https://github.com/hannobraun/Fornjot/pull/444
[#445]: https://github.com/hannobraun/Fornjot/pull/445
[#446]: https://github.com/hannobraun/Fornjot/pull/446
[#447]: https://github.com/hannobraun/Fornjot/pull/447
[#448]: https://github.com/hannobraun/Fornjot/pull/448

[#104]: https://github.com/hannobraun/Fornjot/issues/104
[#105]: https://github.com/hannobraun/Fornjot/issues/105
[#141]: https://github.com/hannobraun/Fornjot/issues/141
[#143]: https://github.com/hannobraun/Fornjot/issues/143

[@hendrikmaus]: https://github.com/hendrikmaus
