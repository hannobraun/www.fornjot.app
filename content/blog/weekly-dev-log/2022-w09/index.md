+++
title = "Weekly Dev Log - 2022-W09"
date  = 2022-03-07
+++

Last week, I had started working on vertex validation ([#242]) and was hopeful to  make quick progress. Turns out, that work was a bit *too* successful. The original intent was to use vertex validation to provide some coverage for upcoming work on the sweep code, as previous attempts had resulted in subtle bugs.

Before I could get there though, it uncovered some bugs in the existing sweep code! It's good to know that working on vertex validation was worthwhile, but it also means more unexpected work. On top of that, fixing those bugs turned out to be difficult with the current shape representation, so I decided to fix that problem at the source, by improving the shape representation ([#280]).

All in all, it's business as usual: more problems, more fixes, more opportunities to make the CAD kernel more robust!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### User-visible improvements

- Update sponsors in README ([#258])
- Remove `fj::Difference` ([#265])
- Fix sweeping of un-symmetrical sketches ([#284])
- Fix incorrectly shaded faces ([#289])


### Internal improvements

- Run Clippy and formatting check in CI build ([#208], [#266]; special thanks to first-time contributor [@ObiWanRohan]!)
- Update dependencies ([#259], [#260], [#264])
- Prepare for vertex validation ([#267], [#268], [#273], [#274], [#276], [#277])
- Fix warnings in documentation ([#270])
- Document kernel design principle ([#272])
- Consolidate all transformation code in a single module ([#275])
- Move sweep code to `kernel::algorithms::sweep` ([#279])
- Improve test suite ([#281], [#282], [#285])
- Work on improving shape representation ([#283], [#286], [#287], [#288], [#290])


### Issue of the week

When you submit a pull request to the Fornjot repository, it runs through a continuous integration (CI) build, to make sure it doesn't have any obvious issues. The CI build checks for a few things, like Clippy warnings and code formatting, that might not be obvious to new users.

To make it a bit easier to determine, if your pull request is ready to be merged, it would be great to have a local build script to determine whether your changes will likely pass CI.

If you're looking for a way to get involved with Fornjot, why not give [#269 - Add local build script](https://github.com/hannobraun/Fornjot/issues/269) a try?


### Outlook

The work on improving the shape representation ([#280]) has been progressing nicely. Once that is done, I can fix the bugs uncovered by vertex validation and enable that permanently ([#242]). After that, I can resume work on [#97], which has been the main driver behind all this recent work on kernel robustness.


[#208]: https://github.com/hannobraun/Fornjot/pull/208
[#258]: https://github.com/hannobraun/Fornjot/pull/258
[#259]: https://github.com/hannobraun/Fornjot/pull/259
[#260]: https://github.com/hannobraun/Fornjot/pull/260
[#264]: https://github.com/hannobraun/Fornjot/pull/264
[#265]: https://github.com/hannobraun/Fornjot/pull/265
[#266]: https://github.com/hannobraun/Fornjot/pull/266
[#267]: https://github.com/hannobraun/Fornjot/pull/267
[#268]: https://github.com/hannobraun/Fornjot/pull/268
[#270]: https://github.com/hannobraun/Fornjot/pull/270
[#272]: https://github.com/hannobraun/Fornjot/pull/272
[#273]: https://github.com/hannobraun/Fornjot/pull/273
[#274]: https://github.com/hannobraun/Fornjot/pull/274
[#275]: https://github.com/hannobraun/Fornjot/pull/275
[#276]: https://github.com/hannobraun/Fornjot/pull/276
[#277]: https://github.com/hannobraun/Fornjot/pull/277
[#279]: https://github.com/hannobraun/Fornjot/pull/279
[#281]: https://github.com/hannobraun/Fornjot/pull/281
[#282]: https://github.com/hannobraun/Fornjot/pull/282
[#283]: https://github.com/hannobraun/Fornjot/pull/283
[#284]: https://github.com/hannobraun/Fornjot/pull/284
[#285]: https://github.com/hannobraun/Fornjot/pull/285
[#286]: https://github.com/hannobraun/Fornjot/pull/286
[#287]: https://github.com/hannobraun/Fornjot/pull/287
[#288]: https://github.com/hannobraun/Fornjot/pull/288
[#289]: https://github.com/hannobraun/Fornjot/pull/289
[#290]: https://github.com/hannobraun/Fornjot/pull/290

[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#242]: https://github.com/hannobraun/Fornjot/issues/242
[#280]: https://github.com/hannobraun/Fornjot/issues/280

[@ObiWanRohan]: https://github.com/ObiWanRohan
