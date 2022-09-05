+++
title = "Weekly Release - 2022-W36"
date = 2022-09-05

[extra]
version = "0.14.0"
+++

I spent most of last week working on CAD kernel cleanups, with the goal of addressing the issue that is blocking further progress on the union operation ([#993]; also see the [more detailed update](https://github.com/hannobraun/Fornjot/issues/993#issuecomment-1234425471) I left in there).

 I'm very happy with the progress I've made, but I also hope to finish this work soon, so I can return to working on the union operation ([#42]) directly.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix crash on some graphics hardware ([#1035])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Clean up approximation code ([#1011], [#1012], [#1013], [#1028])
- Clean up and expand `algorithms::reverse` ([#1017])
- Fix edge cases in object equality comparisons ([#1018], [#1022])
- Improve and expand object validation ([#1023], [#1024], [#1030], [#1031])
- Make small improvements in kernel ([#1025])
- Clean up sweep algorithm ([#1026], [#1033])

#### `fj-math`

- Add some validation code to `PolyChain` ([#1027])
- Derive `Default` for all math types ([#1029])
- Add `Line::from_points_with_line_coords` ([#1032])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1008], [#1009])
- Update dependencies ([#1010])
- Expand release automation ([#1016])


[#1008]: https://github.com/hannobraun/Fornjot/pull/1008
[#1009]: https://github.com/hannobraun/Fornjot/pull/1009
[#1010]: https://github.com/hannobraun/Fornjot/pull/1010
[#1011]: https://github.com/hannobraun/Fornjot/pull/1011
[#1012]: https://github.com/hannobraun/Fornjot/pull/1012
[#1013]: https://github.com/hannobraun/Fornjot/pull/1013
[#1016]: https://github.com/hannobraun/Fornjot/pull/1016
[#1017]: https://github.com/hannobraun/Fornjot/pull/1017
[#1018]: https://github.com/hannobraun/Fornjot/pull/1018
[#1022]: https://github.com/hannobraun/Fornjot/pull/1022
[#1023]: https://github.com/hannobraun/Fornjot/pull/1023
[#1024]: https://github.com/hannobraun/Fornjot/pull/1024
[#1025]: https://github.com/hannobraun/Fornjot/pull/1025
[#1026]: https://github.com/hannobraun/Fornjot/pull/1026
[#1027]: https://github.com/hannobraun/Fornjot/pull/1027
[#1028]: https://github.com/hannobraun/Fornjot/pull/1028
[#1029]: https://github.com/hannobraun/Fornjot/pull/1029
[#1030]: https://github.com/hannobraun/Fornjot/pull/1030
[#1031]: https://github.com/hannobraun/Fornjot/pull/1031
[#1032]: https://github.com/hannobraun/Fornjot/pull/1032
[#1033]: https://github.com/hannobraun/Fornjot/pull/1033
[#1035]: https://github.com/hannobraun/Fornjot/pull/1035


### Issue of the Week

One important function of a CAD application is to export the CAD model to external file formats, for further processing in other software. Fornjot currently supports export to STL and 3MF. Both of those formats are comparable, in that they contain a triangle mesh.

3MF is more tightly specified than STL, however, and we make use of that by verifying the validity of exported 3MF files as part of our CI build. This has caught a lot of bugs already! We use a custom tool based on [lib3mf](https://github.com/3MFConsortium/lib3mf) to do that, but unfortunately this tool only works on Linux, making it impossible for users of other operating systems to do a full build locally.

If you have access to Windows or macOS and know a thing or two about linking C/C++ code into a Rust program (or would like to learn), [#920 - `export-validator` doesn't support macOS and Windows](https://github.com/hannobraun/Fornjot/issues/920) might interest you.


### Outlook

My priority remains the union operation ([#42]), which is currently blocked by [#993]. Addressing that will keep me busy with cleanups for some more time.

Next up is finishing the cleanup of the sweep algorithm, which should help unblock [#993] and [#1020]. After that, I plan to address [#1020], as that will simplify a lot of code, promises to address some long-standing issues ([#97], [#250]), and should make the next steps easier. After that, I'll look into [#695], as solving that will help address [#993].


[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#250]: https://github.com/hannobraun/Fornjot/issues/250
[#695]: https://github.com/hannobraun/Fornjot/issues/695
[#993]: https://github.com/hannobraun/Fornjot/issues/993
[#1020]: https://github.com/hannobraun/Fornjot/issues/1020
