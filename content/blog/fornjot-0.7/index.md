+++
title = "Fornjot 0.7"
date = "2022-07-07"
+++

[Fornjot](https://www.fornjot.app/) is an early-stage project with the goal to create a next-generation, code-first CAD application. This is the announcement for Fornjot's new release, version 0.7.0.

**Fornjot is still at an early stage and far from being useful as a general-purpose CAD tool.** This release should be seen as a preview for anyone who's interested in following Fornjot's development.

This release features improvements to how models are defined, how they can be viewed and exported, massive cleanups of the CAD kernel, and some improvements to the ecosystem libraries.

This release announcement provides a high-level summary of those changes. For more details, please refer to the [changelog](https://github.com/hannobraun/Fornjot/blob/main/CHANGELOG.md). For pre-compiled binaries, check out the [release on GitHub](https://github.com/hannobraun/Fornjot/releases/tag/v0.7.0). Please report bugs to any of the usual [community channels](/community).


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

To help make the project stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Modeling Improvements

Fornjot is a code-first CAD application, meaning CAD models are defined using a programming language. Right now, only Rust is supported, but in the future I hope that support for more languages will be available.

This release brings with it some great improvements for defining CAD models! Here's how a model with three parameters (including default values for each) looked in version 0.6:

``` rust
#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let x: f64 = args.get("x").map(|arg| arg.parse().unwrap()).unwrap_or(3.0);
    let y: f64 = args.get("y").map(|arg| arg.parse().unwrap()).unwrap_or(2.0);
    let z: f64 = args.get("z").map(|arg| arg.parse().unwrap()).unwrap_or(1.0);

    // model code goes here
}
```

The same model can now be defined like this:

``` rust
#[fj::model]
pub fn model(
    #[value(default = 3.0)] x: f64,
    #[value(default = 2.0)] y: f64,
    #[value(default = 1.0)] z: f64,
) -> fj::Shape {
    // model code goes here
}
```

Much better, isn't it?

Models can also specify minimum and maximum values for each parameter:

``` rust
#[fj::model]
pub fn model(
    #[value(default = 3.0, min = 2.5)] x: f64,
    #[value(default = 2.0, max = 2.5)] y: f64,
    #[value(default = 1.0, min = 0.5, max = 1.5)] z: f64,
) -> fj::Shape {
    // model code goes here
}
```

And these `min`/`max` values can even refer to other parameters:

``` rust
#[fj::model]
pub fn model(
    #[value(default = 3.0, min = y)] x: f64,
    #[value(default = 2.0, max = x)] y: f64,
    #[value(default = 1.0, min = y / 2.0, max = x / 2.0)] z: f64,
) -> fj::Shape {
    // model code goes here
}
```

---

One thing that always seems to be error-prone are angles. Previously, Fornjot would let you write this:

``` rust
// Rotate 180 degrees around the z-axis
let rotated = my_shape.rotate([0., 0., 1.], 180.);
```

But would this have been correct? No, actually not. Fornjot internally handles its angles in radians, not degrees.

This is no longer a problem. Now you would write the same code like this:

``` rust
// Rotate 180 degrees around the z-axis
let rotated = my_shape.rotate([0., 0., 1.], fj::Angle::from_deg(180.));
```

Or you can specify your angle in radians:

``` rust
// Rotate 180 degrees around the z-axis
let rotated = my_shape.rotate([0., 0., 1.], fj::Angle::from_rad(2. * PI));
```

Or even revolutions:

``` rust
// Rotate 180 degrees around the z-axis
let rotated = my_shape.rotate([0., 0., 1.], fj::Angle::from_rev(0.5));
```

All of these features were implemented by contributor [@gabsi26](https://github.com/gabsi26), with improvements by [@T0mstone](https://github.com/T0mstone). Thank you!


### Camera Improvements

Some time ago, I implemented an advanced system for controlling the camera in the 3D view, meant to solve some problems I had encountered in other CAD programs.

The most annoying one, to me, is when you're zoomed in to look at a detail of your model, and need to adjust your viewpoint a tiny bit. But when you try to do that, the detail you were looking at is gone from your view, because the model rotates around some far-away point.

A related issue appears when you're moving the model, but because you're zoomed in too closely, it moves way too fast. Or you are zoomed way out, and it moves really slowly.

In Fornjot, there is the concept of the focus point, which is the point on the model that the mouse cursor is currently hovering over. If you rotate the model, it will rotate around the focus point. And if you move it, the focus point will stay under the cursor.

Unfortunately, that was a case where my ambition outpaced my ability to put time into that specific area, and the system didn't work very well. Until very recently, I was contemplating to rip out all that code and replace it with a simpler solution that didn't try to fix those problems.

No more! The camera system works much, much better now.

<video controls width="600">
    <source src="fornjot-camera.webm" type="video/webm" />

    You shouldn't be seeing this. There should be a video instead. Sorry!
</video>

(Sorry for those weird green artifacts at the beginning of the video. They appeared when I encoded the edited video. When it comes to video editing, I barely know what I'm doing.)

This work was also completely driven by contributors. Thank you, [@chrisprice](https://github.com/chrisprice) and [@jeevcat](https://github.com/jeevcat)!


### STL Export

Fornjot was already capable of exporting models to [3MF](https://en.wikipedia.org/wiki/3D_Manufacturing_Format), the up-and-coming file format for 3D printing. For the traditionalists, we now also support exporting to [STL](https://en.wikipedia.org/wiki/STL_(file_format)).

Just specify the file extension when calling `fj-app` on the command line:

``` bash
fj-app --model=cuboid --export cuboid.stl # export to STL
fj-app --model=cuboid --export cuboid.3mf # export to 3MF
```

This has been implemented by contributor [@chrisprice](https://github.com/chrisprice). Thank you!


### Kernel

A lot of effort went into cleaning up Fornjot's CAD kernel! As Fornjot is slowly edging closer towards support for [boolean operations](https://en.wikipedia.org/wiki/Constructive_solid_geometry), architectural problems show themselves that need to be fixed.

Some of those fixes have been overly complex, as it turned out, and provided ample opportunity for simplifications. Many of those opportunities have been identified and taken advantage of, and the kernel now does similar things as it did before, but with much less code.

This work is very important, as it paves the way for all the CAD feature work going forward, but this release, it didn't lead to any outwardly visible changes. I figured I'd still call it out, as that's basically all I've been doing in recent weeks (besides pressing the "Merge" button for the awesome contributors that did the real work 😁).


### Ecosystem

Fornjot is not just a CAD application. The components that make up this application are released separately, and can be used by other projects that would like to augment Fornjot, or use parts of it for something completely different.

This release, there have been some additions and cleanups of `fj-math`, Fornjot's math library, and `fj-operations`, which is the glue that binds `fj-kernel` to the operations that Fornjot models have access to. These improvements have largely been driven by the changes in `fj-kernel`.

There is also a new library in the Fornjot ecosystem, `fj-window`, which has been extracted from `fj-viewer`. This leaves `fj-viewer` without a dependency on any windowing library, meaning alternative applications that want to use something else than [winit](https://crates.io/crates/winit) (the library that Fornjot uses for this), can still use `fj-viewer` to display models.


### Smaller Improvements

In addition to all this, there have been many smaller improvements. Two I'd like to call out are a memory leak in `fj::Sketch` that has been fixed (thank you, [@kamirr](https://github.com/kamirr)!) and improved usability of `fj-app`'s `--parameters` argument (thank you, [@A-Walrus](https://github.com/A-Walrus)!).


### Contributors

Fornjot is an ambitious project, and it wouldn't be possible without contributors! I've already called out a few high-profile contributions in this announcement, but more have helped.

Thank you, [@connor-lennox](https://github.com/connor-lennox), [@chrisprice](https://github.com/chrisprice), [@freylint](https://github.com/freylint), [@gabsi26](https://github.com/gabsi26), [@T0mstone](https://github.com/T0mstone), [@kamirr](https://github.com/kamirr), [@A-Walrus](https://github.com/A-Walrus), [@eLVas](https://github.com/eLVas), and [@jeevcat](https://github.com/jeevcat)!
