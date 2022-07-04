+++
title = "Weekly Dev Log - 2022-W26"
date  = 2022-07-04
+++

I've recently spent a lot of time cleaning up code, thinking about better ways to clean up code, and struggling to clean up code that is really committed to stay dirty. All that effort paid off last week, and what a payoff it was!

The overly complicated `Shape` API: removed ([#697])! Handling of local forms: simplified ([#691])! Approximations of faces in surface coordinates: implemented ([#568])!

There's always more to do, of course, but all the improvements I deemed impactful enough to work on immediately are finished now. Nothing else is blocking further progress on boolean operations ([#42], [#43], [#44]).

Meanwhile, [@eLVas] has fixed an annoying issue with `export-validator` (a tool that exports and validates the test models from the Fornjot repository), and [@jeevcat] has encountered and fixed a UI-related performance issue.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Update list of sponsors ([#727])
- Update README ([#729])
- Fix performance issue related to mouse movements ([#758]; special thanks to first-time contributor [@jeevcat]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Rewrite sweep algorithm ([#723], [#732], [#742])
- Remove `Shape` API ([#730], [#735], [#736], [#737], [#738], [#747], [#748])
- Rewrite transform algorithm ([#733], [#743])
- Add missing re-export ([#731])
- Add face reversal algorithm ([#744])
- Fix vertex uniqueness validation ([#746])
- Simplify handling of local forms ([#750], [#751], [#752], [#753], [#755], [#756], [#759], [#761])
- Fix face equality ([#754])
- Approximate faces in surface coordinates ([#762])

#### `fj-operations`

- Reduce reliance on `Shape` ([#734])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#724], [#725], [#726], [#728])
- Include `export-validator` in `just build` ([#739], [#740])
- Use temporary directory for export validation ([#749]; special thanks to first-time contributor [@eLVas]!)
- Fix typo in comment ([#760])


### Issue of the Week

Making releases is a pain for a lot of projects, and so it is for Fornjot. I'm thankful that we have some automation in place to ease the burden, but of course, there's always room for improvement!

If you're into this kind of build infrastructure and automation work, why not take a look at [#479 - `release-operator` should automatically determine which crates to publish](https://github.com/hannobraun/Fornjot/issues/479)?


### Outlook

With all that cleanup work out of the way, I'm ready to resume work on implementing the union operation ([#42]). However, I want to publish a new release first. Now seems as good a time as any, and the last one is already two months old. Waiting longer will only make it more difficult.


[#723]: https://github.com/hannobraun/Fornjot/pull/723
[#724]: https://github.com/hannobraun/Fornjot/pull/724
[#725]: https://github.com/hannobraun/Fornjot/pull/725
[#726]: https://github.com/hannobraun/Fornjot/pull/726
[#727]: https://github.com/hannobraun/Fornjot/pull/727
[#728]: https://github.com/hannobraun/Fornjot/pull/728
[#729]: https://github.com/hannobraun/Fornjot/pull/729
[#730]: https://github.com/hannobraun/Fornjot/pull/730
[#731]: https://github.com/hannobraun/Fornjot/pull/731
[#732]: https://github.com/hannobraun/Fornjot/pull/732
[#733]: https://github.com/hannobraun/Fornjot/pull/733
[#734]: https://github.com/hannobraun/Fornjot/pull/734
[#735]: https://github.com/hannobraun/Fornjot/pull/735
[#736]: https://github.com/hannobraun/Fornjot/pull/736
[#737]: https://github.com/hannobraun/Fornjot/pull/737
[#738]: https://github.com/hannobraun/Fornjot/pull/738
[#739]: https://github.com/hannobraun/Fornjot/pull/739
[#740]: https://github.com/hannobraun/Fornjot/pull/740
[#742]: https://github.com/hannobraun/Fornjot/pull/742
[#743]: https://github.com/hannobraun/Fornjot/pull/743
[#744]: https://github.com/hannobraun/Fornjot/pull/744
[#746]: https://github.com/hannobraun/Fornjot/pull/746
[#747]: https://github.com/hannobraun/Fornjot/pull/747
[#748]: https://github.com/hannobraun/Fornjot/pull/748
[#749]: https://github.com/hannobraun/Fornjot/pull/749
[#750]: https://github.com/hannobraun/Fornjot/pull/750
[#751]: https://github.com/hannobraun/Fornjot/pull/751
[#752]: https://github.com/hannobraun/Fornjot/pull/752
[#753]: https://github.com/hannobraun/Fornjot/pull/753
[#754]: https://github.com/hannobraun/Fornjot/pull/754
[#755]: https://github.com/hannobraun/Fornjot/pull/755
[#756]: https://github.com/hannobraun/Fornjot/pull/756
[#758]: https://github.com/hannobraun/Fornjot/pull/758
[#759]: https://github.com/hannobraun/Fornjot/pull/759
[#760]: https://github.com/hannobraun/Fornjot/pull/760
[#761]: https://github.com/hannobraun/Fornjot/pull/761
[#762]: https://github.com/hannobraun/Fornjot/pull/762

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#568]: https://github.com/hannobraun/Fornjot/issues/568
[#691]: https://github.com/hannobraun/Fornjot/issues/691
[#697]: https://github.com/hannobraun/Fornjot/issues/697

[@eLVas]: https://github.com/eLVas
[@jeevcat]: https://github.com/jeevcat
