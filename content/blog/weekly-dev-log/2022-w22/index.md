+++
title = "Weekly Dev Log - 2022-W22"
date  = 2022-06-07
+++

Sorry for being a day late with this dev log! Yesterday was a public holiday here in Germany, and I spent most of it away from the computer.

I'm still working on [#568], a cleanup that will pave the way for the implementation of boolean operations ([#42], [#43], [#44]). I seem to be stuck. It's very clear to me what needs to happen on a high level, but every time I dive down into the specifics, various problems prevent me from making progress. I'm going wrong somewhere, and I need to figure out where that is.

Fortunately that effort has resulted in lots of cleanup (in an effort to understand the code in question better), and improvements to the CAD kernel's debug and validation infrastructure.

Meanwhile, it has been a great week of contributor work! [@gabsi26] implemented a procedural macro to make specifying models much more convenient, [@chrisprice] has been working on making the input/camera code work like it should, and [@kamirr] has fixed a long-standing memory leak.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Make specifying model parameters more convenient ([#643], [#652], [#655], [#659]; thanks to [@gabsi26]!)
- Fix model rotation behavior ([#644]; thanks to [@chrisprice]!)
- Fix memory leak in `fj::Sketch` ([#646]; special thanks to first-time contributor [@kamirr]!)
- Update README ([#660])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Make `Mapping` API more convenient ([#661])
- Improve validation infrastructure ([#662], [#668])
- Require surface coordinates when building faces or cycles ([#665])
- Make it possible to assign labels to `Shape`s ([#666])
- Add custom data type to represent edge vertices ([#667])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#645], [#647])
- Fix Clippy warning ([#648])
- Clean up sweep algorithm ([#649], [#650], [#651], [#654], [#663])
- Update contribution guidelines ([#664])


### Issue of the Week

Making changes to the source code of the currently loaded model should automatically reload that model, leading to the updated model being visible shortly after. There's an older issue about this behavior, [#12 - Reloading the model doesn't work on macOS](https://github.com/hannobraun/Fornjot/issues/12).

If you're a macOS user, why not take a look? It's not quite clear whether this issue is still present, so even testing that and reporting back would be a big help!


### Outlook

My priority for this week remains [#568]. I've been trying to take a step back and clear my head over the weekend, so hopefully I can take a fresh look and figure out what the problem with my current approach is.


[#643]: https://github.com/hannobraun/Fornjot/pull/643
[#644]: https://github.com/hannobraun/Fornjot/pull/644
[#645]: https://github.com/hannobraun/Fornjot/pull/645
[#646]: https://github.com/hannobraun/Fornjot/pull/646
[#647]: https://github.com/hannobraun/Fornjot/pull/647
[#648]: https://github.com/hannobraun/Fornjot/pull/648
[#649]: https://github.com/hannobraun/Fornjot/pull/649
[#650]: https://github.com/hannobraun/Fornjot/pull/650
[#651]: https://github.com/hannobraun/Fornjot/pull/651
[#652]: https://github.com/hannobraun/Fornjot/pull/652
[#654]: https://github.com/hannobraun/Fornjot/pull/654
[#655]: https://github.com/hannobraun/Fornjot/pull/655
[#659]: https://github.com/hannobraun/Fornjot/pull/659
[#660]: https://github.com/hannobraun/Fornjot/pull/660
[#661]: https://github.com/hannobraun/Fornjot/pull/661
[#662]: https://github.com/hannobraun/Fornjot/pull/662
[#663]: https://github.com/hannobraun/Fornjot/pull/663
[#664]: https://github.com/hannobraun/Fornjot/pull/664
[#665]: https://github.com/hannobraun/Fornjot/pull/665
[#666]: https://github.com/hannobraun/Fornjot/pull/666
[#667]: https://github.com/hannobraun/Fornjot/pull/667
[#668]: https://github.com/hannobraun/Fornjot/pull/668

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#568]: https://github.com/hannobraun/Fornjot/issues/568

[@chrisprice]: https://github.com/chrisprice
[@gabsi26]: https://github.com/gabsi26
[@kamirr]: https://github.com/kamirr
