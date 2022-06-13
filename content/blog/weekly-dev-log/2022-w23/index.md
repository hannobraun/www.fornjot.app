+++
title = "Weekly Dev Log - 2022-W23"
date  = 2022-06-13
+++

Looking at the number of pull requests, this was an exceptionally slow week. I spent a lot of my time thinking about the Fornjot kernel's data structures. This lead to some immediate (if minor) simplifications, but mostly to interesting ideas. Nothing actionable as of yet, but I hope that long-term, we can improve those data structures to simplify the kernel and increase robustness. We'll see. I'll keep thinking.

At the end of the week, I had a break-through in my work on [#568], removing the main blocker that I have been working on for weeks now ([#680]). Unfortunately, there wasn't any time left to follow up on that.

Meanwhile, [@chrisprice] continued his work on improving the input code, fixing the previously unintuitive rotation behavior of the model. [@kamirr] fixed `fj`'s `serialization` feature, which somewhat embarrassingly was completely broken.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Make model rotation behavior intuitive ([#669]; thanks to [@chrisprice]!)
- Update list of sponsors ([#671])
- Fix color of sketches being ignored ([#675])
- Fix serialization in `fj` crate ([#682]; thanks to [@kamirr]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Embed `Point<3>` in `Vertex` directly ([#673])
- Rebrand geometric validation as coherence validation ([#676])
- Make `CycleBuilder` more flexible ([#679])
- Add local forms of cycles to `Face` ([#680])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#670], [#672])
- Small fixes ([#674])


### Issue of the Week

When creating a CAD model, you often need to take a close look, and to do that, you need fine control over moving and rotating the model. Fornjot tries to address this with an intuitive approach, based on where the mouse pointer is touching the model (called the "focus point").

This is a work in progress, and some kinks still need to be ironed out. One such kink is [#678 - Moving the model should work without a focus point](https://github.com/hannobraun/Fornjot/issues/678). If you are interested in user interfaces, this issue could be a great introduction to Fornjot!


### Outlook

Given the breakthrough I had last week, it feels like I'm really close to wrapping up [#568]. That, or I'll uncover more previously unknown problems shortly. Either way, it's progress.


[#669]: https://github.com/hannobraun/Fornjot/pull/669
[#670]: https://github.com/hannobraun/Fornjot/pull/670
[#671]: https://github.com/hannobraun/Fornjot/pull/671
[#672]: https://github.com/hannobraun/Fornjot/pull/672
[#673]: https://github.com/hannobraun/Fornjot/pull/673
[#674]: https://github.com/hannobraun/Fornjot/pull/674
[#675]: https://github.com/hannobraun/Fornjot/pull/675
[#676]: https://github.com/hannobraun/Fornjot/pull/676
[#679]: https://github.com/hannobraun/Fornjot/pull/679
[#680]: https://github.com/hannobraun/Fornjot/pull/680
[#682]: https://github.com/hannobraun/Fornjot/pull/682

[#568]: https://github.com/hannobraun/Fornjot/issues/568

[@chrisprice]: https://github.com/chrisprice
[@kamirr]: https://github.com/kamirr
