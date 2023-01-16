+++
title = "Weekly Release - Slow Progress"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-01-16

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.32.0"
subtitle = "Work on the builder API is progressing."
+++

Work on fixing the known object duplication issues continues ([#1162]). I'm still focused an expanding the builder API ([short update here](https://github.com/hannobraun/Fornjot/issues/1162#issuecomment-1382126600)), which provides some foundational building blocks that will be useful beyond addressing this issue. Expanding the builder API mostly involves design work, which is hard, and therefore quite slow. I'm making progress though.

I've also been experimenting with some ideas I have on improving the kernel architecture. This hasn't gotten anywhere yet, so nothing to report at this time.

Meanwhile, [@zthompson47] has continued helping out by addressing some Clippy warnings that were previously silenced.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix app crashing, if it is minimized too long ([#1504])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve builder API ([#1495], [#1501], [#1502], [#1509], [#1510])
- Don't stop on first validation error ([#1505])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1490], [#1491], [#1492], [#1494])
- Fix rust-analyzer configuration in VS Code ([#1497])
- Upgrade to Rust 1.66.1 ([#1500])
- Box large error variants ([#1506]; thank you, [@zthompson47]!)


[#1490]: https://github.com/hannobraun/Fornjot/pull/1490
[#1491]: https://github.com/hannobraun/Fornjot/pull/1491
[#1492]: https://github.com/hannobraun/Fornjot/pull/1492
[#1494]: https://github.com/hannobraun/Fornjot/pull/1494
[#1495]: https://github.com/hannobraun/Fornjot/pull/1495
[#1497]: https://github.com/hannobraun/Fornjot/pull/1497
[#1500]: https://github.com/hannobraun/Fornjot/pull/1500
[#1501]: https://github.com/hannobraun/Fornjot/pull/1501
[#1502]: https://github.com/hannobraun/Fornjot/pull/1502
[#1504]: https://github.com/hannobraun/Fornjot/pull/1504
[#1505]: https://github.com/hannobraun/Fornjot/pull/1505
[#1506]: https://github.com/hannobraun/Fornjot/pull/1506
[#1509]: https://github.com/hannobraun/Fornjot/pull/1509
[#1510]: https://github.com/hannobraun/Fornjot/pull/1510

[@zthompson47]: https://github.com/zthompson47

[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
