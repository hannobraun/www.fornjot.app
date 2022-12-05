+++
title = "Weekly Release - New Idea"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-12-05

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.27.0"
subtitle = "Trying out a new approach to solve a big problem."
+++

The main thrust of last week's development was the continuing cleanup of the object construction code ([#1249]). This remains a high priority, as it's blocking work on boolean operations ([#42], [#43], [#44]), which are the next step on the way to making Fornjot actually useful.

I *might* actually be in the middle of a breakthrough. I've had an idea last week ([described here](https://github.com/hannobraun/Fornjot/issues/1249#issuecomment-1333891251)), which could solve most of the issues with the object construction code. I've been prototyping this idea in a local branch, and so far haven't hit any blockers. It's too early to tell, really, but I'm optimistic!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Display more errors in the GUI and display more information about them ([#1405])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Move validation to separate service ([#1403], [#1404])
- Continue cleanup of object construction code ([#1406], [#1407], [#1408], [#1409])
- Rename `GlobalVertex::from_position` to `new` ([#1410])
- Touch up documentation of objects ([#1411])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1397], [#1398], [#1399], [#1400], [#1402])


[#1397]: https://github.com/hannobraun/Fornjot/pull/1397
[#1398]: https://github.com/hannobraun/Fornjot/pull/1398
[#1399]: https://github.com/hannobraun/Fornjot/pull/1399
[#1400]: https://github.com/hannobraun/Fornjot/pull/1400
[#1402]: https://github.com/hannobraun/Fornjot/pull/1402
[#1403]: https://github.com/hannobraun/Fornjot/pull/1403
[#1404]: https://github.com/hannobraun/Fornjot/pull/1404
[#1405]: https://github.com/hannobraun/Fornjot/pull/1405
[#1406]: https://github.com/hannobraun/Fornjot/pull/1406
[#1407]: https://github.com/hannobraun/Fornjot/pull/1407
[#1408]: https://github.com/hannobraun/Fornjot/pull/1408
[#1409]: https://github.com/hannobraun/Fornjot/pull/1409
[#1410]: https://github.com/hannobraun/Fornjot/pull/1410
[#1411]: https://github.com/hannobraun/Fornjot/pull/1411

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#1249]: https://github.com/hannobraun/Fornjot/issues/1249