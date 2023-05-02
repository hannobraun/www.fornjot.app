+++
title = "Weekly Release - Inching Along"
date = 2023-05-02

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.45.0"
subtitle = "Nothing exciting this week, just a bit of progress."
+++

What's this, another Tuesday release? Yeah, I'm sorry. This time, a public holiday was to blame. I happened to be out hiking for most of Monday, so no time to publish a release, really 😁

Not much else to say this week. [#1713](https://github.com/hannobraun/Fornjot/issues/1713) is inching along. No great problems, nor any great breakthroughs. You, you know, work.

Meanwhile, [@A-Walrus] found and fixed an issue in the documentation, which is always very welcome!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix sweep docs ([#1800]; thank you, [@A-Walrus]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Clean up and expand operations API ([#1794], [#1797])
- Clean up in-kernel services API ([#1795])
- Add missing information to error messages ([#1796])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Fix warning; make sure warnings can't slip through CI again ([#1793])
- Update list of sponsors in README ([#1798])
- Update dependencies ([#1808])


[#1793]: https://github.com/hannobraun/Fornjot/pull/1793
[#1794]: https://github.com/hannobraun/Fornjot/pull/1794
[#1795]: https://github.com/hannobraun/Fornjot/pull/1795
[#1796]: https://github.com/hannobraun/Fornjot/pull/1796
[#1797]: https://github.com/hannobraun/Fornjot/pull/1797
[#1798]: https://github.com/hannobraun/Fornjot/pull/1798
[#1800]: https://github.com/hannobraun/Fornjot/pull/1800
[#1808]: https://github.com/hannobraun/Fornjot/pull/1808

[@A-Walrus]: https://github.com/A-Walrus
