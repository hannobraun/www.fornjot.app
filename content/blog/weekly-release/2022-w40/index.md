+++
title = "Weekly Release - 2022-W40"
date = 2022-10-04

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.18.0"
+++

Hey folks, sorry for being a day late with the weekly release! Yesterday was a public holiday here in Germany.

Last week, I've been able to finish the round of cleanups that kept me busy over the last few weeks. I addressed [#1079], then fixed [#993], which had been holding up further progress on the union operation ([#42]).

I returned to implementing the intersection tests required for the union operation and made some progress there. I pretty quickly found out that the sweep algorithm (and possibly more code) is generating invalid geometry though ([#1162]), which is preventing further progress on the intersection tests, so I started working on that.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

*None this week. Busy improving the kernel!*


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Improve some panic messages ([#1139], [#1149])
- Improve partial object API ([#1140], [#1144], [#1148], [#1150])
- Fix some code that creates duplicate global curves ([#1145], [#1151], [#1152])
- Remove redundant geometry from `GlobalCurve` ([#1146], [#1153])
- Make `GlobalEdge` undirected ([#1155])
- Validate winding of interior cycles of `Face` ([#1158])
- Add `HorizontalRayToTheRight::direction` ([#1159])
- Integrate `Surface` into centralized object storage ([#1163])

#### `fj-math`

- Add `Plane` ([#1157], [#1160])
- Expand and clean up API of `Vector` ([#1161])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1138])
- Expand release automation ([#1141])
- Update dependencies ([#1142])
- Update screenshot of test model ([#1156])


[#1138]: https://github.com/hannobraun/Fornjot/pull/1138
[#1139]: https://github.com/hannobraun/Fornjot/pull/1139
[#1140]: https://github.com/hannobraun/Fornjot/pull/1140
[#1141]: https://github.com/hannobraun/Fornjot/pull/1141
[#1142]: https://github.com/hannobraun/Fornjot/pull/1142
[#1144]: https://github.com/hannobraun/Fornjot/pull/1144
[#1145]: https://github.com/hannobraun/Fornjot/pull/1145
[#1146]: https://github.com/hannobraun/Fornjot/pull/1146
[#1148]: https://github.com/hannobraun/Fornjot/pull/1148
[#1149]: https://github.com/hannobraun/Fornjot/pull/1149
[#1150]: https://github.com/hannobraun/Fornjot/pull/1150
[#1151]: https://github.com/hannobraun/Fornjot/pull/1151
[#1152]: https://github.com/hannobraun/Fornjot/pull/1152
[#1153]: https://github.com/hannobraun/Fornjot/pull/1153
[#1155]: https://github.com/hannobraun/Fornjot/pull/1155
[#1156]: https://github.com/hannobraun/Fornjot/pull/1156
[#1157]: https://github.com/hannobraun/Fornjot/pull/1157
[#1158]: https://github.com/hannobraun/Fornjot/pull/1158
[#1159]: https://github.com/hannobraun/Fornjot/pull/1159
[#1160]: https://github.com/hannobraun/Fornjot/pull/1160
[#1161]: https://github.com/hannobraun/Fornjot/pull/1161
[#1163]: https://github.com/hannobraun/Fornjot/pull/1163


### Issue of the Week

Fornjot being code-first gives a lot of flexibility to models. They can run arbitrary logic to create geometry, process complex inputs, and perform all kinds of calculations. But they lack ways to present output to the user.

So far, the best they can do is print to `stdout`, which the user might not even see. Fornjot already displays model status updates to the user. If models could display arbitrary message there, that would be a step in the right direction.

Does that sound interesting to you? If so, maybe give [#996 - Models should be able to add status messages](https://github.com/hannobraun/Fornjot/issues/996) a try.


### Outlook

Since [#1162] is preventing further progress on the union operation ([#42]), I'm working on that right now. I'd like to fix that bug properly, so instead of just addressing the specific instances I'm aware of, I'd like to add validation code that makes *all* instances visible.

That new validation code would benefit from the scope of the centralized object storage being expanded ([#1021]), so that's my next step.


[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#993]: https://github.com/hannobraun/Fornjot/issues/993
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1079]: https://github.com/hannobraun/Fornjot/issues/1079
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
