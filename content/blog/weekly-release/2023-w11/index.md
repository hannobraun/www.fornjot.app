+++
title = "Weekly Release - Thoroughly Mediocre"
date = 2023-03-13

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.40.0"
subtitle = "It wasn't a bad week. Not a great one either."
+++

When you've just had one of the more productive weeks of your life, it's hard to live up to that anyway. Add to that a cold and the resulting low energy, and you end up with something as thoroughly mediocre as last week.

It was fine. I made some progress on the big kernel cleanup ([#1589]), or more specifically the unification of full and partial objects ([#1570]). That went easier than expected so far, but now I'm hitting some open questions. I'll attempt to answer those this week!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week. Busy working on the kernel!*


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Unify `HalfEdge` and `PartialHalfEdge` ([#1660], [#1661], [#1662], [#1663], [#1664])
- Rewrite `HalfEdgeBuilder` ([#1665])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1651], [#1652], [#1653], [#1654], [#1655], [#1656], [#1657], [#1658], [#1659])
- Update list of sponsors in README ([#1667])


[#1651]: https://github.com/hannobraun/Fornjot/pull/1651
[#1652]: https://github.com/hannobraun/Fornjot/pull/1652
[#1653]: https://github.com/hannobraun/Fornjot/pull/1653
[#1654]: https://github.com/hannobraun/Fornjot/pull/1654
[#1655]: https://github.com/hannobraun/Fornjot/pull/1655
[#1656]: https://github.com/hannobraun/Fornjot/pull/1656
[#1657]: https://github.com/hannobraun/Fornjot/pull/1657
[#1658]: https://github.com/hannobraun/Fornjot/pull/1658
[#1659]: https://github.com/hannobraun/Fornjot/pull/1659
[#1660]: https://github.com/hannobraun/Fornjot/pull/1660
[#1661]: https://github.com/hannobraun/Fornjot/pull/1661
[#1662]: https://github.com/hannobraun/Fornjot/pull/1662
[#1663]: https://github.com/hannobraun/Fornjot/pull/1663
[#1664]: https://github.com/hannobraun/Fornjot/pull/1664
[#1665]: https://github.com/hannobraun/Fornjot/pull/1665
[#1667]: https://github.com/hannobraun/Fornjot/pull/1667

[#1570]: https://github.com/hannobraun/Fornjot/issues/1570
[#1589]: https://github.com/hannobraun/Fornjot/issues/1589
