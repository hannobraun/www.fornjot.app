+++
title = "Weekly Dev Log - 2022-W08"
date  = 2022-02-28
+++

It's been another productive week of cleanups! I continue working towards addressing [#97], although I've been forced to take a bit of a detour through [#176] and [#242]. Sometimes I worry that this lack of user-visible changes over the last few weeks looks, from the outside, as if Fornjot isn't making any real progress. But I can assure you, this is not true!

All those cleanups happen in response to problems that I encounter while working towards supporting [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG) operations. And every problem forces me to come up with a solution that makes the CAD kernel a bit more flexible, robust, and better able to handle future challenges.

This is necessary work, and I'm really excited about the progress I'm making. I hope you are too!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### User-visible improvements

- Update README ([#222], [#234])


### Internal improvements

- Update dependencies ([#224], [#225], [#226], [#227])
- Clean up `Vertex` ([#228], [#231], [#236], [#255])
- Make some cleanups in topology code ([#229], [#244])
- Fix some Clippy warnings ([#232]; thanks to [@therealprof]!)
- Add Bacon configuration file ([#233]; thanks to [@hendrikmaus]!)
- Clean up some things in `math` module ([#235], [#237], [#243], [#252])
- Validate `Segment` and `Triangle` on construction ([#238], [#239], [#245], [#249], [#251]; thanks to [@therealprof]!)
- Clean up approximation code ([#245], [#247])
- Make sweep operation slightly more correct ([#248])
- Simplify creation of boundary representation ([#254])
- Add new `algorithms` module ([#256])
- Fix continuous deployment workflow ([#257]; thanks to [@hendrikmaus]!)


### Issue of the week

If you're interested in getting involved with Fornjot but aren't sure where to start, why not look into issue [#9 - Make `--model` parameter of host application more flexible](https://github.com/hannobraun/Fornjot/issues/9)?

Fornjot tries to load models from a `models/` directory, which is convenient for development, because we have such a directory in the repository. But it's one of those things that make Fornjot *in*convenient, if someone tries to use Fornjot outside of the repository.

This issue is a small step towards making Fornjot more usable, while also being relatively small and self-contained, making it a good first issue for new Fornjot contributors.


### Outlook

As I alluded to above, I'm currently on a bit of a detour on my way to fixing [#97]. I've already started working towards [#242], which I hope to wrap up quickly. After that, I'll be in a good position to clean up, and write a test suite for, the sweep operation. This should put [#97] into reach, as the sweep operation is the last piece of code that still resists that change.


[#222]: https://github.com/hannobraun/Fornjot/pull/222
[#224]: https://github.com/hannobraun/Fornjot/pull/224
[#225]: https://github.com/hannobraun/Fornjot/pull/225
[#226]: https://github.com/hannobraun/Fornjot/pull/226
[#227]: https://github.com/hannobraun/Fornjot/pull/227
[#228]: https://github.com/hannobraun/Fornjot/pull/228
[#229]: https://github.com/hannobraun/Fornjot/pull/229
[#231]: https://github.com/hannobraun/Fornjot/pull/231
[#232]: https://github.com/hannobraun/Fornjot/pull/232
[#233]: https://github.com/hannobraun/Fornjot/pull/233
[#234]: https://github.com/hannobraun/Fornjot/pull/234
[#235]: https://github.com/hannobraun/Fornjot/pull/235
[#236]: https://github.com/hannobraun/Fornjot/pull/236
[#237]: https://github.com/hannobraun/Fornjot/pull/237
[#238]: https://github.com/hannobraun/Fornjot/pull/238
[#239]: https://github.com/hannobraun/Fornjot/pull/239
[#243]: https://github.com/hannobraun/Fornjot/pull/243
[#244]: https://github.com/hannobraun/Fornjot/pull/244
[#245]: https://github.com/hannobraun/Fornjot/pull/245
[#247]: https://github.com/hannobraun/Fornjot/pull/247
[#248]: https://github.com/hannobraun/Fornjot/pull/248
[#249]: https://github.com/hannobraun/Fornjot/pull/249
[#251]: https://github.com/hannobraun/Fornjot/pull/251
[#252]: https://github.com/hannobraun/Fornjot/pull/252
[#254]: https://github.com/hannobraun/Fornjot/pull/254
[#255]: https://github.com/hannobraun/Fornjot/pull/255
[#256]: https://github.com/hannobraun/Fornjot/pull/256
[#257]: https://github.com/hannobraun/Fornjot/pull/257

[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#176]: https://github.com/hannobraun/Fornjot/issues/176
[#242]: https://github.com/hannobraun/Fornjot/issues/242

[@therealprof]: https://github.com/therealprof
[@hendrikmaus]: https://github.com/hendrikmaus
