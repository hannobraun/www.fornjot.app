+++
title = "Fornjot 0.6"
path  = "/blog/fornjot-0.6"
+++

[Fornjot](https://www.fornjot.app/) is an early-stage project to create a next-generation Code-CAD application. This is the announcement for Fornjot's new release, version 0.6.0.

**Fornjot is still at an early stage and far from being useful as a general-purpose CAD application.** This release should be seen as a preview for anyone who's interested in following Fornjot's development.

The project is steadily inching forward, however, and this new version comes with some new and refined modeling features, lots of small improvements, lots of bug fixes, and a huge amount of internal cleanup.

This release announcement provides a high-level summary of those changes. For more details, please refer to the [changelog](https://github.com/hannobraun/Fornjot/blob/main/CHANGELOG.md). For pre-compiled binaries, check out the [release on GitHub](https://github.com/hannobraun/Fornjot/releases/tag/v0.6.0). Please report bugs in any of the usual [community channels](/community).


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Modeling Improvements

The ways that models can be defined have been improved in various ways. For example, it's now possible to sweep 2D shapes in arbitrary directions, not just along the z-axis.

This was already possible:
![3D model of a cylinder with a circular hole along its height](/blog/fornjot-0.6/sweep-old.png)

And now we can also do this:
![a similar cylinder, slanted to the right](/blog/fornjot-0.6/sweep-new.png)

And if you don't like the classic "Fornjot red", you have options now:
![3D model of a cylinder with a circular hole, colored blue](/blog/fornjot-0.6/color.png)

There's more: `fj::Union` has been renamed to `fj::Group` (with support for actual unions still being a work-in-progress), new convenient syntax for `fj::Difference2d` was added, documentation of the `fj` crate has been improved, and other small tweaks have been made.


### Bug Fixes

You can define abstract geometry all day, but at some point (namely to render it, or to export it for 3D printing), you need to approximate it using a triangle mesh. This process is non-trivial and provides lots of opportunity for exciting bugs.

Here's what happened, if you tried to sweep a non-symmetric sketch:

![a sweep of a non-symmetric sketch; totally borked](/blog/fornjot-0.6/non-symmetric-sweep-old.png)

It's much better now, although this model still does not work perfectly (missing triangles in the long, narrow part of the upper face):
![a sweep of a non-symmetric sketch; less borked](/blog/fornjot-0.6/non-symmetric-sweep-new.png)

Here's another model with triangles in the wrong places:
![star-shaped model with triangles in the wrong places](/blog/fornjot-0.6/triangulation-old.png)

And here's how that looks in the new version:
![star-shaped model with triangles in the right places](/blog/fornjot-0.6/triangulation-new.png)

These are just some examples. Many more bugs have been fixed. And of course, more bugs are still lurking. But Fornjot has become much more robust in this release, and will continue to do so.


### Small Improvements

There have been lots of smaller feature additions and bug fixes!

If you display a model, or export it for 3D printing, the model needs to be approximated. You can now specify the tolerance value, i.e. the maximum allowed deviation of the approximation from the actual model, as a command-line argument.

For example, running this from the Fornjot repository:

`cargo run -- --model=spacer --tolerance=0.1`

Results in this:
![a model that should be round has really jagged corners instead](/blog/fornjot-0.6/tolerance-low.png)


While this:

`cargo run -- --model=spacer --tolerance=0.001`

Results in this:
![a model that should be round is actually round](/blog/fornjot-0.6/tolerance-high.png)

This is just one example of many small features that were added.


### The Fornjot Ecosystem

Some people I've talked to have expressed interest in building their own applications on top of Fornjot, once it's ready. It might still take a while before Fornjot has matured enough for that to make sense, but in principle this is possible starting with this version.

While the previous version of Fornjot consisted mostly of one monolithic application, parts of that application have been extracted into libraries that can be used on their own.

[Check out my blog post on the Fornjot ecosystem](/blog/ecosystem/), if you're interested in this.


### Cleanup

Maybe the most important aspect of this release is the huge amount of cleanup work that has gone into it. Fornjot's code base is now much more capable, and much more ready to support the features we'll want to add.

While this has enabled some improvements in this release, this is mostly invisible. But it will pay dividends as more features are being added over the next releases.


### Contributors

Fornjot is an ambitious project, and it wouldn't be possible without contributors! Thanks you [@gzsombor](https://github.com/gzsombor), [@mxdamien](https://github.com/mxdamien), [@hendrikmaus](https://github.com/hendrikmaus), [@ObiWanRohan](https://github.com/ObiWanRohan), [@therealprof](https://github.com/therealprof), [@danieleades](https://github.com/danieleades), [@anwentec](https://github.com/anwentec), [@ozghimire](https://github.com/ozghimire), [@homersimpsons](https://github.com/homersimpsons), [@liubog2008](https://github.com/liubog2008), [@freylint](https://github.com/freylint), and [@connor-lennox](https://github.com/connor-lennox) for your help with this release!
