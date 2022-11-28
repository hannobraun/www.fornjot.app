+++
title = "Weekly Release - Experimentation"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-11-28

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.26.0"
subtitle = "Experimenting with an approach that could accelerate development."
+++

A week ago, I reported how I got inspired by a talk I've seen, and my time since then was dominated by experimentation, with some promising first results!

My goal is to make the Fornjot kernel interactive. Right now, you more or less launch it with some input, then get back the results. I want to be able to inspect it while it is running, to understand exactly what is going on at every point. The immediate benefit should be that debugging the kernel becomes easier. If it works out like that, this could help accelerate development in the future.

I've been prototyping an architecture that enables this interactivity. The results are promising, in that it's possible now to inspect exactly what a core part of the kernel does over time, without that complicating any other parts of the code. However, there is no tooling that makes use of this new capability yet, so it's hard to judge how useful it is going to be in practice.

I want to wrap up this initial experimentation quickly, then continue this work on the side. While I believe that this is going to be very beneficial for Fornjot's further development, there is so much more important work to do, and I can't afford to have one thing take over my time completely.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Remove model generation feature to fix `cargo install` error ([#1373])
- Enable model version check on Windows ([#1374])
- Change messages to say "evaluating" instead of "compiling" ([#1396])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Build service abstraction around `Objects` ([#1377], [#1384], [#1390], [#1392], [#1393])
- Fix `Store` iteration bug ([#1383])
- Simplify old builder structs ([#1388])
- Add `Object` enum ([#1391])

#### `fj-operations`

- Take `&mut Objects` in `Shape::compute_brep` ([#1389])
- Simplify return value of `Shape::compute_brep` ([#1394])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1378], [#1379], [#1380], [#1381], [#1382], [#1386])
- Make some small cleanups ([#1395])


[#1373]: https://github.com/hannobraun/Fornjot/pull/1373
[#1374]: https://github.com/hannobraun/Fornjot/pull/1374
[#1377]: https://github.com/hannobraun/Fornjot/pull/1377
[#1378]: https://github.com/hannobraun/Fornjot/pull/1378
[#1379]: https://github.com/hannobraun/Fornjot/pull/1379
[#1380]: https://github.com/hannobraun/Fornjot/pull/1380
[#1381]: https://github.com/hannobraun/Fornjot/pull/1381
[#1382]: https://github.com/hannobraun/Fornjot/pull/1382
[#1383]: https://github.com/hannobraun/Fornjot/pull/1383
[#1384]: https://github.com/hannobraun/Fornjot/pull/1384
[#1386]: https://github.com/hannobraun/Fornjot/pull/1386
[#1388]: https://github.com/hannobraun/Fornjot/pull/1388
[#1389]: https://github.com/hannobraun/Fornjot/pull/1389
[#1390]: https://github.com/hannobraun/Fornjot/pull/1390
[#1391]: https://github.com/hannobraun/Fornjot/pull/1391
[#1392]: https://github.com/hannobraun/Fornjot/pull/1392
[#1393]: https://github.com/hannobraun/Fornjot/pull/1393
[#1394]: https://github.com/hannobraun/Fornjot/pull/1394
[#1395]: https://github.com/hannobraun/Fornjot/pull/1395
[#1396]: https://github.com/hannobraun/Fornjot/pull/1396

