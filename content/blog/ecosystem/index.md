+++
title = "The Fornjot Ecosystem"
date  = 2022-05-04
+++

The goal of Fornjot is not only to create a CAD application, but also to encourage an ecosystem to develop around it. I want to enable others to build on top of the components that make Fornjot work, so they can develop their own solutions more easily.

This has recently become possible. The previously monolithic Fornjot application has been broken up into a number of smaller, self-contained libraries. This work has been tracked in issue [#141].


### Why is enabling an ecosystem important?

By opening up Fornjot's components for wider use, the project can become useful to a larger number of people, both directly and indirectly. Directly, to developers who want to build their own CAD-related software; indirectly, to the people who end up using the software built by those developers.

A broader user base means the project could attract more contributions, which would be great for Fornjot's long-term health. And contributions to Fornjot's components would lead to improvements that directly benefit users of the Fornjot application itself.

Aside from encouraging other projects to build on top of Fornjot, this also makes extensions to Fornjot itself easier. New features can be prototyped by outside contributors, then later be merged into Fornjot; or kept as standalone extensions, as deemed appropriate.

And all of this is not purely theoretical. Several people I've talked to have expressed the hope that Fornjot will become a suitable base for them to build their own projects on top of.


### Won't this be a distraction?

It could be. Opening up Fornjot like that could dilute the project's focus, and ultimately distract from its main objective of building a next-generation Code-CAD application. To some extent, that doesn't matter. So what, if contributors who otherwise wouldn't have been involved in the first place, end up doing work that doesn't directly benefit that goal?

But it can turn into a problem, if work on the Fornjot ecosystem diverts existing resources, like my own development time. I'm aware of that danger, and will do my best to not let myself get distracted. It will be a matter of focusing on Fornjot's core goals, and drawing clear boundaries regarding what is in scope for the project, and what isn't.

My hope is, that any additional work will be a worthwhile investment. Publishing parts of Fornjot as separate libraries and making sure they are documented isn't taking that much additional time, but it could pay off for all the reasons mentioned above.

And in the end, this ecosystem effort could result in the opposite: Enabling external developers to build on top of Fornjot means more work can happen without my involvement. And if specific use cases can be addressed by such work, it becomes easier to declare those uses cases as out of scope for the core project.


### An overview

But enough talk about why this is or isn't a good idea. Let's take a look at each of the new libraries, and what they do:

- `fj-math` provides math primitives for the Fornjot ecosystem. It is itself built on top of [nalgebra] and [Parry], but its interface is tailor-made for Fornjot, simplifying the math code in the rest of the ecosystem.
- `fj` provides the API for defining Fornjot models. It is used in model code, and is thus the only of the libraries that is directly relevant to end users.
- `fj-interop` defines various types that are used for interoperation between other Fornjot components, without requiring those other components to depend on each other.
- `fj-kernel` is Fornjot's CAD kernel, the core part of Fornjot. It define geometric and topological primitives, as well as algorithms that operate on those.
- `fj-operations` serves as the link between `fj` and `fj-kernel`. It uses `fj-kernel` to implement the CAD operations that models define via the `fj` crate.
- `fj-export` can export Fornjot models to external data formats, like 3MF, and hopefully many more in the future.
- `fj-host` loads Fornjot models, and watches them for changes. Fornjot models are basically plugins that are loaded into the Fornjot app, and `fj-host` serves as the host for those model plugins.
- `fj-viewer` provides the user interface for Fornjot: it handles input, controls the camera, and can render Fornjot models.
- `fj-app` is the only application in this list. It puts all of the aforementioned libraries together to provide a CAD application to end users.


#### Interested?

Are you interested in using parts of Fornjot to build your own application? I'd love to hear about that! Please [get in touch](/community)!


[#141]: https://github.com/hannobraun/Fornjot/issues/141
[nalgebra]: https://nalgebra.org/
[Parry]: https://www.parry.rs/