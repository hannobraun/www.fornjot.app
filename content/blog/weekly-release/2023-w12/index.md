+++
title = "Weekly Release - Finished!"
date = 2023-03-20

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.41.0"
subtitle = "Finished with the cleanup work, ready for what comes next."
+++

The big kernel cleanup ([#1589]) is finished! The last big item was unification of partial and full objects ([#1570]), and I wrapped that up last week. There's still more cleanup to do, of course (I opened [#1691], for example), but that's always going to be the case.

The important thing is, we're in a much better place now, and ready for the next challenges! As I [wrote in #1589](https://github.com/hannobraun/Fornjot/issues/1589#issuecomment-1473648478), the kernel has reached a level of simplicity it hasn't had in many months, and back then it had fewer features and more bugs. A clear win!

Next, I'm going to restart my work on the union operation ([#42]), but with a different approach: The union algorithm is going to need both intersection tests and an API to build/modify geometry. Previously, I focused on the intersection tests, but now I'm going to focus on the builder side first. I already [wrote about my reasoning](https://github.com/hannobraun/Fornjot/issues/42#issuecomment-1473709945), in case you're interested.

Meanwhile, [@A-Walrus] has sent a whole avalanche of pull requests, updating the Nix flake, expanding validation, and improving test tooling.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week, busy working on the kernel!*


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Require `Handle<HalfEdge>` in fewer places ([#1680])
- Simplify some tests ([#1681])
- Unify remaining partial objects with their full variants ([#1682], [#1683], [#1684], [#1685])
- Add `Cycle` validation ([#1686]; thank you, [@A-Walrus]!)
- Update and fix Nix flake ([#1687], [#1690]; thank you, [@A-Walrus]!)
- Clean up builder API ([#1692], [#1693])
- Update docs for `Solid` ([#1696]; thank you, [@A-Walrus]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1669], [#1670], [#1671], [#1672], [#1673], [#1674], [#1675], [#1676], [#1677], [#1678])
- Upgrade to Rust 1.68.0 ([#1679])
- Allow export-validator to receive model name ([#1698]; thank you, [@A-Walrus]!)
- Update list of sponsors in README ([#1699])


[#1669]: https://github.com/hannobraun/Fornjot/pull/1669
[#1670]: https://github.com/hannobraun/Fornjot/pull/1670
[#1671]: https://github.com/hannobraun/Fornjot/pull/1671
[#1672]: https://github.com/hannobraun/Fornjot/pull/1672
[#1673]: https://github.com/hannobraun/Fornjot/pull/1673
[#1674]: https://github.com/hannobraun/Fornjot/pull/1674
[#1675]: https://github.com/hannobraun/Fornjot/pull/1675
[#1676]: https://github.com/hannobraun/Fornjot/pull/1676
[#1677]: https://github.com/hannobraun/Fornjot/pull/1677
[#1678]: https://github.com/hannobraun/Fornjot/pull/1678
[#1679]: https://github.com/hannobraun/Fornjot/pull/1679
[#1680]: https://github.com/hannobraun/Fornjot/pull/1680
[#1681]: https://github.com/hannobraun/Fornjot/pull/1681
[#1682]: https://github.com/hannobraun/Fornjot/pull/1682
[#1683]: https://github.com/hannobraun/Fornjot/pull/1683
[#1684]: https://github.com/hannobraun/Fornjot/pull/1684
[#1685]: https://github.com/hannobraun/Fornjot/pull/1685
[#1686]: https://github.com/hannobraun/Fornjot/pull/1686
[#1687]: https://github.com/hannobraun/Fornjot/pull/1687
[#1690]: https://github.com/hannobraun/Fornjot/pull/1690
[#1692]: https://github.com/hannobraun/Fornjot/pull/1692
[#1693]: https://github.com/hannobraun/Fornjot/pull/1693
[#1696]: https://github.com/hannobraun/Fornjot/pull/1696
[#1698]: https://github.com/hannobraun/Fornjot/pull/1698
[#1699]: https://github.com/hannobraun/Fornjot/pull/1699

[@A-Walrus]: https://github.com/A-Walrus

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1570]: https://github.com/hannobraun/Fornjot/issues/1570
[#1589]: https://github.com/hannobraun/Fornjot/issues/1589
[#1691]: https://github.com/hannobraun/Fornjot/issues/1691
