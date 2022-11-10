+++
title = "Progress Report - 2022-11"
date = 2022-11-10

[extra]
subtitle = "An overview over the progress made since roughly July 2022."
+++

Hey folks!

**Fornjot** is an **early-stage project** to create a **next-generation, code-first CAD application**. In this progress report I present the changes that happened over the last few months. This is a new format on this blog, and it replaces the old release announcements that I used to write. This progress report summarizes the changes that happened since [the release of Fornjot 0.7](/blog/fornjot-0.7/). *(We have since switched to a weekly release schedule, and the new weekly release announcements have, by necessity, a very different format.)*

**It's still early days for Fornjot!** It is limited and immature. What's currently there should be seen as a preview of what's to come, not *yet* as a useful tool.


### Modeling

Let's start with the core of what any CAD program is about: modeling! Fornjot is code-first, meaning CAD models are defined using a programming language. Currently, only Rust is supported for that.

Here's what a simple Fornjot model looks like:

``` rust
use fj::syntax::*;

#[fj::model]
pub fn model(
    #[param(default = 1.0, min = inner * 1.01)] outer: f64,
    #[param(default = 0.5, max = outer * 0.99)] inner: f64,
    #[param(default = 1.0)] height: f64,
) -> fj::Shape {
    let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
    let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}
```

This example displays two tweaks that have been made since version 0.7:

- Parameters are now annotated with `#[param]`, which is more descriptive than the old `#[value]`.
- `fj::Circle` is no longer its own shape and has been integrated with `fj::Sketch`. This is only an early peek of what's to come, as it's still not possible to create sketches with both straight lines and arcs.

Here's the 3D model that the above code creates:

![3D model of a cylinder with a circular hole along its height](/blog/progress-report-2022-11/spacer.png)

There have also been other improvements related to modeling:

- Group and transform operations now also work on 2D shapes.
- The `model` function shown above is now type-checked (contributed by [@Michael-F-Bryan]). Less opportunity for accidentally producing weird errors!
- Fornjot models are technically Rust libraries right now, and you had to specify a very specific `crate-type` in the model's `Cargo.toml` file. This is no longer necessary.
- The Fornjot app will look for the model's `target/` directory (where Cargo, the Rust build tool, puts its output) where it's actually supposed to be, and no longer blindly expects it in the model directory (contributed by [@Michael-F-Bryan]). This means you can have models in Cargo workspaces.
- The app now detects, if a model is using a different version of Fornjot, showing errors or warnings as appropriate (contributed by [@zthompson47] and others)
- And many other small tweaks and improvements!


### User Interface

Fornjot's GUI (graphical user interface) has made big strides! In version 0.7 (the last version that I published a progress report kinda like this for, see above), we only had an extremely basic GUI that didn't allow you to do a whole lot. Since then, we have integrated a new GUI based on [egui](https://github.com/emilk/egui) (originally by [@follower], with many follow-up improvements by [@devanlooches], [@connor-lennox], [@erenoku], and others).

Here's what the GUI looked like in version 0.7:

![Screenshot of the Fornjot app from version 0.7; nothing that's recognizable as a modern GUI is visible](/blog/progress-report-2022-11/gui-before.png)

And this is what the GUI looks like right now (development version, shortly after version 0.22.0):

![Screenshot of the current Fornjot app; shows a GUI with checkboxes and labels for some debug options, and a UI element displaying status messages](/blog/progress-report-2022-11/gui-after.png)

You can now start the app without any arguments or configuration that specify a model, and you'll be prompted to select one (contributed by [@erenoku]):

![Screenshot of the current Fornjot app; no model is being displayed, and there's a button in the center of the window that prompts the user to pick a model](/blog/progress-report-2022-11/gui-no-model.png)

And there's more:

- The app always starts immediately (contributed by [@payload] and others). No more wondering if something went wrong, when you're running for the first time and it takes a moment to compile the model.
- Some weird crashes have been fixed, informative and actionable error messages being shown instead (contributed by [@ArshErgon] and others).
- The app now has a `--version` command-line argument, which intelligently displays the current version and whether it's a release or not (contributed by [@Michael-F-Bryan]).
- Zooming behavior has been simplified/fixed (contributed by [@jeevcat]). It previously was a bit overly ambitious in what it tried to do and didn't work that great.
- The graphics backend should be more robust, working with more combinations of operating systems and graphics hardware (contributed by [@hekno25] and others)
- Anti-aliasing has been implemented, making models look nicer.
- And many other small improvements!

Fornjot still has some ways to go before it provides a truly user-friendly experience, but we're slowly getting there.


### Under the Hood

Despite all the improvements mentioned above, most work by far has gone into technical foundations that don't (yet) have a big impact on the user experience. The primary focus remains the implementation of [boolean operations](https://en.wikipedia.org/wiki/Constructive_solid_geometry), which is an essential feature that will help raise Fornjot from its current preview state to a level of basic usefulness.

Unfortunately there's more to it than implementing a few algorithms. To make implementing those algorithms practical, and the results of these algorithms reliable, the data structures of the CAD kernel need to be at a certain level of robustness and maturity.

The whole process of working on boolean operations has been defined by a long series of bugs and limitations being discovered, each requiring changes to the data structures to fix them properly. When that happens, the code that generates and manipulates those data structures is affected, often requiring deep architectural changes.

We're still figuring out how to structure a robust CAD kernel. It's a slow process, but things are moving in the right direction.

Another notable improvement is the new API that is used to communicate between the Fornjot app and the models it loads (contributed by [@Michael-F-Bryan]). This will enable many useful features in the future, like being able to manipulate model parameters from the GUI; or models that export multiple shapes, assemblies, and other kinds of artifacts.


### Conclusion

Fornjot is making slow but steady forward progress on its way from what is essentially a preview, to a basic but useful, and eventually a fully productive, CAD application.

Citing from the [home page](/), the goal of this project is to create a CAD application that:

- uses a **code-first** approach;
- is **open source**;
- is broadly available, with **support for all major platforms**;
- is based on a **new b-rep CAD kernel**, written in Rust;
- provides a **well-rounded feature set**;
- and support for **different modeling languages**.

**If this vision resonates with you, please consider supporting the project.** Join the [community](/community), [contribute](https://github.com/hannobraun/Fornjot/blob/main/CONTRIBUTING.md) to the project, or help out by [sponsoring me](/funding).


[@ArshErgon]: https://github.com/ArshErgon
[@connor-lennox]: https://github.com/connor-lennox
[@devanlooches]: https://github.com/devanlooches
[@erenoku]: https://github.com/erenoku
[@follower]: https://github.com/follower
[@hekno25]: https://github.com/hekno25
[@jeevcat]: https://github.com/jeevcat
[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan
[@payload]: https://github.com/payload
[@zthompson47]: https://github.com/zthompson47
