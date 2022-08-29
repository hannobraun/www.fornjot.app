+++
title = "Weekly Release - 2022-W35"
date = 2022-08-29

[extra]
version = "0.13.0"
+++

Last week was eventful! Initially I made some progress on the intersection tests (see [#42], specifically the [list I keep updated](https://github.com/hannobraun/Fornjot/issues/42#issuecomment-1206449099)). Then, I hit a hurdle ([#993]). That issue has more details, but the gist of it is, that the kernel data structures are not robust enough to handle certain tasks required for the next batch of intersection tests.

Solving this required some cleanups, and while planning those out, inspiration struck and I recognized an opportunity for a big simplification of the kernel code, which in turn required some more cleanups... and down the rabbit hole I went. Check out the pull requests below for the progress I made so far.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Update usage documentation in README ([#994])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-host`

- Improve comments and documentation ([#988])

#### `fj-kernel`

- Expand intersection tests ([#977], [#978])
- Extract `Shell` from `Solid` ([#983])
- Clean up sweep API ([#984], [#989], [#991])
- Add builder API for `Sketch` ([#992])
- Add `GlobalEdge` ([#998], [#999])
- Make some minor cleanups ([#1000], [#1001], [#1005])
- Clean up `approx` module ([#1003], [#1006])

#### `fj-math`

- Make minor API additions ([#1004])

#### `fj-viewer`/`fj-window`

- Upgrade dependencies related to wgpu/winit ([#975], [#979])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#964], [#965], [#966], [#968], [#969], [#970], [#974])
- Update release procedure ([#972])
- Upgrade to Rust 1.63.0 ([#973])
- Expand release automation ([#981], [#982])
- Add usage documentation to `CONTRIBUTING.md` ([#995])


### Issue of the Week

Fornjot is still very much an early-stage project, and I'm pretty sure that most of its usage still happens as part of developing it. Being run mostly from within its own repository has allowed some problems to creep in, that become really obvious once you start using it in the real world.

One such problem is [#980 - Version mismatch between `fj` and `fj-app` can cause problems](https://github.com/hannobraun/Fornjot/issues/980). If you like fiddling with Rust and aren't deterred by the presence of an FFI boundary, this might be the issue for you!


### Outlook

As I explained above, I fell into a rabbit hole of kernel cleanups. My priority this week will be to figure out, which of those cleanups are going to be required to solve the problem at hand ([#993]); which of them aren't, but are still worth the investment (because they will soon start saving more work than implementing them costs me now); and which of them are currently a distraction.

Implementing the union operation ([#42]) remains my top priority and I don't want to get stuck in a cleanup cycle that won't be conducive to that.


[#961]: https://github.com/hannobraun/Fornjot/pull/961
[#962]: https://github.com/hannobraun/Fornjot/pull/962
[#964]: https://github.com/hannobraun/Fornjot/pull/964
[#965]: https://github.com/hannobraun/Fornjot/pull/965
[#966]: https://github.com/hannobraun/Fornjot/pull/966
[#968]: https://github.com/hannobraun/Fornjot/pull/968
[#969]: https://github.com/hannobraun/Fornjot/pull/969
[#970]: https://github.com/hannobraun/Fornjot/pull/970
[#972]: https://github.com/hannobraun/Fornjot/pull/972
[#973]: https://github.com/hannobraun/Fornjot/pull/973
[#974]: https://github.com/hannobraun/Fornjot/pull/974
[#975]: https://github.com/hannobraun/Fornjot/pull/975
[#977]: https://github.com/hannobraun/Fornjot/pull/977
[#978]: https://github.com/hannobraun/Fornjot/pull/978
[#979]: https://github.com/hannobraun/Fornjot/pull/979
[#981]: https://github.com/hannobraun/Fornjot/pull/981
[#982]: https://github.com/hannobraun/Fornjot/pull/982
[#983]: https://github.com/hannobraun/Fornjot/pull/983
[#984]: https://github.com/hannobraun/Fornjot/pull/984
[#988]: https://github.com/hannobraun/Fornjot/pull/988
[#989]: https://github.com/hannobraun/Fornjot/pull/989
[#991]: https://github.com/hannobraun/Fornjot/pull/991
[#992]: https://github.com/hannobraun/Fornjot/pull/992
[#994]: https://github.com/hannobraun/Fornjot/pull/994
[#995]: https://github.com/hannobraun/Fornjot/pull/995
[#998]: https://github.com/hannobraun/Fornjot/pull/998
[#999]: https://github.com/hannobraun/Fornjot/pull/999
[#1000]: https://github.com/hannobraun/Fornjot/pull/1000
[#1001]: https://github.com/hannobraun/Fornjot/pull/1001
[#1003]: https://github.com/hannobraun/Fornjot/pull/1003
[#1004]: https://github.com/hannobraun/Fornjot/pull/1004
[#1005]: https://github.com/hannobraun/Fornjot/pull/1005
[#1006]: https://github.com/hannobraun/Fornjot/pull/1006

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#993]: https://github.com/hannobraun/Fornjot/issues/993
