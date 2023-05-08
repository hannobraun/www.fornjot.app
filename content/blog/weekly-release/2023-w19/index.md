+++
title = "Weekly Release - The Last One"
date = 2023-05-08

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.46.0"
subtitle = "This is going to be the last weekly release for the time being! See inside for some more info."
+++

Last week saw some solid, but boring progress on the operations API ([#1713](https://github.com/hannobraun/Fornjot/issues/1713)). The new API is becoming ever more capable, and has now fully replaced the previous builder API. I also managed to make some design decisions that I had struggled with the week before.

This is going to be the last weekly release for the time being. Development has slowed down recently, and there's simply no need for a weekly release schedule anymore. For now, I'll play it by ear and just put out a new release whenever I think that's warranted.

I'll be publishing an article on the blog soon, about what has changed, and what's going on with the project in general.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week! Busy working on the kernel.*


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Derive `Eq`/`Ord` for `Object` ([#1810])
- Clean up validation service ([#1811])
- Replace remaining parts of obsolete builder API with operations API ([#1812], [#1813])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

*None this week.*

[#1810]: https://github.com/hannobraun/Fornjot/pull/1810
[#1811]: https://github.com/hannobraun/Fornjot/pull/1811
[#1812]: https://github.com/hannobraun/Fornjot/pull/1812
[#1813]: https://github.com/hannobraun/Fornjot/pull/1813
[#1814]: https://github.com/hannobraun/Fornjot/pull/1814
[#1815]: https://github.com/hannobraun/Fornjot/pull/1815
[#1816]: https://github.com/hannobraun/Fornjot/pull/1816

