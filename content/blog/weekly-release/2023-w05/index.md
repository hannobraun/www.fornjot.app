+++
title = "Weekly Release - The Usual Rabbit Hole"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-01-30

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.34.0"
subtitle = "1. Have work to do; 2. Identify obstacle; 3. Add obstacle to list of work, go back to 1."
+++

The simplification of the core data structures, currently focused on `HalfEdge` and its neighbors in the object graph, continues ([#1525]). I fell down the usual rabbit hole of finding more work that needs to be done, before I can do the work that I need to do.

This hasn't resulted in any big moves on [#1525] itself last week, but there were a bunch of cleanups, improved error messages, and so on. I have some more substantial work in local branches that weren't ready to be merged yet, and I expect more movement there this week.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week. Busy working on the kernel!*


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Continue simplifying object graph around `HalfEdge` ([#1535], [#1536])
- Add more debug information to approximation ([#1537])
- Improve validation error messages ([#1540])
- Respect existing boundary when updating `HalfEdge` as line segment ([#1541])

#### `fj-window`

- Box event loop error variants ([#1539])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1528], [#1529], [#1530], [#1531], [#1533])
- Minimize scope of `unsafe` block ([#1538])
- Update release automation ([#1542])
- Upgrade to Rust 1.67.0 ([#1543])


[#1528]: https://github.com/hannobraun/Fornjot/pull/1528
[#1529]: https://github.com/hannobraun/Fornjot/pull/1529
[#1530]: https://github.com/hannobraun/Fornjot/pull/1530
[#1531]: https://github.com/hannobraun/Fornjot/pull/1531
[#1533]: https://github.com/hannobraun/Fornjot/pull/1533
[#1535]: https://github.com/hannobraun/Fornjot/pull/1535
[#1536]: https://github.com/hannobraun/Fornjot/pull/1536
[#1537]: https://github.com/hannobraun/Fornjot/pull/1537
[#1538]: https://github.com/hannobraun/Fornjot/pull/1538
[#1539]: https://github.com/hannobraun/Fornjot/pull/1539
[#1540]: https://github.com/hannobraun/Fornjot/pull/1540
[#1541]: https://github.com/hannobraun/Fornjot/pull/1541
[#1542]: https://github.com/hannobraun/Fornjot/pull/1542
[#1543]: https://github.com/hannobraun/Fornjot/pull/1543

[#1525]: https://github.com/hannobraun/Fornjot/issues/1525