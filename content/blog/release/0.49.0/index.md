+++
title = "Fornjot 0.49.0"
date = 2024-03-21

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/release.html"

[extra]
version = "0.49.0"

subtitle = "Transitionary"
+++

It's about time for a new release of Fornjot! This one is a bit of a transitionary one, with lots of work that has been started but isn't finished yet. But none the less, there are a few goodies in here.

Let's take a look at the highlights first. A full (curated) changelog is available below.

#### `fj::Instance`

`fj::Instance` serves as the new entry point to the Fornjot API, and you can now easily create an instance of Fornjot by calling `fj::Instance::new()`.

`fj-core`, the most substantial of Fornjot's libraries, also has a new entry point, `fj_core::Core`, which is available as a field on `fj::Instance`.

The following sections have some examples that show off the improved ease of use.

#### Create a shell from vertices and indices

It's now possible to create a shell by providing a bunch of vertices and indices:

``` rust
use fj::core::{objects::Shell, operations::build::BuildShell};

let mut fj = fj::Instance::new();

let tetrahedron = Shell::from_vertices_and_indices(
    [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    [[2, 1, 0], [0, 1, 3], [1, 2, 3], [2, 0, 3]],
    &mut fj.core,
);
```

Here's the result:

![A tetrahedron](/blog/release/0.49.0/tetrahedron.png)

This is not the most convenient way to create a shape, nor the most powerful one, but it does allow you to specify any arbitrary polyhedron. Where the shape is too complex for less general methods, but not yet too complex to get unwieldy, there's a sweet spot for this approach.

#### Layers

The core of Fornjot's data structure is the object graph, which describes shapes as a graph of interrelated objects (like faces, edges, vertices, and more). Traditionally, this graph has contained all the data necessary to describe a shape. But in this release, I've started to introduce the concept of dedicated layers.

I've already finished extraction color information to a separate presentation layer. This makes it possible to update the color of an object, without having to create a new, modified object.

```rust
let mut fj = fj::Instance::new();

let size = 1.;
let cuboid = cuboid::model([size, size, size], &mut fj.core);

cuboid
  .shells().only()
  .faces().first()
  .region()
  .set_color([0., 1., 0.], &mut fj.core);
```

Here's the result:

![A mostly red cube with one green face](/blog/release/0.49.0/cube-with-color.png)

This was mostly a test run, and the main effort is still ongoing: Extracting all geometric information from the object graph into a new geometry layer.

The goal is to leave the object graph as a purely topological data structure, while geometry is defined separately from that, referencing the object graph and enriching it with geometric data.

Once this work is finished, I'm hoping to get some nice advantages from that:

- **Simplified object graph:** So far, every simplification of the object graph that I've managed has been a huge win in maintainability throughout the whole project. This will hopefully have the same effect.
- **Easier and more efficient updates of geometry:** If geometry is part of the object graph, it's necessary to create a new object with modified geometry to update it. This is not practical, if you're using Fornjot in a GUI app, your user is dragging an edge over the screen, and you need to re-create most of the object graph every pixel to show a preview.
- **Easier to experiment with geometry:** I have big plans for how to make the geometry representation in Fornjot more flexible, and thus much more capable. Having geometry not all tied up in the object graph should help a lot with making those changes.

This is currently the main focus of my ongoing work, and I hope to have more progress to show soon.

#### Improved validation infrastructure

Validation is a critical piece of Fornjot, which checks your shapes and points out any problems. In this release, I've started to create a new and improved validation infrastructure. This work is still ongoing, and most validation checks still run on the old infrastructure.

I'm taking this as an opportunity to clean up and document all the validation checks, which is a big win in itself. But the new validation infrastructure also has some inherent advantages over the old one.

##### 1. Validation checks are now types

Whereas before, a validation check was a combination of a method and an error variant, hidden away in a private module, each validation check is now a dedicated type. This makes validation checks more discoverable, and provides a place to document them.

Here's an example from Fornjot's API reference:

![Screenshot from Fornjot's API reference, showing the documentation of a validation check](/blog/release/0.49.0/validation-check-docs.png)

##### 2. Validation checks implement a trait

There already was a validation trait, but it was implemented per-object. With the new trait being implemented per-check, but referencing the object, it shows up in the object's documentation, further aiding discoverability.

Here's how that looks:

![Screenshot from Fornjot's API reference, showing a validation check referenced from an object's documentation](/blog/release/0.49.0/validation-check-in-object-docs.png)

##### 3. Easier code sharing

With validation checks no longer organized strictly per-object, it's now easier to share code between similar checks, or even implement the same validation check for multiple types of object.

#### More flexible exporting

Fornjot can currently export shapes to `.3mf`, `.obj`, and `.stl` files. Before, you had to export the shape to an actual file, but now you can pass any `impl io::Write + io::Seek`, providing you better flexibility in where the exported data actually goes.

In addition, you can now explicitly choose which format to export to via the respective methods in the `fj-export` crate, whereas before, the format was chosen automatically based on the file extension.

This work was driven and implemented by contributor [@IamTheCarl]. Thank you!

#### Cleanups, fixes, improvements to documentation

In addition to the highlights already mentioned, we have the usual avalanche of small improvements, fixes, and documentation updates. Check out the pull requests linked below, for a full view of what happened!

I'd like to thank all the contributors who helped out with this release. Thank you [@nathan-folsom], [@brungardtdb], [@watana318], [@IamTheCarl], and [@emka]! You're awesome!


### Sponsors

Fornjot is supported by [@MitchellHansen](https://github.com/MitchellHansen), [@webtrax-oz](https://github.com/webtrax-oz), [@seanjensengrey](https://github.com/seanjensengrey), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), [@sucaba](https://github.com/sucaba), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### Library improvements

Improvements to Fornjot libraries.

#### `fj`

- Add `fj::Instance` ([#2217])

#### `fj-core`

- Expand and improve validation checking ([#2144], [#2148], [#2256]; thank you, [@nathan-folsom] and [@emka]!)
- Add operation for creating shell from vertices and indices ([#2149])
- Make background validation configurable ([#2150]; thank you, [@brungardtdb]!)
- Make some cleanups in `transform`, move it to `operations` ([#2169])
- Refactor `services` into `layers` ([#2212], [#2213], [#2214], [#2215])
- Move color information to dedicated presentation layer ([#2167], [#2168], [#2179], [#2219], [#2220], [#2221], [#2230], [#2232], [#2233], [#2234])
- Add `IsObject` trait ([#2178])
- Rename `Object` to `AnyObject`; clean it up ([#2185], [#2186], [#2187])
- Improve documentation ([#2188], [#2209]; thank you, [@watana318]!)
- Add new entry point into API: `Core` ([#2191], [#2192], [#2196], [#2197], [#2198], [#2207], [#2210], [#2211], [#2218])
- Clean up `update` operations API ([#2200], [#2203], [#2205], [#2206], [#2253], [#2255]; thank you, [@IamTheCarl]!)
- Don't require dropping validation layer to handle validation errors ([#2216])
- Fix panic in `ObjectSet` due to equal but not identical objects ([#2228])
- Start cleanup of validation infrastructure ([#2229], [#2231], [#2235], [#2242], [#2277])
- Remove unused intersection checks ([#2236], [#2268])
- Start moving geometry from object graph to dedicate geometry layer ([#2237], [#2241], [#2245], [#2246], [#2247], [#2261], [#2263], [#2265], [#2267], [#2270], [#2271], [#2272], [#2274], [#2275])
- Expand `build` operations API ([#2239], [#2240], [#2276])
- Return more information from shell-face sweep operation ([#2260])
- Lots of minor clean-ups ([#2147], [#2195], [#2199], [#2204], [#2208], [#2262], [#2273])


#### `fj-export`

- Upgrade to `3mf-rs` 0.5 ([#2248]; thank you, [@IamTheCarl]!)
- Make export functions more general; make them public ([#2252], [#2254]; thank you, [@IamTheCarl]!)

#### `fj-interop`

- Clean up crate ([#2165])
- Add various conversions to `Color` ([#2166])

#### `fj-viewer`

- Upgrade to wgpu 0.19 ([#2182])


### Other changes

Improvements that are not associated with a specific Fornjot library.

- Improve release procedure ([#2141], [#2142], [#2177])
- Clean up example models ([#2143], [#2183], [#2184])
- Keep dependencies up-to-date ([#2146], [#2151], [#2152], [#2154], [#2156], [#2173], [#2181], [#2190], [#2202], [#2225], [#2238], [#2251], [#2257], [#2258], [#2269])
- Keep Rust version up-to-date ([#2153], [#2226])
- Keep README files in repository up-to-date ([#2155], [#2163], [#2176], [#2189], [#2243], [#2244], [#2259])
- Update `CONTRIBUTING.md` ([#2175])


[#2141]: https://github.com/hannobraun/fornjot/pull/2141
[#2142]: https://github.com/hannobraun/fornjot/pull/2142
[#2143]: https://github.com/hannobraun/fornjot/pull/2143
[#2144]: https://github.com/hannobraun/fornjot/pull/2144
[#2146]: https://github.com/hannobraun/fornjot/pull/2146
[#2147]: https://github.com/hannobraun/fornjot/pull/2147
[#2148]: https://github.com/hannobraun/fornjot/pull/2148
[#2149]: https://github.com/hannobraun/fornjot/pull/2149
[#2150]: https://github.com/hannobraun/fornjot/pull/2150
[#2151]: https://github.com/hannobraun/fornjot/pull/2151
[#2152]: https://github.com/hannobraun/fornjot/pull/2152
[#2153]: https://github.com/hannobraun/fornjot/pull/2153
[#2154]: https://github.com/hannobraun/fornjot/pull/2154
[#2155]: https://github.com/hannobraun/fornjot/pull/2155
[#2156]: https://github.com/hannobraun/fornjot/pull/2156
[#2163]: https://github.com/hannobraun/fornjot/pull/2163
[#2165]: https://github.com/hannobraun/fornjot/pull/2165
[#2166]: https://github.com/hannobraun/fornjot/pull/2166
[#2167]: https://github.com/hannobraun/fornjot/pull/2167
[#2168]: https://github.com/hannobraun/fornjot/pull/2168
[#2169]: https://github.com/hannobraun/fornjot/pull/2169
[#2173]: https://github.com/hannobraun/fornjot/pull/2173
[#2175]: https://github.com/hannobraun/fornjot/pull/2175
[#2176]: https://github.com/hannobraun/fornjot/pull/2176
[#2177]: https://github.com/hannobraun/fornjot/pull/2177
[#2178]: https://github.com/hannobraun/fornjot/pull/2178
[#2179]: https://github.com/hannobraun/fornjot/pull/2179
[#2181]: https://github.com/hannobraun/fornjot/pull/2181
[#2182]: https://github.com/hannobraun/fornjot/pull/2182
[#2183]: https://github.com/hannobraun/fornjot/pull/2183
[#2184]: https://github.com/hannobraun/fornjot/pull/2184
[#2185]: https://github.com/hannobraun/fornjot/pull/2185
[#2186]: https://github.com/hannobraun/fornjot/pull/2186
[#2187]: https://github.com/hannobraun/fornjot/pull/2187
[#2188]: https://github.com/hannobraun/fornjot/pull/2188
[#2189]: https://github.com/hannobraun/fornjot/pull/2189
[#2190]: https://github.com/hannobraun/fornjot/pull/2190
[#2191]: https://github.com/hannobraun/fornjot/pull/2191
[#2192]: https://github.com/hannobraun/fornjot/pull/2192
[#2195]: https://github.com/hannobraun/fornjot/pull/2195
[#2196]: https://github.com/hannobraun/fornjot/pull/2196
[#2197]: https://github.com/hannobraun/fornjot/pull/2197
[#2198]: https://github.com/hannobraun/fornjot/pull/2198
[#2199]: https://github.com/hannobraun/fornjot/pull/2199
[#2200]: https://github.com/hannobraun/fornjot/pull/2200
[#2202]: https://github.com/hannobraun/fornjot/pull/2202
[#2203]: https://github.com/hannobraun/fornjot/pull/2203
[#2204]: https://github.com/hannobraun/fornjot/pull/2204
[#2205]: https://github.com/hannobraun/fornjot/pull/2205
[#2206]: https://github.com/hannobraun/fornjot/pull/2206
[#2207]: https://github.com/hannobraun/fornjot/pull/2207
[#2208]: https://github.com/hannobraun/fornjot/pull/2208
[#2209]: https://github.com/hannobraun/fornjot/pull/2209
[#2210]: https://github.com/hannobraun/fornjot/pull/2210
[#2211]: https://github.com/hannobraun/fornjot/pull/2211
[#2212]: https://github.com/hannobraun/fornjot/pull/2212
[#2213]: https://github.com/hannobraun/fornjot/pull/2213
[#2214]: https://github.com/hannobraun/fornjot/pull/2214
[#2215]: https://github.com/hannobraun/fornjot/pull/2215
[#2216]: https://github.com/hannobraun/fornjot/pull/2216
[#2217]: https://github.com/hannobraun/fornjot/pull/2217
[#2218]: https://github.com/hannobraun/fornjot/pull/2218
[#2219]: https://github.com/hannobraun/fornjot/pull/2219
[#2220]: https://github.com/hannobraun/fornjot/pull/2220
[#2221]: https://github.com/hannobraun/fornjot/pull/2221
[#2225]: https://github.com/hannobraun/fornjot/pull/2225
[#2226]: https://github.com/hannobraun/fornjot/pull/2226
[#2228]: https://github.com/hannobraun/fornjot/pull/2228
[#2229]: https://github.com/hannobraun/fornjot/pull/2229
[#2230]: https://github.com/hannobraun/fornjot/pull/2230
[#2231]: https://github.com/hannobraun/fornjot/pull/2231
[#2232]: https://github.com/hannobraun/fornjot/pull/2232
[#2233]: https://github.com/hannobraun/fornjot/pull/2233
[#2234]: https://github.com/hannobraun/fornjot/pull/2234
[#2235]: https://github.com/hannobraun/fornjot/pull/2235
[#2236]: https://github.com/hannobraun/fornjot/pull/2236
[#2237]: https://github.com/hannobraun/fornjot/pull/2237
[#2238]: https://github.com/hannobraun/fornjot/pull/2238
[#2239]: https://github.com/hannobraun/fornjot/pull/2239
[#2240]: https://github.com/hannobraun/fornjot/pull/2240
[#2241]: https://github.com/hannobraun/fornjot/pull/2241
[#2242]: https://github.com/hannobraun/fornjot/pull/2242
[#2243]: https://github.com/hannobraun/fornjot/pull/2243
[#2244]: https://github.com/hannobraun/fornjot/pull/2244
[#2245]: https://github.com/hannobraun/fornjot/pull/2245
[#2246]: https://github.com/hannobraun/fornjot/pull/2246
[#2247]: https://github.com/hannobraun/fornjot/pull/2247
[#2248]: https://github.com/hannobraun/fornjot/pull/2248
[#2251]: https://github.com/hannobraun/fornjot/pull/2251
[#2252]: https://github.com/hannobraun/fornjot/pull/2252
[#2253]: https://github.com/hannobraun/fornjot/pull/2253
[#2254]: https://github.com/hannobraun/fornjot/pull/2254
[#2255]: https://github.com/hannobraun/fornjot/pull/2255
[#2256]: https://github.com/hannobraun/fornjot/pull/2256
[#2257]: https://github.com/hannobraun/fornjot/pull/2257
[#2258]: https://github.com/hannobraun/fornjot/pull/2258
[#2259]: https://github.com/hannobraun/fornjot/pull/2259
[#2260]: https://github.com/hannobraun/fornjot/pull/2260
[#2261]: https://github.com/hannobraun/fornjot/pull/2261
[#2262]: https://github.com/hannobraun/fornjot/pull/2262
[#2263]: https://github.com/hannobraun/fornjot/pull/2263
[#2265]: https://github.com/hannobraun/fornjot/pull/2265
[#2267]: https://github.com/hannobraun/fornjot/pull/2267
[#2268]: https://github.com/hannobraun/fornjot/pull/2268
[#2269]: https://github.com/hannobraun/fornjot/pull/2269
[#2270]: https://github.com/hannobraun/fornjot/pull/2270
[#2271]: https://github.com/hannobraun/fornjot/pull/2271
[#2272]: https://github.com/hannobraun/fornjot/pull/2272
[#2273]: https://github.com/hannobraun/fornjot/pull/2273
[#2274]: https://github.com/hannobraun/fornjot/pull/2274
[#2275]: https://github.com/hannobraun/fornjot/pull/2275
[#2276]: https://github.com/hannobraun/fornjot/pull/2276
[#2277]: https://github.com/hannobraun/fornjot/pull/2277

[@nathan-folsom]: https://github.com/nathan-folsom
[@brungardtdb]: https://github.com/brungardtdb
[@watana318]: https://github.com/watana318
[@IamTheCarl]: https://github.com/IamTheCarl
[@emka]: https://github.com/emka
