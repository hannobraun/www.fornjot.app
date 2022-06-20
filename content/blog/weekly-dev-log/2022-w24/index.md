+++
title = "Weekly Dev Log - 2022-W24"
date  = 2022-06-20
+++

Last week, I said that I expected to either wrap up [#568], or discover new problems that prevent that. Well, I didn't discover new problems exactly, but I did discover new opportunities! As it turns out, that was just as effective in preventing me from making progress on [#568].

It started early in the week, when some thoughts I'd been having over the last weeks coalesced into the insight that local forms can be managed much more simply ([#691]). Simplifying things always pays dividends, as it makes whatever else I'm working on easier. Hence, I decided to start implementing that immediately.

While doing that, I ran into a problem pretty quickly: the current, somewhat hacky, way of handling surface orientation got in the way, so I started developing a plan for dealing with that ([#695]). And while I was already in deep thinking mode, inspiration struck again, and I came up with a way for simplifying the core data structures of `fj-kernel` ([#696], [#697]).

So, definitely more of a "thinking" week than a "doing" week. It doesn't feel quite as productive, but you need both kinds to make progress long-term.

Meanwhile, [@kamirr] fixed some build system issues, and [@A-Walrus] improved the usability of the Fornjot app's CLI arguments.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Rename `fj` crate's `serialization` feature to `serde` ([#688])
- Improve usability of `--parameters` ([#692]; special thanks to first-time contributor [@A-Walrus]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Add `Curve` constructors for surface axes ([#690])
- Remove `Edge::new` ([#693])
- Move all objects to new `objects` module ([#694])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Fix some build system issues ([#685]; thanks to [@kamirr]!)
- Update dependencies ([#686], [#687])
- Clean up coherence validation infrastructure ([#689])


### Issue of the Week

Fornjot's input/camera control behavior has improved a lot recently, but there are still some annoyances. If you enjoy fiddling with behavior at the interface between human and computer, why not take a look at [#677 - Improve zooming behavior](https://github.com/hannobraun/Fornjot/issues/677)?


### Outlook

My immediate priority has switched to implementing all the simplifications I came up with last week. Yes, this will delay the completion of [#568], but I think it will be worth it. As I said above, simplifications pay dividends. If I don't do them now, I'll just make everything more difficult going forward.


[#685]: https://github.com/hannobraun/Fornjot/pull/685
[#686]: https://github.com/hannobraun/Fornjot/pull/686
[#687]: https://github.com/hannobraun/Fornjot/pull/687
[#688]: https://github.com/hannobraun/Fornjot/pull/688
[#689]: https://github.com/hannobraun/Fornjot/pull/689
[#690]: https://github.com/hannobraun/Fornjot/pull/690
[#692]: https://github.com/hannobraun/Fornjot/pull/692
[#693]: https://github.com/hannobraun/Fornjot/pull/693
[#694]: https://github.com/hannobraun/Fornjot/pull/694

[#568]: https://github.com/hannobraun/Fornjot/issues/568
[#691]: https://github.com/hannobraun/Fornjot/issues/691
[#695]: https://github.com/hannobraun/Fornjot/issues/695
[#696]: https://github.com/hannobraun/Fornjot/issues/696
[#697]: https://github.com/hannobraun/Fornjot/issues/697

[@A-Walrus]: https://github.com/A-Walrus
[@kamirr]: https://github.com/kamirr
