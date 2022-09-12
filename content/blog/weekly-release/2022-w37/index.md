+++
title = "Weekly Release - 2022-W37"
date = 2022-09-12

[extra]
version = "0.15.0"
+++

A few weeks ago, I fell down a rabbit hole of kernel cleanups, and I'm starting to climb back up. Doing cleanups can cause me this nagging feeling that I'm indulging myself; that I should find some hack to work around the problem at hand, so I can continue doing what's actually important. But a) stacking hacks on top of each other soon causes more problems than it solves; and b) when those cleanups pay off, that is so sweet.

And last weeks, those cleanups paid off! I managed to simplify lots of kernel code and many issues are now closed ([#97], [#250], [#695], [#1020]). Some of them very longstanding ones! This not only clears the way for continuing the implementation of the union operation, it will make *all* work easier going forward.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

*None this week; busy improving the kernel!*


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Improve sweep algorithm ([#1038], [#1054], [#1061], [#1063], [#1068])
- Add `SurfaceVertex` ([#1048])
- Produce better approximations, validate their correctness ([#1049], [#1053], [#1056], [#1058])
- Make triangulation more flexible ([#1050])
- Add `Faces` ([#1051])
- Simplify `Edge`; perform cleanups this enables ([#1055], [#1057], [#1059], [#1062])
- Rename `Edge` to `HalfEdge` ([#1064])
- Define face orientation by the winding of its exterior cycle ([#1066])
- Add API for finding faces ([#1067])

#### `fj-math`

- Add `Vector<2>::cross` ([#1065])

#### `fj-operations`

- Make use of `Faces` ([#1052])
- Remove unused parameter of `Shape::compute_brep` ([#1060])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1037])
- Update dependencies ([#1039], [#1040], [#1041], [#1042], [#1043], [#1044], [#1045], [#1047])
- Expand release automation ([#1046])


[#1037]: https://github.com/hannobraun/Fornjot/pull/1037
[#1038]: https://github.com/hannobraun/Fornjot/pull/1038
[#1039]: https://github.com/hannobraun/Fornjot/pull/1039
[#1040]: https://github.com/hannobraun/Fornjot/pull/1040
[#1041]: https://github.com/hannobraun/Fornjot/pull/1041
[#1042]: https://github.com/hannobraun/Fornjot/pull/1042
[#1043]: https://github.com/hannobraun/Fornjot/pull/1043
[#1044]: https://github.com/hannobraun/Fornjot/pull/1044
[#1045]: https://github.com/hannobraun/Fornjot/pull/1045
[#1046]: https://github.com/hannobraun/Fornjot/pull/1046
[#1047]: https://github.com/hannobraun/Fornjot/pull/1047
[#1048]: https://github.com/hannobraun/Fornjot/pull/1048
[#1049]: https://github.com/hannobraun/Fornjot/pull/1049
[#1050]: https://github.com/hannobraun/Fornjot/pull/1050
[#1051]: https://github.com/hannobraun/Fornjot/pull/1051
[#1052]: https://github.com/hannobraun/Fornjot/pull/1052
[#1053]: https://github.com/hannobraun/Fornjot/pull/1053
[#1054]: https://github.com/hannobraun/Fornjot/pull/1054
[#1055]: https://github.com/hannobraun/Fornjot/pull/1055
[#1056]: https://github.com/hannobraun/Fornjot/pull/1056
[#1057]: https://github.com/hannobraun/Fornjot/pull/1057
[#1058]: https://github.com/hannobraun/Fornjot/pull/1058
[#1059]: https://github.com/hannobraun/Fornjot/pull/1059
[#1060]: https://github.com/hannobraun/Fornjot/pull/1060
[#1061]: https://github.com/hannobraun/Fornjot/pull/1061
[#1062]: https://github.com/hannobraun/Fornjot/pull/1062
[#1063]: https://github.com/hannobraun/Fornjot/pull/1063
[#1064]: https://github.com/hannobraun/Fornjot/pull/1064
[#1065]: https://github.com/hannobraun/Fornjot/pull/1065
[#1066]: https://github.com/hannobraun/Fornjot/pull/1066
[#1067]: https://github.com/hannobraun/Fornjot/pull/1067
[#1068]: https://github.com/hannobraun/Fornjot/pull/1068


### Issue of the Week

Fornjot is still an early-stage project. Many things don't work very well yet, and that can lead to a rough user experience in some places. One such problem is how the GUI is coupled to the loaded model.

The GUI assumes that a model is always loaded. If a model can't be loaded when you start the Fornjot app, due to some error in the model, the GUI won't show at all. This is *extremely* confusing!

If you're interested in digging into Fornjot's graphics and UI code, to make an improvement that will have a big impact on Fornjot's usability, check out [#1015 - GUI should start immediately, without waiting for model to be loaded](https://github.com/hannobraun/Fornjot/issues/1015).


### Outlook

With all those cleanups finished, I should finally be in a position to address the issue that has been blocking the work on the union operation ([#993]). I expect that to be pretty easy now, but when I started looking into it on Friday (I only had a few minutes left), making the necessary changes caused *a lot* of test failures.

Hopefully this is going to be straight-forward. We'll see.


[#97]: https://github.com/hannobraun/Fornjot/issues/97
[#250]: https://github.com/hannobraun/Fornjot/issues/250
[#695]: https://github.com/hannobraun/Fornjot/issues/695
[#993]: https://github.com/hannobraun/Fornjot/issues/993
[#1020]: https://github.com/hannobraun/Fornjot/issues/1020