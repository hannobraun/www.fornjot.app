+++
title = "Weekly Release - Still More To Do"
date = 2023-02-27

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.38.0"
subtitle = "The ongoing work is going well and still ongoing. I don't have a lot to say this week, do I."
+++

The simplification of the kernel code is going well. I managed to pick off two issues last week ([#1588], [#1605]). Still more to do though! The current state of the work is tracked in [#1589].

In other news, [@Jzow] made a small but welcome contribution, fixing a broken link in a README file.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix `cargo install fj-app` ([#1606])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Consolidate redundant references to `Surface` ([#1604])
- Simplify object graph by removing `Curve`/`GlobalCurve` ([#1607], [#1610], [#1614], [#1615], [#1616])
- Update documentation on edges ([#1609])
- Rename `SurfacePath` to `Curve` ([#1617])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1601], [#1602])
- Fix wrong link ([#1618]; thank you, [@Jzow]!)


[#1601]: https://github.com/hannobraun/Fornjot/pull/1601
[#1602]: https://github.com/hannobraun/Fornjot/pull/1602
[#1604]: https://github.com/hannobraun/Fornjot/pull/1604
[#1606]: https://github.com/hannobraun/Fornjot/pull/1606
[#1607]: https://github.com/hannobraun/Fornjot/pull/1607
[#1609]: https://github.com/hannobraun/Fornjot/pull/1609
[#1610]: https://github.com/hannobraun/Fornjot/pull/1610
[#1614]: https://github.com/hannobraun/Fornjot/pull/1614
[#1615]: https://github.com/hannobraun/Fornjot/pull/1615
[#1616]: https://github.com/hannobraun/Fornjot/pull/1616
[#1617]: https://github.com/hannobraun/Fornjot/pull/1617
[#1618]: https://github.com/hannobraun/Fornjot/pull/1618

[@Jzow]: https://github.com/Jzow

[#1588]: https://github.com/hannobraun/Fornjot/issues/1588
[#1589]: https://github.com/hannobraun/Fornjot/issues/1589
[#1605]: https://github.com/hannobraun/Fornjot/issues/1605