+++
title = "Weekly Release - Really Shouldn't Be That Hard"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-02-13

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.36.0"
subtitle = "When the very thing you're trying to fix complicates your fixes."
+++

I've really been struggling with the ongoing object graph simplification work ([#1525]) this week. I'm still stuck on the rewrite of parts of the sweep algorithm, which I was already working on the week before. I'm down to what should be one last problem (at least I hope).

![a mangled model that should actually be a nice, cylindrical sweep](/blog/weekly-release/2023-w07/mangled-sweep.png)

This should be a cylindrical model, but instead it is totally mangled. I think I understand the problem (the coordinate systems of coincident curves don't match), but there are some subtleties to the solution. In my first few attempts, any fix I tried broke something else.

This is frustrating, because this really shouldn't be that hard. But it *is* hard, because of the unnecessary complexity of the object graph. The very thing I'm working on fixing. This shows how important that simplification work is.

Meanwhile, [@mxdamien] had made a very important improvement: Panics in the model code no longer crash the app!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Don't abort application, if model code panics ([#1534]; thank you, [@mxdamien]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make various fixes and small updates in builder API ([#1572])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1560], [#1561], [#1562], [#1563], [#1564], [#1566], [#1567])
- Upgrade to Rust 1.67.1 ([#1574])


[#1534]: https://github.com/hannobraun/Fornjot/pull/1534
[#1560]: https://github.com/hannobraun/Fornjot/pull/1560
[#1561]: https://github.com/hannobraun/Fornjot/pull/1561
[#1562]: https://github.com/hannobraun/Fornjot/pull/1562
[#1563]: https://github.com/hannobraun/Fornjot/pull/1563
[#1564]: https://github.com/hannobraun/Fornjot/pull/1564
[#1566]: https://github.com/hannobraun/Fornjot/pull/1566
[#1567]: https://github.com/hannobraun/Fornjot/pull/1567
[#1572]: https://github.com/hannobraun/Fornjot/pull/1572
[#1574]: https://github.com/hannobraun/Fornjot/pull/1574

[@mxdamien]: https://github.com/mxdamien

[#1525]: https://github.com/hannobraun/Fornjot/issues/1525
