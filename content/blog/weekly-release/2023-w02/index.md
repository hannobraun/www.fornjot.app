+++
title = "Weekly Release - Back to Normal"
date = 2023-01-09

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.31.0"
subtitle = "Some nice improvements this week!"
+++

After my end-of-year vacation, we're back to normal this week, and things are progressing nicely! I've [declared victory](https://github.com/hannobraun/Fornjot/issues/1249#issuecomment-1373541839) on [#1249] and refocused my attention on [#1162]. This requires extensions to the builder API, and I've [already written a bit](https://github.com/hannobraun/Fornjot/issues/1162#issuecomment-1373565796) about how all of that fits together.

These hand-on programming activities are mixed in with lots of thinking going on in the background, about how the kernel needs to be structured in the future. I want to build a CAD system that makes changes to geometry easy to understand and reason about, and this will need to be supported in the kernel on an architectural level. A complete picture is still forming in my mind, and I'll let you know how that progresses!

In perhaps more interesting news, we've had some awesome contributions this week! [@zthompson47] has fixed an annoying issue, that (re-)loading the model would freeze the GUI, and [@antonok-edm] has implemented support for arcs in sketches, allowing for more interesting shapes to be created!

![Screenshot of a star-like 3D model with rounded edges](/blog/weekly-release/2023-w02/star-with-rounded-edges.png)


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix model updates freezing GUI; improve loading messages ([#1476]; thank you, [@zthompson47]!)
- Don't wrap `Angle` by default ([#1478]; thank you, [@antonok-edm]!)
- Support arcs in sketches ([#1484]; thank you, [@antonok-edm]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Expand and clean up builder API ([#1479], [#1483], [#1485], [#1489])
- Remove `fj_kernel::iter` ([#1480])
- Remove `Vertex::global_form` ([#1481])
- Improve some validation error messages and validation test output ([#1486])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1472], [#1473], [#1474], [#1475], [#1487])
- Expand and clean up release automation ([#1482])


[#1472]: https://github.com/hannobraun/Fornjot/pull/1472
[#1473]: https://github.com/hannobraun/Fornjot/pull/1473
[#1474]: https://github.com/hannobraun/Fornjot/pull/1474
[#1475]: https://github.com/hannobraun/Fornjot/pull/1475
[#1476]: https://github.com/hannobraun/Fornjot/pull/1476
[#1478]: https://github.com/hannobraun/Fornjot/pull/1478
[#1479]: https://github.com/hannobraun/Fornjot/pull/1479
[#1480]: https://github.com/hannobraun/Fornjot/pull/1480
[#1481]: https://github.com/hannobraun/Fornjot/pull/1481
[#1482]: https://github.com/hannobraun/Fornjot/pull/1482
[#1483]: https://github.com/hannobraun/Fornjot/pull/1483
[#1484]: https://github.com/hannobraun/Fornjot/pull/1484
[#1485]: https://github.com/hannobraun/Fornjot/pull/1485
[#1486]: https://github.com/hannobraun/Fornjot/pull/1486
[#1487]: https://github.com/hannobraun/Fornjot/pull/1487
[#1489]: https://github.com/hannobraun/Fornjot/pull/1489

[@zthompson47]: https://github.com/zthompson47
[@antonok-edm]: https://github.com/antonok-edm

[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
