+++
title = "Weekly Release - Accidental Side-Effect"
date = 2023-02-20

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.37.0"
subtitle = "Stopped trying to fix a bug; did something else entirely, that ended up fixing that bug."
+++

I *finally* managed to finish the partial rewrite of the sweep code that had been kicking my ass for the last few weeks! This took longer than it had any right taking, but it really hammered home how important the ongoing work to simplify the kernel is ([#1589]). This rewrite also had an accidental side-effect: it fixed a long-standing bug ([#494], [#1162]), that had been blocking further work on the union operation ([#42]).

My previous attempt to fix that bug had sucked up a lot of time and was unsuccessful, which is part of what started that current focus on simplification. So it's kind of surprising that the work I did after I no longer worked on this bug is what ended up then fixing it. Programming works in mysterious ways, sometimes.

Even though work on the union operation, which had been my main priority before I went down this recursive rabbit hole of fixing foundational problems, is unblocked now, I am not going to return to it immediately. I am convinced that the ongoing simplification work is the best thing I can do to ensure forward progress mid- to long-term. Prioritizing short-term progress here would only lead into a dead-end.

Meanwhile, we've had more awesome contributions last week! [@erenoku] added the navigation cube, an UI element that shows how the model is currently oriented. [@tmayoff] implemented a workaround for a crash in the Fornjot kernel, when a model creates an empty sketch.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Add UI element that shows model orientation ([#1573], [#1584]; thank you, [@erenoku]!)
- Don't panic, if model code creates empty sketch ([#1585]; thank you, [@tmayoff]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Rewrite parts of sweep code ([#1593])
- Start consolidating redundant references to `Surface` ([#1596], [#1597], [#1598], [#1599])

#### `fj-math`

- Make various minor improvements ([#1590])
- Clean up and simplify `Arc` ([#1591], [#1592])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1576])
- Update dependencies ([#1580], [#1582], [#1583], [#1587], [#1595])


[#1573]: https://github.com/hannobraun/Fornjot/pull/1573
[#1576]: https://github.com/hannobraun/Fornjot/pull/1576
[#1580]: https://github.com/hannobraun/Fornjot/pull/1580
[#1582]: https://github.com/hannobraun/Fornjot/pull/1582
[#1583]: https://github.com/hannobraun/Fornjot/pull/1583
[#1584]: https://github.com/hannobraun/Fornjot/pull/1584
[#1585]: https://github.com/hannobraun/Fornjot/pull/1585
[#1587]: https://github.com/hannobraun/Fornjot/pull/1587
[#1590]: https://github.com/hannobraun/Fornjot/pull/1590
[#1591]: https://github.com/hannobraun/Fornjot/pull/1591
[#1592]: https://github.com/hannobraun/Fornjot/pull/1592
[#1593]: https://github.com/hannobraun/Fornjot/pull/1593
[#1595]: https://github.com/hannobraun/Fornjot/pull/1595
[#1596]: https://github.com/hannobraun/Fornjot/pull/1596
[#1597]: https://github.com/hannobraun/Fornjot/pull/1597
[#1598]: https://github.com/hannobraun/Fornjot/pull/1598
[#1599]: https://github.com/hannobraun/Fornjot/pull/1599

[@erenoku]: https://github.com/erenoku
[@tmayoff]: https://github.com/tmayoff

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#494]: https://github.com/hannobraun/Fornjot/issues/494
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1589]: https://github.com/hannobraun/Fornjot/issues/1589
