+++
title = "Weekly Release - Progressed Extremely Well"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-03-06

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.39.0"
subtitle = "Finally, a week without any hard problems! Let's not get used to it."
+++

Last week has been awesome! No spending days or weeks tracking down some complicated bug. No fumbling around, trying to come up with the solution to a design limitation. Just cleaning up code, making it much better than it was before. And lots of that!

Work on the ongoing kernel cleanup has progressed extremely well ([#1589]). I've removed redundant data from the object graph, instead computing it on-demand and caching it where that's necessary to guarantee correctness ([#1586], [#1634]). And I've simplified the object graph a lot, consolidating all the redundant references that I'm aware of ([#1525], [#1643]).

We shouldn't get used to weeks like this, of course. Sooner or later a really hard problem will crop up again. And even though bashing my head against that won't *feel* as productive, it will be just as necessary to make progress.

The last big item that's left in the cleanup (as of now; I might be thinking of more ideas!) is the unification of full and partial objects ([#1570]). It's not fully clear to me how that's going to work yet, but I'm optimistic that I can come up with something.

While I was busy dealing with technical debt that an earlier, less seasoned version of me introduced in the first place, [@Jzow] has updated the installation instructions for macOS, which will hopefully help interested users in the future!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Update installation instructions for macOS ([#1631]; thank you, [@Jzow]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make some tweaks in service code ([#1629])
- Simplify object graph around `HalfEdge` ([#1630], [#1632], [#1638], [#1642], [#1644])
- Remove unused builder methods ([#1633])
- Remove redundant data from object graph, compute it on-demand instead ([#1635], [#1636], [#1640], [#1641], [#1647])
- Make validation unit tests more explicit ([#1637])
- Remove unused validation error ([#1639])
- Start unifying full and partial objects ([#1645], [#1646], [#1648])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1622], [#1623], [#1624], [#1625], [#1628])
- Update list of sponsors in README ([#1649])


[#1622]: https://github.com/hannobraun/Fornjot/pull/1622
[#1623]: https://github.com/hannobraun/Fornjot/pull/1623
[#1624]: https://github.com/hannobraun/Fornjot/pull/1624
[#1625]: https://github.com/hannobraun/Fornjot/pull/1625
[#1628]: https://github.com/hannobraun/Fornjot/pull/1628
[#1629]: https://github.com/hannobraun/Fornjot/pull/1629
[#1630]: https://github.com/hannobraun/Fornjot/pull/1630
[#1631]: https://github.com/hannobraun/Fornjot/pull/1631
[#1632]: https://github.com/hannobraun/Fornjot/pull/1632
[#1633]: https://github.com/hannobraun/Fornjot/pull/1633
[#1635]: https://github.com/hannobraun/Fornjot/pull/1635
[#1636]: https://github.com/hannobraun/Fornjot/pull/1636
[#1637]: https://github.com/hannobraun/Fornjot/pull/1637
[#1638]: https://github.com/hannobraun/Fornjot/pull/1638
[#1639]: https://github.com/hannobraun/Fornjot/pull/1639
[#1640]: https://github.com/hannobraun/Fornjot/pull/1640
[#1641]: https://github.com/hannobraun/Fornjot/pull/1641
[#1642]: https://github.com/hannobraun/Fornjot/pull/1642
[#1644]: https://github.com/hannobraun/Fornjot/pull/1644
[#1645]: https://github.com/hannobraun/Fornjot/pull/1645
[#1646]: https://github.com/hannobraun/Fornjot/pull/1646
[#1647]: https://github.com/hannobraun/Fornjot/pull/1647
[#1648]: https://github.com/hannobraun/Fornjot/pull/1648
[#1649]: https://github.com/hannobraun/Fornjot/pull/1649

[@Jzow]: https://github.com/Jzow

[#1525]: https://github.com/hannobraun/Fornjot/issues/1525
[#1570]: https://github.com/hannobraun/Fornjot/issues/1570
[#1586]: https://github.com/hannobraun/Fornjot/issues/1586
[#1589]: https://github.com/hannobraun/Fornjot/issues/1589
[#1634]: https://github.com/hannobraun/Fornjot/issues/1634
[#1643]: https://github.com/hannobraun/Fornjot/issues/1643