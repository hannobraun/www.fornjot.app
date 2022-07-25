+++
title = "Weekly Release - 2022-W30"
path = "/blog/weekly-release/2022-w30/"

[extra]
version = "0.9.0"
+++

Last week wasn't one of those super-focused work weeks that I like most. I spent Monday dealing with the new weekly release procedure; Tuesday I was out sick; Wednesday and much of Thursday, I accidentally distracted myself into doing some cleanup work that was useful, but not immediately important.

Thursday and Friday I finally managed to get back on track with boolean operations. Or rather, the latest detour from boolean operations. I mentioned last week that I could really use a low-level shape updating API for use in unit test, and I've been looking into that.

No definite results so far, but I've come up with some changes to the kernel APIs that would be required to make it happen. And since I very much liked those changes on their own terms, I've just ended up making them, as I look into the topic of the update API more deeply.

Meanwhile, I've had some awesome help from contributors again this week! [@hendrikmaus] has fixed some issues with the release automation; [@Michael-F-Bryan] has started improving the API we expose to models, as well as aspects of the host/model interface; and [@willhansen] has contributed a test case for an open bug, which should make that bug much easier to fix.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Determine model's `target/` directory from Cargo metadata ([#828], [#841], [#853]; special thanks go to first-time contributor [@Michael-F-Bryan]!)
- Derive `PartialEq` for types in `fj` crate ([#832]; thank you, [@Michael-F-Bryan]!)
- Type-check model functions ([#867]; thank you, [@Michael-F-Bryan]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Convert `Color` into a struct ([#862])

#### `fj-kernel`

- Clean up and expand APIs of `Edge`, `Face`, and `Cycle` ([#854], [#855], [#863], [#865])
- Return references to objects, where appropriate ([#858])
- Make names of `Local` methods more explicit ([#860])
- Revamp builder API ([#864], [#866])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Fix release automation issues ([#814], [#843]; thank you, [@hendrikmaus]!)
- Update dependencies ([#836], [#840])
- Update release procedure ([#838], [#839], [#857])
- Add unit test for triangulation bug ([#842]; special thanks go to first-time contributor [@willhansen]!)
- Upgrade to  Rust 1.62.1 ([#852])
- Clean up `fj-kernel`'s `iter` module ([#859])
- Expand implementation note ([#861])


### Issue of the Week

Fornjot is still in its infancy, and an area where that really shows is usability. One especially annoying issue, is that errors or status messages are not shown anywhere in the UI, being relegated to the command-line instead.

If you're interested in GUI, especially [`egui`](https://crates.io/crates/egui), then [#856 - Add UI element that displays status updates](https://github.com/hannobraun/Fornjot/issues/856) might be an interesting issue for you.


### Outlook

I'm still looking into the potential low-level shape update API, as that would be very useful for my further work on boolean operations, and pretty much any other kernel work down the line.

I'm currently experimenting with what the syntax could be, and how it would work. I expect that very soon, I'll come to a decision between either going on with the implementation, deferring it, or abandoning the effort.


[#814]: https://github.com/hannobraun/Fornjot/pull/814
[#828]: https://github.com/hannobraun/Fornjot/pull/828
[#832]: https://github.com/hannobraun/Fornjot/pull/832
[#836]: https://github.com/hannobraun/Fornjot/pull/836
[#838]: https://github.com/hannobraun/Fornjot/pull/838
[#839]: https://github.com/hannobraun/Fornjot/pull/839
[#840]: https://github.com/hannobraun/Fornjot/pull/840
[#841]: https://github.com/hannobraun/Fornjot/pull/841
[#842]: https://github.com/hannobraun/Fornjot/pull/842
[#843]: https://github.com/hannobraun/Fornjot/pull/843
[#852]: https://github.com/hannobraun/Fornjot/pull/852
[#853]: https://github.com/hannobraun/Fornjot/pull/853
[#854]: https://github.com/hannobraun/Fornjot/pull/854
[#855]: https://github.com/hannobraun/Fornjot/pull/855
[#857]: https://github.com/hannobraun/Fornjot/pull/857
[#858]: https://github.com/hannobraun/Fornjot/pull/858
[#859]: https://github.com/hannobraun/Fornjot/pull/859
[#860]: https://github.com/hannobraun/Fornjot/pull/860
[#861]: https://github.com/hannobraun/Fornjot/pull/861
[#862]: https://github.com/hannobraun/Fornjot/pull/862
[#863]: https://github.com/hannobraun/Fornjot/pull/863
[#864]: https://github.com/hannobraun/Fornjot/pull/864
[#865]: https://github.com/hannobraun/Fornjot/pull/865
[#866]: https://github.com/hannobraun/Fornjot/pull/866
[#867]: https://github.com/hannobraun/Fornjot/pull/867

[@hendrikmaus]: https://github.com/hendrikmaus
[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan
[@willhansen]: https://github.com/willhansen
