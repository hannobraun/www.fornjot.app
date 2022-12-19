+++
title = "Weekly Release - Consolidation"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-12-19

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.29.0"
subtitle = "Consolidating gains in a quiet and focused week."
+++

Last week was boring and quiet, both of which were good things. That breakthrough I keep talking about, in solving the problems with the object construction code ([#1249]), that definitely turned out to be real, and I have no doubt that the new approach is the solution I've been looking for. I've been working on applying the new approach in more places, consolidating the gains made.

I'm slowly starting to shift my focus to [#1162]. Addressing this issue requires new validation code, which requires new unit tests, which need some non-trivial object construction done. This is a good proving ground for the new object construction approach. At the same time, it's blocking further progress on the union operation ([#42]), which I'd like to get back to as soon as possible.

Of course, nothing is ever perfect, and I've already started [thinking about the next round of improvements](https://github.com/hannobraun/Fornjot/discussions/1454). But I'll reign myself in. I might start looking into this on the side though, if I'm in the mood for some extra work.

Meanwhile, [@kazatsuyu] has fixed an annoying bug that happened on Windows. I'm only using Linux myself, so help in tracking down those platform-specific bugs is especially welcome!

Please note that **this is going to be the last release of the year**, as I'm on vacation next week. Regular weekly releases will resume in 2023.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix crash when minimizing window on Windows ([#1447]; thank you, [@kazatsuyu]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Continue cleanup of object construction code ([#1445], [#1446], [#1448], [#1449], [#1450], [#1451], [#1452], [#1453], [#1456], [#1457])
- Fix doc comment ([#1458])

#### `fj-math`

- Return line coordinates from `Line::from_points` ([#1455])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1437], [#1438], [#1440], [#1443], [#1444])


[#1437]: https://github.com/hannobraun/Fornjot/pull/1437
[#1438]: https://github.com/hannobraun/Fornjot/pull/1438
[#1440]: https://github.com/hannobraun/Fornjot/pull/1440
[#1443]: https://github.com/hannobraun/Fornjot/pull/1443
[#1444]: https://github.com/hannobraun/Fornjot/pull/1444
[#1445]: https://github.com/hannobraun/Fornjot/pull/1445
[#1446]: https://github.com/hannobraun/Fornjot/pull/1446
[#1447]: https://github.com/hannobraun/Fornjot/pull/1447
[#1448]: https://github.com/hannobraun/Fornjot/pull/1448
[#1449]: https://github.com/hannobraun/Fornjot/pull/1449
[#1450]: https://github.com/hannobraun/Fornjot/pull/1450
[#1451]: https://github.com/hannobraun/Fornjot/pull/1451
[#1452]: https://github.com/hannobraun/Fornjot/pull/1452
[#1453]: https://github.com/hannobraun/Fornjot/pull/1453
[#1455]: https://github.com/hannobraun/Fornjot/pull/1455
[#1456]: https://github.com/hannobraun/Fornjot/pull/1456
[#1457]: https://github.com/hannobraun/Fornjot/pull/1457
[#1458]: https://github.com/hannobraun/Fornjot/pull/1458

[@kazatsuyu]: https://github.com/kazatsuyu

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
