+++
title = "Weekly Release - Inspiration"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-11-21

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.25.0"
subtitle = "Struck by inspiration; using that to promote a chat room."
+++

It's been a good week for Fornjot! I made progress with the cleanup of the object construction code ([#1249]), fixed bugs, and made some cosmetic improvements. See below for more details!

But honestly, the most significant change this weeks wasn't anything that happened with the Fornjot code. It was a change in my thinking, after watching [a talk](https://www.youtube.com/watch?v=8Ab3ArE8W3s) that I was must have been primed for, having thought about these topics for the last 10 years.

I'm not going to say any more than that  here, as I'm still in the middle of sorting out what all of it means for Fornjot. If you're interested in hearing more, why not [join the Fornjot Matrix channel](https://matrix.to/#/#fornjot:braun-odw.eu)? Not only have we been talking about the topic there, it's also a good place for day-to-day updates on Fornjot's development.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix potential soundness hole in version comparison ([#1358])
- Fix error when `cargo install`ing `fj-app` from `crates.io` ([#1364], [#1365])
- Soften shading ([#1366])
- Improve output of `--version` ([#1367])
- Fix triangulation of sharp, concave faces ([#1369])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Add infrastructure for abstracting over access to referenced objects ([#1359])
- Continue cleanup of partial object API ([#1360], [#1361], [#1362])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1345], [#1347], [#1348], [#1355])
- Update release procedure ([#1363])
- Make some smaller code cleanups ([#1368], [#1370], [#1371])


[#1345]: https://github.com/hannobraun/Fornjot/pull/1345
[#1347]: https://github.com/hannobraun/Fornjot/pull/1347
[#1348]: https://github.com/hannobraun/Fornjot/pull/1348
[#1355]: https://github.com/hannobraun/Fornjot/pull/1355
[#1358]: https://github.com/hannobraun/Fornjot/pull/1358
[#1359]: https://github.com/hannobraun/Fornjot/pull/1359
[#1360]: https://github.com/hannobraun/Fornjot/pull/1360
[#1361]: https://github.com/hannobraun/Fornjot/pull/1361
[#1362]: https://github.com/hannobraun/Fornjot/pull/1362
[#1363]: https://github.com/hannobraun/Fornjot/pull/1363
[#1364]: https://github.com/hannobraun/Fornjot/pull/1364
[#1365]: https://github.com/hannobraun/Fornjot/pull/1365
[#1366]: https://github.com/hannobraun/Fornjot/pull/1366
[#1367]: https://github.com/hannobraun/Fornjot/pull/1367
[#1368]: https://github.com/hannobraun/Fornjot/pull/1368
[#1369]: https://github.com/hannobraun/Fornjot/pull/1369
[#1370]: https://github.com/hannobraun/Fornjot/pull/1370
[#1371]: https://github.com/hannobraun/Fornjot/pull/1371

[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
