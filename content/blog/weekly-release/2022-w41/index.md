+++
title = "Weekly Release - 2022-W41"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-10-10

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.19.0"
+++

I've continued the slow work of expanding the scope of the centralized object storage ([#1021]). Including a new object in the centralized object storage opens opportunities for making any validation code that deals with that object more strict, which helps to prevent bugs. I wrote [documentation about that](https://github.com/hannobraun/Fornjot/blob/1417e0a612729bf3a1b4f3f545e14e3d92f99d48/crates/fj-kernel/src/objects/mod.rs#L8-L71), in case you're interested in the details.

The gist of it is, that sometimes you expect two references to an object to be the same. You can check that using equality (do they look the same?) or identity (are they actually the same object?), the latter being much stricter and less error-prone. The bulk of the work is to update any code that triggers to fulfill those stricter requirements.

Last week, I've integrated `Curve` and `GlobalVertex` into the centralized object storage, but most code creating `GlobalVertex` instances has not been updated to meet the stricter validation requirements yet.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week. Still busy improving the kernel!*


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make minor cleanups in sweep code ([#1167])
- Fix various instances of duplicate objects being created ([#1168], [#1170], [#1172], [#1174])
- Expand and improve partial object API ([#1169], [#1171])
- Improve `Debug` implementation of `ObjectId` ([#1173])
- Simplify `HalfEdge` and `Vertex` ([#1175], [#1178])
- Expand scope of centralized object storage ([#1176], [#1179], [#1180])
- Clean up handling of vertices in normalized order ([#1181])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1165], [#1177])
- Update dependencies ([#1166], [#1182], [#1183], [#1184], [#1185], [#1186], [#1187], [#1188], [#1189], [#1192])


[#1165]: https://github.com/hannobraun/Fornjot/pull/1165
[#1166]: https://github.com/hannobraun/Fornjot/pull/1166
[#1167]: https://github.com/hannobraun/Fornjot/pull/1167
[#1168]: https://github.com/hannobraun/Fornjot/pull/1168
[#1169]: https://github.com/hannobraun/Fornjot/pull/1169
[#1170]: https://github.com/hannobraun/Fornjot/pull/1170
[#1171]: https://github.com/hannobraun/Fornjot/pull/1171
[#1172]: https://github.com/hannobraun/Fornjot/pull/1172
[#1173]: https://github.com/hannobraun/Fornjot/pull/1173
[#1174]: https://github.com/hannobraun/Fornjot/pull/1174
[#1175]: https://github.com/hannobraun/Fornjot/pull/1175
[#1176]: https://github.com/hannobraun/Fornjot/pull/1176
[#1177]: https://github.com/hannobraun/Fornjot/pull/1177
[#1178]: https://github.com/hannobraun/Fornjot/pull/1178
[#1179]: https://github.com/hannobraun/Fornjot/pull/1179
[#1180]: https://github.com/hannobraun/Fornjot/pull/1180
[#1181]: https://github.com/hannobraun/Fornjot/pull/1181
[#1182]: https://github.com/hannobraun/Fornjot/pull/1182
[#1183]: https://github.com/hannobraun/Fornjot/pull/1183
[#1184]: https://github.com/hannobraun/Fornjot/pull/1184
[#1185]: https://github.com/hannobraun/Fornjot/pull/1185
[#1186]: https://github.com/hannobraun/Fornjot/pull/1186
[#1187]: https://github.com/hannobraun/Fornjot/pull/1187
[#1188]: https://github.com/hannobraun/Fornjot/pull/1188
[#1189]: https://github.com/hannobraun/Fornjot/pull/1189
[#1192]: https://github.com/hannobraun/Fornjot/pull/1192


### Outlook

I'm continuing my systematic approach of increasing the strictness of the validation code, thereby increasing the general robustness of the kernel code. As a side effect, this should take care of [#1162], which then clears the road for further progress on the union operation ([#42]).


[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
