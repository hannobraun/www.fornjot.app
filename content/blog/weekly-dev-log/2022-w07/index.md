+++
title = "Weekly Dev Log - 2022-W07"
date  = 2022-02-21
+++

I started out last week by continuing my work from the previous one: to use boundary representation for *all* shapes. It's being used for most, but there's still one last hold-out that's based on an older representation ([#97]). Addressing that is necessary before I can start working on support for [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG) operations.

I made some good progress on that early in the week, then let myself get distracted by another bit of cleanup work. Writing math code sometimes required some annoying workarounds to do certain tasks ([#193]). This wasn't serious enough to be an immediate priority, but somehow I got fed up enough about it that I just went and fixed it.

That was a lot of work, but it's probably better this way. Having better tools at your disposal is good for productivity, so this will certainly pay off in the long run.


### Sponsors

Fornjot is supported by [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### User-visible improvements

- Add UI element that shows size of model bounding box ([#217]; special thanks to first-time contributor [Daniel Egger]!)
- Fix example in README ([#218]; thanks to [Daniel Egger]!)


### Internal improvements

- Update dependencies ([#181])
- Replace planes with surfaces created by sweeping curves ([#182], [#183], [#184], [#190])
- Make various minor cleanups in CAD kernel ([#185], [#189], [#191])
- Add custom math types that implement `Eq`/`Ord`/`Hash` ([#194], [#195], [#196], [#197], [#198], [#199], [#200], [#203], [#204], [#205], [#209], [#210], [#211], [#212], [#214], [#215], [#216], [#221])
- Clean up the CI build ([#206], [#207]; thanks to [Hendrik Maus]!)


### Website

- Add Weekly Dev Log ([#31])


### Outlook

Now that I got the math code squared away, I'm back on track to address [#97]. I feel like I'm getting pretty close to finally fixing that, but it's hard to say how long it will still take.

I can already see some issues I need to address first, like improving the approximation and triangulation code to support faces that are continuous, meaning they connect to themselves (like the outside of a cylinder). And I'm sure there are more problems that will pop up as I'm getting closer.


[#181]: https://github.com/hannobraun/Fornjot/pull/181
[#182]: https://github.com/hannobraun/Fornjot/pull/182
[#183]: https://github.com/hannobraun/Fornjot/pull/183
[#184]: https://github.com/hannobraun/Fornjot/pull/184
[#185]: https://github.com/hannobraun/Fornjot/pull/185
[#189]: https://github.com/hannobraun/Fornjot/pull/189
[#190]: https://github.com/hannobraun/Fornjot/pull/190
[#191]: https://github.com/hannobraun/Fornjot/pull/191
[#194]: https://github.com/hannobraun/Fornjot/pull/194
[#195]: https://github.com/hannobraun/Fornjot/pull/195
[#196]: https://github.com/hannobraun/Fornjot/pull/196
[#197]: https://github.com/hannobraun/Fornjot/pull/197
[#198]: https://github.com/hannobraun/Fornjot/pull/198
[#199]: https://github.com/hannobraun/Fornjot/pull/199
[#200]: https://github.com/hannobraun/Fornjot/pull/200
[#203]: https://github.com/hannobraun/Fornjot/pull/203
[#204]: https://github.com/hannobraun/Fornjot/pull/204
[#205]: https://github.com/hannobraun/Fornjot/pull/205
[#206]: https://github.com/hannobraun/Fornjot/pull/206
[#207]: https://github.com/hannobraun/Fornjot/pull/207
[#209]: https://github.com/hannobraun/Fornjot/pull/209
[#210]: https://github.com/hannobraun/Fornjot/pull/210
[#211]: https://github.com/hannobraun/Fornjot/pull/211
[#212]: https://github.com/hannobraun/Fornjot/pull/212
[#214]: https://github.com/hannobraun/Fornjot/pull/214
[#215]: https://github.com/hannobraun/Fornjot/pull/215
[#216]: https://github.com/hannobraun/Fornjot/pull/216
[#217]: https://github.com/hannobraun/Fornjot/pull/217
[#218]: https://github.com/hannobraun/Fornjot/pull/218
[#221]: https://github.com/hannobraun/Fornjot/pull/221

[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#193]: https://github.com/hannobraun/Fornjot/issues/193

[#31]: https://github.com/hannobraun/www.fornjot.app/pull/31

[Daniel Egger]: https://github.com/therealprof
[Hendrik Maus]: https://github.com/hendrikmaus
