+++
title = "Weekly Dev Log - 2022-W21"
date  = 2022-05-30
+++

I worked on a few different things last week, but my main achievement was the implementation of geometric validation, at least as much of it as is needed and useful right now ([#613]). This allowed for some more progress towards approximating faces in surface coordinates ([#568]), which is currently the main issue that blocks work on implementing the union operation ([#42]).

Meanwhile, [@chrisprice] got busy fixing the input/camera code, which had been sitting in a semi-broken state for a while now. [@gabsi26] started contributing to Fornjot, with the addition of an `Angle` type, and some very welcome bug fixes.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix field of view calculation ([#614]; thanks to [@chrisprice]!)
- Add `Angle` type ([#619], [#621]; special thanks to first-time contributor [@gabsi26]!)
- Fix bounding volume calculation for swept shapes ([#623]; thanks to [@gabsi26]!)
- Fix face orientation for sweeps in negative direction ([#628], [#630]; thanks to [@gabsi26]!)
- Improve error message for shape processing errors ([#635])
- Improve `Angle` ([#641]; special thanks to first-time contributor [@T0mstone]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

- Extract `fj-window` from `fj-viewer` ([#640])

#### `fj-kernel`

- Implement geometric validation ([#618], [#620], [#626], [#636], [#637])
- Simplify local representation of edge vertices ([#625])
- Replace problematic `Edge` constructor ([#627])
- Add missing re-export ([#631])

#### `fj-operations`

- Improve error handling ([#629], [#632])

#### `fj-viewer`

- Improve error handling ([#633])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#615], [#616])
- Upgrade to Rust 1.61.0 ([#617])
- Some small cleanups ([#622])
- Fail CI build, if any warnings are present ([#624])
- Improve handling of panics ([#634])
- Add test model that uses most features ([#638], [#639])


### Issue of the Week

Fornjot isn't using a lot of `unsafe` code, but we need some of it in the `fj` crate, to support loading models into the Fornjot application at runtime. Maybe a bit of `unsafe` code sounds like a fun and slightly dangerous challenge to you? If so, why not take a look at [#186 - `fj::Sketch` is leaking memory](https://github.com/hannobraun/Fornjot/issues/186)?


### Outlook

With all new distractions out of the way now, I can continue working on [#568], with the goal of implementing the union operation ([#42]). I keep making slow and steady progress, and I hope that will continue this week.


[#614]: https://github.com/hannobraun/Fornjot/pull/614
[#615]: https://github.com/hannobraun/Fornjot/pull/615
[#616]: https://github.com/hannobraun/Fornjot/pull/616
[#617]: https://github.com/hannobraun/Fornjot/pull/617
[#618]: https://github.com/hannobraun/Fornjot/pull/618
[#619]: https://github.com/hannobraun/Fornjot/pull/619
[#620]: https://github.com/hannobraun/Fornjot/pull/620
[#621]: https://github.com/hannobraun/Fornjot/pull/621
[#622]: https://github.com/hannobraun/Fornjot/pull/622
[#623]: https://github.com/hannobraun/Fornjot/pull/623
[#624]: https://github.com/hannobraun/Fornjot/pull/624
[#625]: https://github.com/hannobraun/Fornjot/pull/625
[#626]: https://github.com/hannobraun/Fornjot/pull/626
[#627]: https://github.com/hannobraun/Fornjot/pull/627
[#628]: https://github.com/hannobraun/Fornjot/pull/628
[#629]: https://github.com/hannobraun/Fornjot/pull/629
[#630]: https://github.com/hannobraun/Fornjot/pull/630
[#631]: https://github.com/hannobraun/Fornjot/pull/631
[#632]: https://github.com/hannobraun/Fornjot/pull/632
[#633]: https://github.com/hannobraun/Fornjot/pull/633
[#634]: https://github.com/hannobraun/Fornjot/pull/634
[#635]: https://github.com/hannobraun/Fornjot/pull/635
[#636]: https://github.com/hannobraun/Fornjot/pull/636
[#637]: https://github.com/hannobraun/Fornjot/pull/637
[#638]: https://github.com/hannobraun/Fornjot/pull/638
[#639]: https://github.com/hannobraun/Fornjot/pull/639
[#640]: https://github.com/hannobraun/Fornjot/pull/640
[#641]: https://github.com/hannobraun/Fornjot/pull/641

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#568]: https://github.com/hannobraun/Fornjot/issues/568
[#613]: https://github.com/hannobraun/Fornjot/issues/613

[@chrisprice]: https://github.com/chrisprice
[@gabsi26]: https://github.com/gabsi26
[@T0mstone]: https://github.com/T0mstone
