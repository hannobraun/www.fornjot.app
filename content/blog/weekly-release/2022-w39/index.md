+++
title = "Weekly Release - 2022-W39"
date = 2022-09-26

[extra]
version = "0.17.0"
+++

I've been busy with cleanups for a few weeks now, and that hasn't changed last week either. My priority remains implementing the union operation ([#42]), but that is currently blocked by [#993], which in turn is blocked by [#1079], which until recently was blocked by [#1021].

I've made enough progress on [#1021] to un-block [#1079], so I've returned my attention there. As it turns out, finishing up that issue requires more changes all around the kernel code than I initially expected, but in the end this isn't really surprising. The changes to the `GlobalCurve` object that are the subject of [#1079] have a follow-on effect on all other objects that reference it, so a lot of code needs to be updated.

Going into the details here would be a bit much, but the gist of it is, there's a new way of doing things ([comparing objects by identity instead of equality](https://github.com/hannobraun/Fornjot/issues/1079#issuecomment-1252416096)). This new way will make the kernel code more reliable, but right now this can't be implemented, because the current code is cutting a lot of corners.

Un-cutting those corners, as well as implementing the infrastructure required to do that without making the affected code too cumbersome, is what I'm currently working on. If you want to know more about that, I suggest to follow the links to those issues, as I keep those updated with the details of the work I'm doing.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

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

- Implement centralized object storage ([#1108], [#1116], [#1121])
- Prepare for removing geometry from `GlobalCurve` ([#1111], [#1114])
- Start converting builder API into partial object API ([#1113], [#1117], [#1118], [#1119], [#1120], [#1123], [#1124], [#1126], [#1128], [#1130], [#1131], [#1133], [#1134], [#1135])
- Simplify `Triangulate` trait ([#1122])
- Clean up `Face` constructor ([#1125])
- Remove `HalfEdge::from_curve_and_vertices` ([#1127])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1101], [#1103], [#1104], [#1105], [#1106], [#1107], [#1109])
- Remove unused dependencies ([#1110])
- Expand release automation ([#1115])
- Upgrade to Rust 1.64.0 ([#1132])
- Update list of sponsors in README ([#1136])


[#1101]: https://github.com/hannobraun/Fornjot/pull/1101
[#1103]: https://github.com/hannobraun/Fornjot/pull/1103
[#1104]: https://github.com/hannobraun/Fornjot/pull/1104
[#1105]: https://github.com/hannobraun/Fornjot/pull/1105
[#1106]: https://github.com/hannobraun/Fornjot/pull/1106
[#1107]: https://github.com/hannobraun/Fornjot/pull/1107
[#1108]: https://github.com/hannobraun/Fornjot/pull/1108
[#1109]: https://github.com/hannobraun/Fornjot/pull/1109
[#1110]: https://github.com/hannobraun/Fornjot/pull/1110
[#1111]: https://github.com/hannobraun/Fornjot/pull/1111
[#1113]: https://github.com/hannobraun/Fornjot/pull/1113
[#1114]: https://github.com/hannobraun/Fornjot/pull/1114
[#1115]: https://github.com/hannobraun/Fornjot/pull/1115
[#1116]: https://github.com/hannobraun/Fornjot/pull/1116
[#1117]: https://github.com/hannobraun/Fornjot/pull/1117
[#1118]: https://github.com/hannobraun/Fornjot/pull/1118
[#1119]: https://github.com/hannobraun/Fornjot/pull/1119
[#1120]: https://github.com/hannobraun/Fornjot/pull/1120
[#1121]: https://github.com/hannobraun/Fornjot/pull/1121
[#1122]: https://github.com/hannobraun/Fornjot/pull/1122
[#1123]: https://github.com/hannobraun/Fornjot/pull/1123
[#1124]: https://github.com/hannobraun/Fornjot/pull/1124
[#1125]: https://github.com/hannobraun/Fornjot/pull/1125
[#1126]: https://github.com/hannobraun/Fornjot/pull/1126
[#1127]: https://github.com/hannobraun/Fornjot/pull/1127
[#1128]: https://github.com/hannobraun/Fornjot/pull/1128
[#1130]: https://github.com/hannobraun/Fornjot/pull/1130
[#1131]: https://github.com/hannobraun/Fornjot/pull/1131
[#1132]: https://github.com/hannobraun/Fornjot/pull/1132
[#1133]: https://github.com/hannobraun/Fornjot/pull/1133
[#1134]: https://github.com/hannobraun/Fornjot/pull/1134
[#1135]: https://github.com/hannobraun/Fornjot/pull/1135
[#1136]: https://github.com/hannobraun/Fornjot/pull/1136


### Issue of the Week

The Fornjot UI can display status updates. For example, that the model has been reloaded, or that there has been an error trying to do just that. A nice incremental improvement to that capability, would be to also display a timestamp with each update.

If you're interested in getting into Fornjot, and are looking for something manageable, maybe check out [#971 - Print timestamp with each status update](https://github.com/hannobraun/Fornjot/issues/971)


### Outlook

Cleanups will continue for the time being. I've been questioning whether I should find some shortcut to get back on track with the union operation ([#42]) instead, but so far I've decided against that. First, it's not clear to me what that shortcut would be. Second, fixing [#1079] and then [#993] the proper way has many positive side effects, making the kernel more reliable and preventing whole classes of bugs.

I'll stay the course for now and will keep re-evaluation the situation as I go on.


[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#993]: https://github.com/hannobraun/Fornjot/issues/993
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1079]: https://github.com/hannobraun/Fornjot/issues/1079
