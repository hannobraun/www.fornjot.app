+++
title = "Weekly Release - Making Room for Distractions"
date = 2022-10-17

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.20.0"
subtitle = "Because too much focus can be detrimental."
+++

My main priority was and remains the implementation of the union operations, but it should have become quite obvious to anyone following these updates, that it's taking a while. Meanwhile, other parts of the project remain in a largely immature state. I have reason to believe that this isn't making a great impression on potential contributors and sponsors, and it isn't great for *my* morale either.

So I've made a decision: In parallel to the ongoing work on the union operation (and boolean operations in general), I'll allocate some time to work on smaller tasks that have an immediate and visible benefit. I've started this effort by picking off some issues that had been open for a while. Check out the list of changes below!

As far as the big picture goes, I've continued the effort to expand the scope of the centralized object storage ([#1021]) and making use of that by enhancing the existing validation code (by using [object identity instead of object equality](https://docs.rs/fj-kernel/0.19.0/fj_kernel/objects/index.html#object-identity-vs-object-equality) for comparisons). This is slow and tedious work, as a lot of code that constructs objects needs to be updated to meet the stricter validation requirements.

All of this leads to more robust code though, and thus fewer bugs. The specific goal I'm working toward is to fix [#1162], which is currently blocking further progress on the union operation ([#42]).

Meanwhile, [@ArshErgon] improved the error message for when a model can't be found, and [@Philipp-M] added a Nix flake.

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Improve error message, if model can't be found ([#1154]; thank you, [@ArshErgon]!)
- Remove old UI ([#1202])
- Invert default zoom direction; add config to override that ([#1204])
- Document convenient syntax for `fj` operations ([#1205])
- Remove the need to specify `crate-type` in `Cargo.toml` ([#1209])
- Fix some `wgpu`/`egui-winit` errors and warnings ([#1216])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Rename `Stores` to `Objects`, move it to `objects` ([#1198])
- Provide access to default planes through `Objects` ([#1200])
- Fix more object duplication issues ([#1206], [#1207], [#1215], [#1218], [#1220], [#1222])
- Expand partial object API([#1212], [#1213])
- Integrate `SurfaceVertex` into centralized object storage ([#1214])
- Add methods to access single `HalfEdge` vertices ([#1219])

#### `fj-math`

- Fix `Triangle::winding` ([#1217])

#### `fj-operations`

- Remove redundant argument from `Shape::compute_brep` ([#1201])

#### `fj-viewer`

- Remove dependency on winit ([#1210])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1195], [#1208])
- Update dependencies ([#1196], [#1197])
- Add Nix build and dev-shell support via Nix flakes ([#1199], [#1203]; thank you, [@Philipp-M]!)
- Clean up egui-related code ([#1211])


[#1154]: https://github.com/hannobraun/Fornjot/pull/1154
[#1195]: https://github.com/hannobraun/Fornjot/pull/1195
[#1196]: https://github.com/hannobraun/Fornjot/pull/1196
[#1197]: https://github.com/hannobraun/Fornjot/pull/1197
[#1198]: https://github.com/hannobraun/Fornjot/pull/1198
[#1199]: https://github.com/hannobraun/Fornjot/pull/1199
[#1200]: https://github.com/hannobraun/Fornjot/pull/1200
[#1201]: https://github.com/hannobraun/Fornjot/pull/1201
[#1202]: https://github.com/hannobraun/Fornjot/pull/1202
[#1203]: https://github.com/hannobraun/Fornjot/pull/1203
[#1204]: https://github.com/hannobraun/Fornjot/pull/1204
[#1205]: https://github.com/hannobraun/Fornjot/pull/1205
[#1206]: https://github.com/hannobraun/Fornjot/pull/1206
[#1207]: https://github.com/hannobraun/Fornjot/pull/1207
[#1208]: https://github.com/hannobraun/Fornjot/pull/1208
[#1209]: https://github.com/hannobraun/Fornjot/pull/1209
[#1210]: https://github.com/hannobraun/Fornjot/pull/1210
[#1211]: https://github.com/hannobraun/Fornjot/pull/1211
[#1212]: https://github.com/hannobraun/Fornjot/pull/1212
[#1213]: https://github.com/hannobraun/Fornjot/pull/1213
[#1214]: https://github.com/hannobraun/Fornjot/pull/1214
[#1215]: https://github.com/hannobraun/Fornjot/pull/1215
[#1216]: https://github.com/hannobraun/Fornjot/pull/1216
[#1217]: https://github.com/hannobraun/Fornjot/pull/1217
[#1218]: https://github.com/hannobraun/Fornjot/pull/1218
[#1219]: https://github.com/hannobraun/Fornjot/pull/1219
[#1220]: https://github.com/hannobraun/Fornjot/pull/1220
[#1222]: https://github.com/hannobraun/Fornjot/pull/1222

[@ArshErgon]: https://github.com/ArshErgon
[@Philipp-M]: https://github.com/Philipp-M
