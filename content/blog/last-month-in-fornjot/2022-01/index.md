+++
title = "Last Month in Fornjot - 2022-01"
date  = 2022-02-02
+++

Welcome to the first issue of Last Month in Fornjot!

This has truly been a big month for the project. At the beginning of January, Fornjot was just a hobby for me. I wasn't [trying to make a living](https://github.com/sponsors/hannobraun) off of it, this website didn't exist, the email newsletter didn't exist, and I wasn't publishing weekly and monthly updates.

The [repository](https://github.com/hannobraun/Fornjot) also changed a lot. A month ago, I was just pushing code to the `main` branch however I pleased. Now there are pull requests, a CI build, bots that update dependencies... this is starting to look somewhat professional!

I've also opened a large number of issues, to document what needs to be done, and to make it easy for people to get involved. This seems to have had some effect, with two new contributors submitting pull requests in January.

Making all of this happen took some time away from writing code, of course, but now that all the new infrastructure is in place, I'm back in the thick of it and excited to see what's going to happen over the next month.


### Sponsors

Fornjot is supported by [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Development Update

Development in January has been a bit slower than it could have been, due to the large amount of work caused by the transition from a hobby to a professional open source project. Still, things got done.

Highlights include improvements to the development infrastructure ([#45](https://github.com/hannobraun/Fornjot/pull/45), [#53](https://github.com/hannobraun/Fornjot/pull/53)), fixes to make the generation of triangle meshes more robust ([#61](https://github.com/hannobraun/Fornjot/pull/61), [#74](https://github.com/hannobraun/Fornjot/pull/74), [#81](https://github.com/hannobraun/Fornjot/pull/81)), more reliable and probably also better performing circle approximation ([#111](https://github.com/hannobraun/Fornjot/pull/111), [#112](https://github.com/hannobraun/Fornjot/pull/112)), as well as many more small fixes and clean-ups.

Some of those improvements have already been published in a new release, [Fornjot 0.5](https://www.fornjot.app/blog/fornjot-0-5-0/).

Check out the Weekly Dev Logs for all the details:

- [2022-W03](https://www.fornjot.app/blog/weekly-dev-log/2022-w03/)
- [2022-W04](https://www.fornjot.app/blog/weekly-dev-log/2022-w04/)


### Issue of the Month

Are you interested in getting involved with Fornjot, but don't know where to start? How about trying issue [#118 - Make tolerance overridable](https://github.com/hannobraun/Fornjot/issues/118)?

This is a small change that doesn't require much knowledge of Fornjot, or even Rust. Compared to its low difficulty, it's likely to have an outsized impact, allowing users to control the accuracy of exported models.

There are many other issues labeled as [`good first issue`](https://github.com/hannobraun/Fornjot/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) in the Fornjot repository, so maybe check those out too!


### Outlook

As I said above, I'm excited to see what's going to happen over the next month. Right now, I'm focused on cleaning up the CAD kernel, to improve its robustness on an architectural level ([#78](https://github.com/hannobraun/Fornjot/issues/78)). Dealing with floating point numbers can be tricky, so this is hopefully going to prevent lots of bugs in the future.

There's an open draft pull request with my current work-in-progress ([#110](https://github.com/hannobraun/Fornjot/pull/110)) and I have already been merging some clean-ups that came out of this work.

After that, I plan to finish another ongoing clean-up ([#97](https://github.com/hannobraun/Fornjot/issues/97)), which will unblock full support for constructive solid geometry ([#42](https://github.com/hannobraun/Fornjot/issues/42), [#43](https://github.com/hannobraun/Fornjot/issues/43), [#44](https://github.com/hannobraun/Fornjot/issues/44)). I don't know how much of that I will get through in the following month. Almost certainly not all of it.
