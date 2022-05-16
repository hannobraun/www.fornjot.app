+++
title = "Weekly Dev Log - 2022-W19"
date  = 2022-05-16
+++

I continued working on the union operation last week ([#42]). I implemented some building blocks for the algorithm, then hit complications (of course 😁), which prompted me to divert my attention to another cleanup effort ([#568]).

The short version is, up until recently, the Fornjot kernel dealt mostly with 3D coordinates. This kept things simple, but there are cases where it makes more sense to work in other coordinates systems (i.e. 2D surface coordinates and 1D curve coordinates). The simplistic "3D-mostly" approach causes problems in various areas (see [#250], for an example), and now boolean operations have been added to that list.

It's unfortunate that there's another detour, but it is what it is. As always, I'd rather do things right, than build on top of an insufficient foundation.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix mistakes in model READMEs ([#555]; special thanks to first-time contributor [@chrisprice]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Replace curve types with new types in `fj_math` ([#558], [#578])
- Expand and clean up intersection code ([#560], [#562])
- Improve APIs for conversion between coordinate systems ([#561], [#564])
- Start using local coordinates where that makes sense ([#574], [#575], [#579], [#582])
- Make minor improvements to various APIs ([#580], [#581], [#583])

#### `fj-math`

- Add `Vector::scalar_projection_onto` ([#553])
- Add `Line` and `Circle` ([#557], [#563], [#577])
- Improve `Aabb` API ([#559])

#### `fj-viewer`

- Fix typo in API documentation ([#573]; thanks to [@freylint]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#548], [#549], [#550], [#551], [#552])
- Simplify `ToShape` implementation of `fj::Sketch` ([#554])
- Make some minor cleanups ([#565], [#576])
- Add local build script ([#569]; thanks to [@chrisprice])
- Add Gnome Builder configuration to `.gitignore` ([#570]; thanks to [@freylint]!)


### Issue of the Week

A while ago, Fornjot gained the ability to sweep a sketch along an arbitrary vector, not just long the z-axis, as before. The bounding volume calculation for the sweep operation wasn't updated correctly, when that change was made.

If you're interested in getting a taste of how CAD operations are implemented in Fornjot, why not take a look at [#566 - Bounding volume of sweep operation is incorrect](https://github.com/hannobraun/Fornjot/issues/566)?


### Outlook

I'm going to continue working on [#568]. I've made good progress already, but there are some aspects of the solution I haven't figured out yet. We'll see how it goes.

After taking care of that, I plan to go back to working on the union operation ([#42]).


[#548]: https://github.com/hannobraun/Fornjot/pull/548
[#549]: https://github.com/hannobraun/Fornjot/pull/549
[#550]: https://github.com/hannobraun/Fornjot/pull/550
[#551]: https://github.com/hannobraun/Fornjot/pull/551
[#552]: https://github.com/hannobraun/Fornjot/pull/552
[#553]: https://github.com/hannobraun/Fornjot/pull/553
[#554]: https://github.com/hannobraun/Fornjot/pull/554
[#555]: https://github.com/hannobraun/Fornjot/pull/555
[#557]: https://github.com/hannobraun/Fornjot/pull/557
[#558]: https://github.com/hannobraun/Fornjot/pull/558
[#559]: https://github.com/hannobraun/Fornjot/pull/559
[#560]: https://github.com/hannobraun/Fornjot/pull/560
[#561]: https://github.com/hannobraun/Fornjot/pull/561
[#562]: https://github.com/hannobraun/Fornjot/pull/562
[#563]: https://github.com/hannobraun/Fornjot/pull/563
[#564]: https://github.com/hannobraun/Fornjot/pull/564
[#565]: https://github.com/hannobraun/Fornjot/pull/565
[#569]: https://github.com/hannobraun/Fornjot/pull/569
[#570]: https://github.com/hannobraun/Fornjot/pull/570
[#573]: https://github.com/hannobraun/Fornjot/pull/573
[#574]: https://github.com/hannobraun/Fornjot/pull/574
[#575]: https://github.com/hannobraun/Fornjot/pull/575
[#576]: https://github.com/hannobraun/Fornjot/pull/576
[#577]: https://github.com/hannobraun/Fornjot/pull/577
[#578]: https://github.com/hannobraun/Fornjot/pull/578
[#579]: https://github.com/hannobraun/Fornjot/pull/579
[#580]: https://github.com/hannobraun/Fornjot/pull/580
[#581]: https://github.com/hannobraun/Fornjot/pull/581
[#582]: https://github.com/hannobraun/Fornjot/pull/582
[#583]: https://github.com/hannobraun/Fornjot/pull/583

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#250]: https://github.com/hannobraun/Fornjot/issues/250
[#568]: https://github.com/hannobraun/Fornjot/issues/568

[@chrisprice]: https://github.com/chrisprice
[@freylint]: https://github.com/freylint
