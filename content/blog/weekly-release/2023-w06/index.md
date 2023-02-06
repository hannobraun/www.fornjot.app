+++
title = "Weekly Release - Ostensibly Quiet"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-02-06

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.35.0"
subtitle = "Work is going on behind the scenes. Not a lot of it is ready to be included in the repository yet."
+++

Ostensibly, this has been a quiet week. Not many pull request, not many commits. But behind the scenes, the object graph simplification work ([#1525]) has been ongoing. Some improvements related to that have made it into the repository this week, but most of it is still blocked by unsolved problems.

I [wrote a short update](https://github.com/hannobraun/Fornjot/issues/1525#issuecomment-1415898182) about that. The short version is, my work simplifying the object graph has been hampered by the complexity of the object graph. Technical debt protecting itself, in a way. At the moment, I'm still looking into ways to solve the specific problems, but it might become necessary to pause this, and try to approach the object graph simplification from another angle.

Meanwhile, [@tmayoff] has made some very welcome improvements in the GUI, while [@Philipp-M] has updated Fornjot's Nix flake. 


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fixed status messages taking up too much space ([#1551]; thank you, [@tmayoff]!)
- Display version mismatch warning in GUI ([#1554]; thank you, [@tmayoff]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve validation error message ([#1553])
- Lift limitation when inferring surface as plane ([#1556])
- Reuse cached curve approximation, if range is reversed ([#1557])

#### `fj-math`

- Improve projections into plane ([#1555])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1546], [#1547], [#1548], [#1550])
- Update list of sponsors ([#1552])
- Updated nix flake inputs ([#1558]; thank you, [@Philipp-M]!)


[#1546]: https://github.com/hannobraun/Fornjot/pull/1546
[#1547]: https://github.com/hannobraun/Fornjot/pull/1547
[#1548]: https://github.com/hannobraun/Fornjot/pull/1548
[#1550]: https://github.com/hannobraun/Fornjot/pull/1550
[#1551]: https://github.com/hannobraun/Fornjot/pull/1551
[#1552]: https://github.com/hannobraun/Fornjot/pull/1552
[#1553]: https://github.com/hannobraun/Fornjot/pull/1553
[#1554]: https://github.com/hannobraun/Fornjot/pull/1554
[#1555]: https://github.com/hannobraun/Fornjot/pull/1555
[#1556]: https://github.com/hannobraun/Fornjot/pull/1556
[#1557]: https://github.com/hannobraun/Fornjot/pull/1557
[#1558]: https://github.com/hannobraun/Fornjot/pull/1558

[@tmayoff]: https://github.com/tmayoff
[@Philipp-M]: https://github.com/Philipp-M

[#1525]: https://github.com/hannobraun/Fornjot/issues/1525
