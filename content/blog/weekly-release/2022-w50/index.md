+++
title = "Weekly Release - Breakthrough"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-12-12

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.28.0"
subtitle = "Looks like that new idea from last week delivered!"
+++

Last week, I wrote that I might be in the middle of a breakthrough, in regards to cleaning up the object construction code ([#1249]). Work is still ongoing, but so far there's every indication that my optimism was warranted, and I have indeed found the solution to the biggest problem that I've been battling with for the last months!

I still need to wrap this up, but I expect that won't take much longer now. I've [posted an update in the issue](https://github.com/hannobraun/Fornjot/issues/1249#issuecomment-1346373387) that summarizes the current state and what's next.

Meanwhile, [@danieleades] has improved the codebase by fixing some non-default Clippy lints, and [@zthompson47] addressed a potentially confusing error condition, by making sure there's a proper error message, if the `RUST_LOG` environment variable can't be parsed.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fail, if RUST_LOG is invalid ([#1435]; thank you, [@zthompson47]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve behavior around validation failures ([#1417], [#1418], [#1436])
- Continue cleanup of object construction code ([#1419], [#1423], [#1428], [#1429], [#1430], [#1432], [#1433])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1412], [#1414], [#1416])
- Fix some Clippy lints ([#1421]; thank you, [@danieleades]!)
- Improve usage and documentation of `justfile` ([#1422], [#1425])


[#1412]: https://github.com/hannobraun/Fornjot/pull/1412
[#1414]: https://github.com/hannobraun/Fornjot/pull/1414
[#1416]: https://github.com/hannobraun/Fornjot/pull/1416
[#1417]: https://github.com/hannobraun/Fornjot/pull/1417
[#1418]: https://github.com/hannobraun/Fornjot/pull/1418
[#1419]: https://github.com/hannobraun/Fornjot/pull/1419
[#1421]: https://github.com/hannobraun/Fornjot/pull/1421
[#1422]: https://github.com/hannobraun/Fornjot/pull/1422
[#1423]: https://github.com/hannobraun/Fornjot/pull/1423
[#1425]: https://github.com/hannobraun/Fornjot/pull/1425
[#1428]: https://github.com/hannobraun/Fornjot/pull/1428
[#1429]: https://github.com/hannobraun/Fornjot/pull/1429
[#1430]: https://github.com/hannobraun/Fornjot/pull/1430
[#1432]: https://github.com/hannobraun/Fornjot/pull/1432
[#1433]: https://github.com/hannobraun/Fornjot/pull/1433
[#1435]: https://github.com/hannobraun/Fornjot/pull/1435
[#1436]: https://github.com/hannobraun/Fornjot/pull/1436

[@danieleades]: https://github.com/danieleades
[@zthompson47]: https://github.com/zthompson47

[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
