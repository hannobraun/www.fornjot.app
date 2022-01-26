+++
title = "Why Fornjot is Using Boundary Representation"
date  = 2022-01-26

# Workaround, since we can't tell a transparent section that we don't want it as
# part of the URL.
path = "blog/why-fornjot-is-using-boundary-representation"

[extra]
canonical = "https://hanno.braun-odw.eu/notes/why-fornjot-is-using-boundary-representation/"
+++

Fornjot uses a technique called boundary representation (or b-rep, for short). This is the traditional technique, used by many of the big commercial CAD packages, and it wasn't my first choice. In this note, I'd like to explain where I started with Fornjot, and why I ended up with b-rep.

What triggered my years-long work on CAD software was [Jamie Wong's article about ray marching and signed distance functions](http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/). I quickly came up with the idea of applying this technique to CAD, a topic that had started to interest me around the same time.


### Falling in love with signed distance functions

I really liked signed distance functions (also known as signed distance fields, implicit functions, function representation; or by their short-hands, SDF or f-rep), for a few reasons:

- You can [describe shapes](https://iquilezles.org/www/articles/distfunctions/distfunctions.htm), ranging from simple to complex, using pure math. That's neat.
- You can combine those shapes trivially using [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry) (CSG).
- To convert your shape to a triangle mesh (to interface with common slicers for 3D printing, for example), you just need a single algorithm.

As it turned out, I was not the only one to [have](http://implicitcad.org/) [that](https://github.com/deadsy/sdfx) [idea](https://libfive.com/), but that only encouraged me that the concept had merit. The following years (starting in 2018, I think) I was working on CAD experiments in the background, on and off.

I started getting more serious about my CAD work some time in 2021. I did a serious push to get *something* to work end-to-end, basically specifying a very simple model, then exporting that model to a 3MF file for 3D printing. I tried different approaches during that phase, some based on signed distance functions, some not.


### Falling out of love

In the end, after some pretty intense work trying to get signed distance functions to work for me, I came to the conclusion that they were just the wrong approach for my problem. I basically found that all the reasons I like them in the beginning were nullified:

- The math just isn't that neat, if you get into it a bit deeper. Various common operations [just don't result in a correct SDF](https://iquilezles.org/www/articles/interiordistance/interiordistance.htm). This is fine for many use cases, but I could see it become a problem for the engineering use cases of a CAD program, where accuracy is important.
- Constructive solid geometry is not enough. Yes, it's a critical part of most (all?) CAD software, but I want more than that. Modeling features like chamfering specific edges; or selecting a face, drawing a sketch on it, and making a hole in the shape of that sketch.
- Yes, you only need a single algorithm to create a triangle mesh, but the available algorithms either aren't that good (in terms of how well the generated triangles match the original geometry), or crazy complicated. Even the complicated ones [have weaknesses](https://github.com/curv3d/curv/blob/bdff8dc6c046ad157f6e088e37285a5113581aa2/ideas/v-rep/To_Mesh.rst).

In the end, I spent a lot of effort and never got a great result. Others have had [better success](https://libfive.com/), but that doesn't change the fact that with signed distance functions, advanced CAD modeling features are a complete unknown. As far as I can tell, no one has ever created an SDF-based CAD program with more than just basic features. For all I know, it's impossible.


### A new way

All that ended when I created a new prototype based on boundary representation (b-rep). Boundary representation defines solid models by their boundaries, i.e. the vertices, edges, and faces that make up the border between model and not model.

Suddenly things started clicking for me. It's the version of Fornjot that I'm still working on, and that I intend to take all the way to a useful piece of CAD software (and, if I can, beyond). But why am I betting Fornjot's future on boundary representation? I think there are three main reasons:

- It's the traditional approach. Even though I don't understand it completely (yet), I know all the CAD modeling features that I want to have are possible. Because others have done it before.
- The things we care about in a CAD model (vertices, edges, and faces) are directly represented, so selecting and manipulating them is straight-forward. After working with a more clever and indirect representation for so long, that is very refreshing.
- It's not all-or-nothing. There isn't a single algorithm to make it work. I can implement things more piecemeal, get a good result for the use case I care about right now, and make it more general over time.

Getting Fornjot to a point where it is really useful as a CAD program will take a long time, and a huge amount of work. But I'm confident now that I've chosen the right path to at least make that practical.


### A third approach

For the sake of completeness, I should note that I'm aware of a third approach: Generating triangle meshes directly from your primitives and operate on those triangle meshes to manipulate and combine them into more complex models.

I haven't studied this approach deeply, but I don't want to go down that route. First, getting those triangle mesh algorithms right is probably complicated. Second, it's unclear to me how fast those algorithms can be. Third, selecting a feature of the model with the intent of manipulating it is probably complicated to support (a round edge is represented by many small triangle edges, for example, so knowing what to select when the user clicks there is unclear).

We'll have to see if I made the right choice. But I can already say, for the first time since starting my work on CAD software, I feel really confident that I'm on a good path to create something really useful. Not just another experiment.
