+++
title = "Weekly Dev Log - 2022-W06"
date  = 2022-02-14
+++

It's been another productive week of Fornjot development! On a high level, my priority right now is to implement [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG) operations. This requires a lot of refactoring and clean-up first, and I keep finding myself in this recursive mode of having to first tackle another clean-up, before I can wrap up the current one. A requires B, requires C, required D, ...usually I'm finding myself 5 layers down that chain before I hit on something that I can actually implement right now.

This is slow-going work, but it's also very satisfying. Every little clean-up I merge, makes the CAD kernel a bit more robust, and better able to handle future requirements. I just need to keep taking these steps, until all the hurdles are out of the way, and I can implement the CSG algorithms themselves.

In other news, new contributor [Hendrik Maus] did some great work improving the build infrastructure!


### Sponsors

Fornjot is supported by [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Improvements and fixes

- Add missing space to error message ([#144])
- Re-enable logging ([#148])
- Fix handling of duplicate points in approximation ([#158])


### Documentation

- Update README ([#162])


### Internal improvements

- Update dependencies ([#135], [#136], [#137], [#142])
- Make some minor clean-ups ([#147], [#160], [#165], [#169], [#172], [#177])
- Add approximation test suite ([#149], [#150], [#159], [#167])
- Add infrastructure to make vertices less error-prone ([#161], [#166])
- Clean up `Circle` code ([#163], [#164])
- Add bounding vertices to `Edge` ([#168])
- Make edge approximation more robust ([#170])
- Change `Line` to point-vector form ([#174])
- Provide pre-compiled binaries for each build of `main` branch ([#175], [#179]; special thanks to first-time contributor [Hendrik Maus])


### Website

- Add Weekly Dev Log ([#20], [#28])
- Simplify footer ([#21])
- Refer to newsletter in blog posts ([#22])
- Make some tweaks to the design ([#23], [#24])
- Improve `<aside>`s on top of blog posts ([#25])
- Cross-post article from my website ([#26])
- Generate Atom feed ([#27])
- Improve title ([#29])
- Add Sponsor page ([#30])


### Outlook

I managed to finish a lot of clean-ups last week, the big-ticket item being [#138]. The goal remains to address [#97], but I'm aware of a few more clean-ups I need to make before I can tackle this directly. As mentioned above, the last operation that needs to be updated for [#97] is the sweep operation, and to do that, I need to add support sweeping curves into surfaces, which is up next. After that, I'm foreseeing the need to make the triangulation code a bit more flexible.

No idea how far I'll get until the next Weekly Dev Log rolls around.


[#135]: https://github.com/hannobraun/Fornjot/pull/135
[#136]: https://github.com/hannobraun/Fornjot/pull/136
[#137]: https://github.com/hannobraun/Fornjot/pull/137
[#142]: https://github.com/hannobraun/Fornjot/pull/142
[#144]: https://github.com/hannobraun/Fornjot/pull/144
[#147]: https://github.com/hannobraun/Fornjot/pull/147
[#148]: https://github.com/hannobraun/Fornjot/pull/148
[#149]: https://github.com/hannobraun/Fornjot/pull/149
[#150]: https://github.com/hannobraun/Fornjot/pull/150
[#158]: https://github.com/hannobraun/Fornjot/pull/158
[#159]: https://github.com/hannobraun/Fornjot/pull/159
[#160]: https://github.com/hannobraun/Fornjot/pull/160
[#161]: https://github.com/hannobraun/Fornjot/pull/161
[#162]: https://github.com/hannobraun/Fornjot/pull/162
[#163]: https://github.com/hannobraun/Fornjot/pull/163
[#164]: https://github.com/hannobraun/Fornjot/pull/164
[#165]: https://github.com/hannobraun/Fornjot/pull/165
[#166]: https://github.com/hannobraun/Fornjot/pull/166
[#167]: https://github.com/hannobraun/Fornjot/pull/167
[#168]: https://github.com/hannobraun/Fornjot/pull/168
[#169]: https://github.com/hannobraun/Fornjot/pull/169
[#170]: https://github.com/hannobraun/Fornjot/pull/170
[#172]: https://github.com/hannobraun/Fornjot/pull/172
[#174]: https://github.com/hannobraun/Fornjot/pull/174
[#175]: https://github.com/hannobraun/Fornjot/pull/175
[#177]: https://github.com/hannobraun/Fornjot/pull/177
[#179]: https://github.com/hannobraun/Fornjot/pull/179

[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#138]: https://github.com/hannobraun/Fornjot/issues/138

[#20]: https://github.com/hannobraun/www.fornjot.app/pull/20
[#21]: https://github.com/hannobraun/www.fornjot.app/pull/21
[#22]: https://github.com/hannobraun/www.fornjot.app/pull/22
[#23]: https://github.com/hannobraun/www.fornjot.app/pull/23
[#24]: https://github.com/hannobraun/www.fornjot.app/pull/24
[#25]: https://github.com/hannobraun/www.fornjot.app/pull/25
[#26]: https://github.com/hannobraun/www.fornjot.app/pull/26
[#27]: https://github.com/hannobraun/www.fornjot.app/pull/27
[#28]: https://github.com/hannobraun/www.fornjot.app/pull/28
[#29]: https://github.com/hannobraun/www.fornjot.app/pull/29
[#30]: https://github.com/hannobraun/www.fornjot.app/pull/30

[Hendrik Maus]: https://github.com/hendrikmaus
