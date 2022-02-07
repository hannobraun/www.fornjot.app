+++
title = "Weekly Dev Log - 2022-W05"
date  = 2022-02-07
+++

### Sponsors

Fornjot is supported by [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Improvements and fixes

- Prevent potential floating-point accuracy issues in triangulation ([#133])


### Documentation

- Update README ([#113], [#127])


### Internal improvements

- Use caching to speed up CI build ([#114]) (thanks to [@mxdamien])
- Update dependencies ([#115], [#119])
- Clean up CAD kernel ([#120], [#121], [#125], [#129], [#130], [#132])
- Update release procedure ([#124])
- Update internal developer documentation ([#126], [#128])
- Replace delaunator-rs with Spade ([#131])


### Website

- Update list of sponsors ([#14])
- Add weekly dev log ([#15])
- Minor clean-ups ([#16])
- Add first issue of Last Month in Fornjot ([#17])
- Add newsletter template for Last Month in Fornjot ([#18])
- Update about page ([#19])


### Outlook

I managed to wrap up [#78] last week, which means I'm now ready to tackle [#97]. This is a refactoring that should enable the implementation of constructive solid geometry ([#42], [#43], [#44]). I've finished most of this a few weeks back, but the last bit will require some additions to, and associated clean-ups of, the CAD kernel.

I don't think this will be too much of a problem, but we'll see.


[#113]: https://github.com/hannobraun/Fornjot/pull/113
[#114]: https://github.com/hannobraun/Fornjot/pull/114
[#115]: https://github.com/hannobraun/Fornjot/pull/115
[#119]: https://github.com/hannobraun/Fornjot/pull/119
[#120]: https://github.com/hannobraun/Fornjot/pull/120
[#121]: https://github.com/hannobraun/Fornjot/pull/121
[#124]: https://github.com/hannobraun/Fornjot/pull/124
[#125]: https://github.com/hannobraun/Fornjot/pull/125
[#126]: https://github.com/hannobraun/Fornjot/pull/126
[#127]: https://github.com/hannobraun/Fornjot/pull/127
[#128]: https://github.com/hannobraun/Fornjot/pull/128
[#129]: https://github.com/hannobraun/Fornjot/pull/129
[#130]: https://github.com/hannobraun/Fornjot/pull/130
[#131]: https://github.com/hannobraun/Fornjot/pull/131
[#132]: https://github.com/hannobraun/Fornjot/pull/132

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#78]: https://github.com/hannobraun/Fornjot/issues/78
[#97]: https://github.com/hannobraun/Fornjot/issues/97

[#14]: https://github.com/hannobraun/www.fornjot.app/pull/14
[#15]: https://github.com/hannobraun/www.fornjot.app/pull/15
[#16]: https://github.com/hannobraun/www.fornjot.app/pull/16
[#17]: https://github.com/hannobraun/www.fornjot.app/pull/17
[#18]: https://github.com/hannobraun/www.fornjot.app/pull/18
[#19]: https://github.com/hannobraun/www.fornjot.app/pull/19

[@mxdamien]: https://github.com/mxdamien
