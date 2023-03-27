+++
title = "Weekly Release - Some Good Progress"
date = 2023-03-27

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.42.0"
subtitle = "The new API for creating and updating geometry is coming along."
+++

That new API for create/updating geometry is coming along. I chose to tackle tests for the new `Solid`/`Shell` validation checks as a first use case, and I've made some good progress there. I [wrote an update](https://github.com/hannobraun/Fornjot/issues/1713#issuecomment-1482809021) on what's left to do here.

Once that use case is addressed, a possible next step is to rewrite the sweep algorithm on top of the new API. This would require further expanding and solidifying the new API, but I'm not sure yet if that's the next logical step. We'll see!

Meanwhile, [@A-Walrus] submitted another fix for the Nix flake and added more validation checks, while [@IamTheCarl] removed and unnecessary dependency and improved the error message for panics in the model code.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Improve error message, if model code panics ([#1716], [#1721]; thank you, [@IamTheCarl]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-host`

- Remove dependency on winit ([#1712]; thank you, [@IamTheCarl]!)

#### `fj-kernel`

- Add validation checks for `Shell` and `Solid` ([#1695]; thank you, [@A-Walrus]!)
- Fix winding algorithm not going back to start ([#1709])
- Create new API for creating/updating geometry ([#1711], [#1714], [#1717], [#1718], [#1719])
- Clean up objects service code ([#1715])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1701], [#1702], [#1703], [#1705], [#1707], [#1723])
- Update release procedure ([#1708])
- Fix rust-analyzer in Nix flake ([#1710]; thank you, [@A-Walrus]!)


[#1695]: https://github.com/hannobraun/Fornjot/pull/1695
[#1701]: https://github.com/hannobraun/Fornjot/pull/1701
[#1702]: https://github.com/hannobraun/Fornjot/pull/1702
[#1703]: https://github.com/hannobraun/Fornjot/pull/1703
[#1705]: https://github.com/hannobraun/Fornjot/pull/1705
[#1707]: https://github.com/hannobraun/Fornjot/pull/1707
[#1708]: https://github.com/hannobraun/Fornjot/pull/1708
[#1709]: https://github.com/hannobraun/Fornjot/pull/1709
[#1710]: https://github.com/hannobraun/Fornjot/pull/1710
[#1711]: https://github.com/hannobraun/Fornjot/pull/1711
[#1712]: https://github.com/hannobraun/Fornjot/pull/1712
[#1714]: https://github.com/hannobraun/Fornjot/pull/1714
[#1715]: https://github.com/hannobraun/Fornjot/pull/1715
[#1716]: https://github.com/hannobraun/Fornjot/pull/1716
[#1717]: https://github.com/hannobraun/Fornjot/pull/1717
[#1718]: https://github.com/hannobraun/Fornjot/pull/1718
[#1719]: https://github.com/hannobraun/Fornjot/pull/1719
[#1721]: https://github.com/hannobraun/Fornjot/pull/1721
[#1723]: https://github.com/hannobraun/Fornjot/pull/1723

[@A-Walrus]: https://github.com/A-Walrus
[@IamTheCarl]: https://github.com/IamTheCarl
