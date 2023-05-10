+++
title = "Roadmap"
+++

This page presents the plans for the further development of Fornjot. Fornjot doesn't have a linear roadmap with scheduled milestones. Keeping to something like that is difficult, even under the best circumstances. And as an open source project, Fornjot's development is driven by the priorities of those willing to contribute.

Instead, the milestones on this page define different focus areas for the project, that make sense as the next steps, and are ready to be worked on. The purpose of this roadmap is to inform users and sponsors about where things are going, as well as show contributors what is considered in scope right now.

**This roadmap is not exhaustive!** If you are interested in something that is not listed here, please [get in touch](/community).


### Current State

Fornjot is an early-stage project. We have provided an end-to-end experience for a while, meaning you can create a CAD model and export that to a 3MF or STL file for 3D printing. However, all aspects of that experience are unfinished.

Fornjot's functionality is very basic, and essential CAD modeling features are still missing. The current release, while functional, should only be considered a preview. Don't expect to be able to use it for anything but the most basic models.


### Immediate Priority: Boolean Operations

Most of the development effort currently goes into the implementation of boolean operations, creating the union, difference, or intersection of two shapes. This is also known as [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG).

The plan is to first focus on providing a stable experience for a subset of shapes, namely those with straight edges and flat faces. For more information, please refer to [the blog post introducing this milestone](/blog/straight-edges-flat-faces-simple-sketches-full-csg/), or [the milestone description on GitHub](https://github.com/hannobraun/Fornjot/milestone/1).


### Actionable Milestones

In addition to the immediate priority presented above, there are other milestones planned, that could be worked on given sufficient development resources. Contributions in these areas would be especially welcome. Once the immediate priority has been addressed, those are good candidates to replace it as the main focus.

#### Basic Usability

Not only are Fornjot's CAD modeling features severely limited, the user experience is still very rough. The subject of this milestone is to make Fornjot more accessible, by improving the app itself, as well as providing user-facing documentation.

For more information, see [the milestone description on GitHub](https://github.com/hannobraun/Fornjot/milestone/2).

#### Initial Web Support

One use case that Fornjot should support, is embedding CAD models into websites. Not just as a simple 3D model, but with the ability to change the model parameters, and export it to external files.

For more information, see [the milestone description on GitHub](https://github.com/hannobraun/Fornjot/milestone/3).

#### Modeling Convenience

Between modeling features and general UI usability sits this milestone: UI issues that directly relate to modeling. Things that make modeling more convenient, and help you inspect your model to help figure out what's going on.

For more information, see [the milestone description on GitHub](https://github.com/hannobraun/Fornjot/milestone/4).

#### 2D CAD

Laser cutting, water jet cutting, engraving... there are good reasons to create a 2D CAD model and export it to a specialized 2D file format.

For more information, see [the milestone description on GitHub](https://github.com/hannobraun/Fornjot/milestone/5).


### Longer-term Plans

The milestones presented above only include work that is actionable, meaning it is not blocked by something else that needs to be done first. Beyond that, there is lots more to be done:

- **More powerful CAD modeling:** Building on the above milestones to provide more modeling features. First, full support for round things, more flexible sketches, sketching on existing faces; later, hopefully full NURBS surfaces and much more. Nobody knows how far we'll be able to take things, but there are no plans to stop.
- **More use cases:** The current feature set is mostly geared towards 3D printing, but support for other use cases, like CNC machining, furniture making, or architecture, is desirable.
- **More modeling languages:** Right now, only Rust is supported for CAD modeling, mostly because it is the easiest to support. The long-term goal is to provide support for a selection of interoperable modeling languages.
- **Broad platform support:** Fornjot should be available on all major platforms, including browsers and mobile devices. Not only for editing models, but also for presenting them, for example embedding a configurable model into a website, or quickly using your phone to show what you're working on.
- **Special-purpose CAD:** Using Fornjot to build custom, special-purpose CAD applications; like specialized generators for gears or other configurable components.

This is not an exhaustive list. Please [check out the feature wishlist](https://github.com/hannobraun/Fornjot/discussions/146) for more, and feel free to add your own suggestions.

<div class="call-to-action">
    <p>
        <strong>Fornjot is an ambitious project.</strong> How much of that ambition can be realized depends on how sustainable the project can become. If you can, please consider helping out, by <a href="https://github.com/hannobraun/Fornjot">contributing to the project</a>, or by <a href="https://github.com/sponsors/hannobraun">becoming a sponsor</a>.
    </p>
</div>
