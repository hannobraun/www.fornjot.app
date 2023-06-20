+++
title = "Fornjot 0.47.0"
date = 2023-06-20

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.47.0"
subtitle = "It's a kernel now."
+++

It's the first release since the end of the weekly release schedule!

The big-ticket item this time is the execution of the [recent change in focus](/blog/a-new-direction/). This involved moving now deprecated components out of the repository, replace some of them with simpler versions, and improve anything else as required. The biggest chunk of work here was restoring the examples (which were based on the app and other now-removed components).

As a result, the API of `fj-core` (formerly `fj-kernel`) has become a lot more convenient and powerful. All the new examples in the repository are using it directly now, instead of through some intermediate high-level API. Improving the `fj-core` API further is an ongoing process.

Other than that, I've been settling into my new schedule, working on Fornjot in a reduced capacity. I've made sure to stay consistent, still putting a bit of work in every week, and that's working well so far.

Not sure what's next, specifically (figuring that out is actually the next item on my task list), but I'm sure it will mainly involve improving the `fj-core` API, to make existing models more convenient to write, and enable more powerful models in the future.


### Sponsors

Fornjot is supported by [@MitchellHansen](https://github.com/MitchellHansen), [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### Library improvements

Improvements to Fornjot libraries.

#### `fj`

- Add new `fj` crate as all-in-one interface to the whole kernel ([#1853])
- Add standardized CLI for models ([#1860])
- Compute tolerance manually, if not provided via CLI ([#1872])

#### `fj-core`

- Add `Region` to share code between `Sketch` and `Face` ([#1828], [#1883]; thank you, [@A-Walrus]!)
- Make various cleanups ([#1830], [#1831])
- Expand and update operations API ([#1850], [#1879], [#1884], [#1891])
- Rename `fj-kernel` to `fj-core` ([#1852])
- Compute AABB from boundary representation ([#1871], [#1886], [#1888])
- Clean up `Reverse` ([#1885])

#### `fj-interop`

- Clean up model-related code in `fj-interop` ([#1863], [#1864])

#### `fj-math`

- Expand `Aabb` API ([#1870])
- Add `Vector::from_component` and `Circle::aabb` ([#1887])

#### `fj-viewer`

- Remove GUI code ([#1829])
- Remove vestigial debug rendering code ([#1862])
- Make more cleanups ([#1893], [#1898])

#### `fj-window`

- Re-add `fj-window` crate ([#1837])
- Work around crash when opening window ([#1849])
- Rename `window::run` to `display` ([#1861])
- Expect `Model` in `fj_window::display` ([#1865])


### Other changes

Improvements that are not associated with a specific Fornjot library.

- Update dependencies ([#1821], [#1827], [#1835], [#1845], [#1847], [#1858], [#1880], [#1897])
- Execute the change in focus, towards building only a CAD kernel ([#1822], [#1823], [#1824], [#1836], [#1838], [#1846], [#1851], [#1889], [#1890], [#1892])
- Upgrade to Rust 1.70.0 ([#1859])
- Update README ([#1869], [#1881])
- Update documentation ([#1882], [#1894], [#1899])
- Determine crates to publish automatically ([#1900])


[#1821]: https://github.com/hannobraun/fornjot/pull/1821
[#1822]: https://github.com/hannobraun/fornjot/pull/1822
[#1823]: https://github.com/hannobraun/fornjot/pull/1823
[#1824]: https://github.com/hannobraun/fornjot/pull/1824
[#1827]: https://github.com/hannobraun/fornjot/pull/1827
[#1828]: https://github.com/hannobraun/fornjot/pull/1828
[#1829]: https://github.com/hannobraun/fornjot/pull/1829
[#1830]: https://github.com/hannobraun/fornjot/pull/1830
[#1831]: https://github.com/hannobraun/fornjot/pull/1831
[#1835]: https://github.com/hannobraun/fornjot/pull/1835
[#1836]: https://github.com/hannobraun/fornjot/pull/1836
[#1837]: https://github.com/hannobraun/fornjot/pull/1837
[#1838]: https://github.com/hannobraun/fornjot/pull/1838
[#1845]: https://github.com/hannobraun/fornjot/pull/1845
[#1846]: https://github.com/hannobraun/fornjot/pull/1846
[#1847]: https://github.com/hannobraun/fornjot/pull/1847
[#1849]: https://github.com/hannobraun/fornjot/pull/1849
[#1850]: https://github.com/hannobraun/fornjot/pull/1850
[#1851]: https://github.com/hannobraun/fornjot/pull/1851
[#1852]: https://github.com/hannobraun/fornjot/pull/1852
[#1853]: https://github.com/hannobraun/fornjot/pull/1853
[#1858]: https://github.com/hannobraun/fornjot/pull/1858
[#1859]: https://github.com/hannobraun/fornjot/pull/1859
[#1860]: https://github.com/hannobraun/fornjot/pull/1860
[#1861]: https://github.com/hannobraun/fornjot/pull/1861
[#1862]: https://github.com/hannobraun/fornjot/pull/1862
[#1863]: https://github.com/hannobraun/fornjot/pull/1863
[#1864]: https://github.com/hannobraun/fornjot/pull/1864
[#1865]: https://github.com/hannobraun/fornjot/pull/1865
[#1869]: https://github.com/hannobraun/fornjot/pull/1869
[#1870]: https://github.com/hannobraun/fornjot/pull/1870
[#1871]: https://github.com/hannobraun/fornjot/pull/1871
[#1872]: https://github.com/hannobraun/fornjot/pull/1872
[#1879]: https://github.com/hannobraun/fornjot/pull/1879
[#1880]: https://github.com/hannobraun/fornjot/pull/1880
[#1881]: https://github.com/hannobraun/fornjot/pull/1881
[#1882]: https://github.com/hannobraun/fornjot/pull/1882
[#1883]: https://github.com/hannobraun/fornjot/pull/1883
[#1884]: https://github.com/hannobraun/fornjot/pull/1884
[#1885]: https://github.com/hannobraun/fornjot/pull/1885
[#1886]: https://github.com/hannobraun/fornjot/pull/1886
[#1887]: https://github.com/hannobraun/fornjot/pull/1887
[#1888]: https://github.com/hannobraun/fornjot/pull/1888
[#1889]: https://github.com/hannobraun/fornjot/pull/1889
[#1890]: https://github.com/hannobraun/fornjot/pull/1890
[#1891]: https://github.com/hannobraun/fornjot/pull/1891
[#1892]: https://github.com/hannobraun/fornjot/pull/1892
[#1893]: https://github.com/hannobraun/fornjot/pull/1893
[#1894]: https://github.com/hannobraun/fornjot/pull/1894
[#1897]: https://github.com/hannobraun/fornjot/pull/1897
[#1898]: https://github.com/hannobraun/fornjot/pull/1898
[#1899]: https://github.com/hannobraun/fornjot/pull/1899
[#1900]: https://github.com/hannobraun/fornjot/pull/1900

[@A-Walrus]: https://github.com/A-Walrus
