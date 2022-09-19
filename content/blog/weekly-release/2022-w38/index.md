+++
title = "Weekly Release - 2022-W38"
date = 2022-09-19

[extra]
version = "0.16.0"
+++

At the beginning of last week, I had just wrapped up a round of cleanups and was about to return to [#993], which is blocking further work on the union operation ([#42]). Instead, I found another problem that I need to address first ([#1079]).

All of this is relatively complex, concerning the architecture of the CAD kernel data structures. If you're interested in the details, check out those issues, but here's my attempt at a high-level summary:
- For the the union operation ([#42]), lots of intersection testing code needs to be written.
- One of those intersection tests needs to determine whether two edges are equal, which isn't really possible right now ([#993]).
- Fixing that isn't possible, because curves can't really compared for equality either ([#1079]).
- The best way to fix that involves having a central data store for curves ([#1021]).

I've made good progress on [#1079] and started working on [#1021]. I ran into some trouble with that, preventing the work I already have in a local branch from getting merged on Friday.

Meanwhile, [@payload] addressed a very confusing problem with the Fornjot app, making sure the GUI is displayed, even if there's a problem with loading the model.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix GUI not being loaded, if model is not available ([#1095]; thank you, [@payload]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-math`

- Fix `Vector::unit_v` ([#1085])
- Expand `Circle` API ([#1086], [#1088])
- Expand `Scalar` API ([#1087], [#1093])

#### `fj-kernel`

- Improve validation of `HalfEdge` and `Vertex` ([#1075])
- Expand builder API ([#1076], [#1083])
- Expand sweep test suite ([#1077])
- Perform various cleanups ([#1080], [#1084])
- Replace `CurveKind` with `SurfacePath`/`GlobalPath` ([#1081])
- Make path approximation deterministic ([#1089], [#1090], [#1094])
- Future-proof curve approximation code ([#1082], [#1091], [#1092], [#1096])
- Un-derive `Copy` from various object types ([#1097])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1071], [#1072], [#1074])
- Update release procedure ([#1073])
- Expand release automation ([#1078])
- Update list of sponsors in README ([#1098])


[#1071]: https://github.com/hannobraun/Fornjot/pull/1071
[#1072]: https://github.com/hannobraun/Fornjot/pull/1072
[#1073]: https://github.com/hannobraun/Fornjot/pull/1073
[#1074]: https://github.com/hannobraun/Fornjot/pull/1074
[#1075]: https://github.com/hannobraun/Fornjot/pull/1075
[#1076]: https://github.com/hannobraun/Fornjot/pull/1076
[#1077]: https://github.com/hannobraun/Fornjot/pull/1077
[#1078]: https://github.com/hannobraun/Fornjot/pull/1078
[#1080]: https://github.com/hannobraun/Fornjot/pull/1080
[#1081]: https://github.com/hannobraun/Fornjot/pull/1081
[#1082]: https://github.com/hannobraun/Fornjot/pull/1082
[#1083]: https://github.com/hannobraun/Fornjot/pull/1083
[#1084]: https://github.com/hannobraun/Fornjot/pull/1084
[#1085]: https://github.com/hannobraun/Fornjot/pull/1085
[#1086]: https://github.com/hannobraun/Fornjot/pull/1086
[#1087]: https://github.com/hannobraun/Fornjot/pull/1087
[#1088]: https://github.com/hannobraun/Fornjot/pull/1088
[#1089]: https://github.com/hannobraun/Fornjot/pull/1089
[#1090]: https://github.com/hannobraun/Fornjot/pull/1090
[#1091]: https://github.com/hannobraun/Fornjot/pull/1091
[#1092]: https://github.com/hannobraun/Fornjot/pull/1092
[#1093]: https://github.com/hannobraun/Fornjot/pull/1093
[#1094]: https://github.com/hannobraun/Fornjot/pull/1094
[#1095]: https://github.com/hannobraun/Fornjot/pull/1095
[#1096]: https://github.com/hannobraun/Fornjot/pull/1096
[#1097]: https://github.com/hannobraun/Fornjot/pull/1097
[#1098]: https://github.com/hannobraun/Fornjot/pull/1098

[@payload]: https://github.com/payload


### Issue of the Week

Fornjot models can have parameters, and those can be provided by the user as command-line arguments. But if the user provides an argument that the model doesn't actually use (which could be the result of a simple typo), that argument just gets ignored. This is confusing.

If you're interested in how the Fornjot application interacts with models, maybe issue [#821 - Exit with error, if user provides unused parameters](https://github.com/hannobraun/Fornjot/issues/821) could be your way to get into that.


### Outlook

I've been doing some thinking over the weekend, and I believe I've come up with solutions for the issues that prevented my work on [#1021] from getting merged last week. I expect to make enough progress there to start finishing up [#1079], and after that, the road should be clear for [#993].

I said something similar last week, so take it with a grain of salt. More issues popping up and needing to be addressed first, wouldn't be a surprise.


[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#993]: https://github.com/hannobraun/Fornjot/issues/993
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1079]: https://github.com/hannobraun/Fornjot/issues/1079
