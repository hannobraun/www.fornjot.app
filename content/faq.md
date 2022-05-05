+++
title = "Frequently Asked Questions"

template = "faq.html"
+++

### What is Code-CAD?

CAD (computer-aided design) is a type of software for designing 3D objects. It's distinct from 3D modeling software in general, in that it's focused on technical applications. Like designing parts of a machine (or the whole machine).

Most CAD software provides a graphical user interface, which can be both complicated and limiting. Code-CAD is an alternative approach, in which CAD models are defined by code. You use a programming language to program your models, like you would software.

There's an [article about the advantages of Code-CAD](https://www.fornjot.app/blog/code-cad-advantages/) on the Fornjot blog.


### Is Fornjot similar to [OpenSCAD](https://openscad.org/)?

In principle, yes. In reality, Fornjot is at a much earlier stage of development, and still much more limited.

Under the hood, Fornjot is using a completely different approach (called [boundary representation or b-rep](https://www.fornjot.app/blog/why-fornjot-is-using-boundary-representation/)), that will allow it to support more advanced features at some point. Like sketching on existing faces, or built-in support for bevels/chamfers.


### How does Fornjot compare to [CadQuery](https://cadquery.readthedocs.io/en/latest/intro.html)?

CadQuery is a more feature-rich alternative to OpenSCAD that is based on a b-rep kernel. So asking what Fornjot hopes to add, given that CadQuery exists, is a fair question.

First, to be clear, at this point Fornjot is at a much earlier stage of development, and can't compete with CadQuery.

Long-term, I see the following opportunities to improve on CadQuery:

- Experimentation on the CAD kernel side. The only big open source b-rep kernel is [Open CASCADE](https://www.opencascade.com/open-cascade-technology/), and I think it's fair to take a fresh look at the problem.
- Provide broader platform support. Fornjot is written in Rust, which makes it practical to support all major desktop and mobile platforms, and also in the browser.
- Provide alternatives in regard to modeling languages. Rust is different enough from Python to provide an interesting alternative that is going to be better for some use cases. And Fornjot is architected to be language-agnostic, so long-term, it's possible to support a whole selection of interoperable modeling languages.


### Is Rust really the right language to define CAD models in?

Maybe not. It certainly isn't going to be ideal for most models.

So why is Fornjot using Rust then? Several reasons:

1. Fornjot itself is written in Rust, so supporting Rust as the modeling language is the easiest option. You gotta start somewhere.
2. To provide some choice. Most other Code-CAD applications either provide custom languages that are very limited, or use Python or JavaScript.

Fornjot is architected to be language-agnostic, and adding support for other languages should be relatively straight-forward, actually. Hopefully one day, there will be a selection of interoperable Fornjot frontends, so you can choose the best language for *your* model.


### Won't Rust's compile times prevent you from iterating on designs quickly?

Yes, compile times are a big, maybe the biggest, problem that Rust has. Earlier versions of Fornjot suffered big from that too.

In the current version, models only depend on a very thin Rust library to interact with Fornjot. They are then compiled into a dynamic library (soon to be a WebAssembly module, most likely) that is loaded into the Fornjot app at runtime. The application does all the heavy lifting, and suffers the compile-time cost of having all the code required for that.

It works pretty well for now. Current Fornjot models have compile-times of under 500ms (and that's not on the latest hardware). Not much effort has gone into optimizing this. It's conceivable that further optimizations (like using a different linker) could improve compile times drastically.

Still, interactive modifications to CAD models benefit from quick turnaround times, and Rust is not the best language to make that happen. Plus, Fornjot will likely be used for more and more complex models as time goes on, exacerbating the problem.

Which is why Fornjot is architected to be language-agnostic. Not sure where we'll end up (hopefully with a good selection of interoperable languages), but we have options.
