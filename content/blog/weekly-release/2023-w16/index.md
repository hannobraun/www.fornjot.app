+++
title = "Weekly Release - Back From Vacation"
date = 2023-04-18

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.43.0"
subtitle = "I'm back from vacation and don't have a whole lot to say."
+++

Hey folks, sorry for being a day late with this release! It took me an extra day to get myself sorted after coming back from vacation.

Speaking of vacation, this is going to be a short one! Not a lot happened while I was away, and the week before I left was also quite slow.

The main attraction is definitely support for exporting `.obj` files, added by [@replicadse]!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Add support for exporting `.obj` format ([#1739]; thank you, [@replicadse]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make minor cleanups in `Solid` validation code ([#1737])
- Expand operations API; replace `HalfEdgeBuilder` ([#1738])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1733], [#1753], [#1760], [#1762], [#1770])
- Upgrade to latest Rust version ([#1734], [#1752])
- Replace custom code with new features from `robust` ([#1761])
- Update list of sponsors in README ([#1771])


[#1733]: https://github.com/hannobraun/Fornjot/pull/1733
[#1734]: https://github.com/hannobraun/Fornjot/pull/1734
[#1737]: https://github.com/hannobraun/Fornjot/pull/1737
[#1738]: https://github.com/hannobraun/Fornjot/pull/1738
[#1739]: https://github.com/hannobraun/Fornjot/pull/1739
[#1752]: https://github.com/hannobraun/Fornjot/pull/1752
[#1753]: https://github.com/hannobraun/Fornjot/pull/1753
[#1760]: https://github.com/hannobraun/Fornjot/pull/1760
[#1761]: https://github.com/hannobraun/Fornjot/pull/1761
[#1762]: https://github.com/hannobraun/Fornjot/pull/1762
[#1770]: https://github.com/hannobraun/Fornjot/pull/1770
[#1771]: https://github.com/hannobraun/Fornjot/pull/1771

[@replicadse]: https://github.com/replicadse
