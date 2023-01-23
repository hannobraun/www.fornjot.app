+++
title = "Weekly Release - Keep Things Moving"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-01-23

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.33.0"
subtitle = "Taking a break from working on the critical path to keep the project moving."
+++

Over the last week, I have started to shift my focus away from the work that directly contributes to advancing [the roadmap](https://www.fornjot.app/roadmap/), towards more foundational work. Right now, that mostly means simplifying the core data structures ([#1525]), which should pretty much make every aspect of kernel development easier.

Honestly, I think I just need a break from staring at the builder code ([#1162]), like I've been doing for the last few weeks. In addition, improving the foundational infrastructure will benefit all future development. I don't know when I'll get back to [#1162], but I think it's most important for me to stay productive and move things in the right direction in general, rather than advance along one specific path, even when that path could be considered the most important one.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

None this week, busy working on the kernel!


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve validation error message ([#1518])
- Add `FaceBuilder::infer_curves` ([#1520])
- Simplify object graph around `HalfEdge` ([#1521], [#1522], [#1524], [#1526], [#1527])

#### `fj-math`

- Fix `Plane::project_vector` ([#1523])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1512], [#1517])


[#1512]: https://github.com/hannobraun/Fornjot/pull/1512
[#1517]: https://github.com/hannobraun/Fornjot/pull/1517
[#1518]: https://github.com/hannobraun/Fornjot/pull/1518
[#1520]: https://github.com/hannobraun/Fornjot/pull/1520
[#1521]: https://github.com/hannobraun/Fornjot/pull/1521
[#1522]: https://github.com/hannobraun/Fornjot/pull/1522
[#1523]: https://github.com/hannobraun/Fornjot/pull/1523
[#1524]: https://github.com/hannobraun/Fornjot/pull/1524
[#1526]: https://github.com/hannobraun/Fornjot/pull/1526
[#1527]: https://github.com/hannobraun/Fornjot/pull/1527

[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1525]: https://github.com/hannobraun/Fornjot/issues/1525
