+++
title = "Fornjot 0.48.0"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2023-12-15

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/release.html"

[extra]
version = "0.48.0"
subtitle = "It's been too long!"
+++

Shutting down the weekly release schedule was necessary. But waiting 6 months for putting out another release? Not quite as I thought it would go 😄

Time passed quickly, as it usually does, and now the year is basically over. But don't worry, that time was filled with lots of work on Fornjot. Let's take a look at some of the highlights!

#### Build/update operations and `ObjectSet`

Fornjot has APIs, called *operations*, to create and modify shapes. Two of the most important types of operations are `build` operations, which build shapes, and `update` operations, for low-level modifications of shapes.

These operations have been expanded and refined since the last release. Here's an example:

``` rust
Sketch::empty()
    .add_region(
        Region::polygon(
            [
                [-x / 2., -y / 2.],
                [x / 2., -y / 2.],
                [x / 2., y / 2.],
                [-x / 2., y / 2.],
            ],
            services,
        )
        .insert(services),
    )
```

This example combines both `build` operations, like `Sketch::empty` and `Region::polygon`, with `update` operations, like `add_region`, to create a rectangular sketch.

There's also the new `ObjectSet` data structure, which is used by all objects that reference multiple other objects of the same type. Combining it with `update` operations makes it easier to select the specific objects you want to modify:

``` rust
solid
    .update_shell(solid.shells().only(), |shell| {
        shell
            .update_face(shell.faces().first(), |face| {
                // Update the face here!
                todo!()
            })
            .insert(services)
    })
```

Here, we express that we want to update the *only* shell of `solid` (which will helpfully panic, if it has multiple shells, to tell us about our wrong assumption), and the *first* face of `shell`.

This is still a very basic way to select objects, and it becomes very tedious, or even impossible, to use in non-trivial scenarios. This is an area where further improvement is required.

#### Split and sweep

Sweeping is an operation that "sweeps" a 2D shape through space, to create a 3D shape (you can also sweep a 1D shape into a 2D shape, but that's more of an implementation detail). We've had this feature for a long time!

In this release, the `sweep` code has been cleaned up significantly, and this cleanup has enabled an important new capability: Where previously, the main use case was to sweep a sketch into a new shell, you can now take an existing shell and sweep one of its faces to extend the shell.

Combined with the new `split` operation, which can split a face into two, this can create interesting shapes that haven't been possible before. Here's an example:

``` rust
cube
    .update_shell(cube.shells().only(), |shell| {
        let face = shell.faces().first();
        let cycle = face.region().exterior();

        let line = [
            (cycle.half_edges().nth(0).unwrap(), [split_pos]),
            (cycle.half_edges().nth(2).unwrap(), [split_pos]),
        ];

        let (shell, [face, _]) = shell.split_face(face, line, services);

        shell
            .sweep_face_of_shell(face, [0., 0., -size / 2.], services)
            .insert(services)
    })
```

And here's the result:
![A cube, which had one of its faces split, then one of the resulting smaller faces swept.](/blog/release/0.48.0/split-and-sweep.png)

This is a big step forward in capability, but there's also much left to do:

- The new `split` operation is clunky and limited. Creating the line along which the face is split is awkward, and it can *only* be a line.
- What we *actually* want here, is to apply a sketch to a face, then extrude that sketch. This will happen, but it requires smarter operations, and better infrastructure to support them.
- Actually using the `split` operation suffers from the limited ways to select objects (as mentioned above).

Splitting faces is only a first step, a proof of concept. Right now, it enables models that haven't been possible before, and it paves the way for more advanced operations to be implemented in the future.

#### Holes

Cleaning up the `sweep` operations enabled another new feature: You can now create holes!

![A cuboid with two holes: a blind hole on the left, and a through hole on the right](/blog/release/0.48.0/holes.png)

Here's an example that creates a blind hole in the bottom face of a shell:

``` rust
shell.add_blind_hole(
    HoleLocation {
        face: bottom_face,
        position: [-offset, Scalar::ZERO].into(),
    },
    radius,
    [Scalar::ZERO, Scalar::ZERO, depth],
    services,
)
```

And this one creates a through hole, from a shell's bottom face to its top face:

``` rust
shell
    .add_through_hole(
        [
            HoleLocation {
                face: bottom_face,
                position: [offset, Scalar::ZERO].into(),
            },
            HoleLocation {
                face: top_face,
                position: [offset, Scalar::ZERO].into(),
            },
        ],
        radius,
        services,
    )
```

While this is pretty neat, it is still quite limited:

- The new `hole` operations are not very smart. You have to explicitly specify whether to create a blind or through hole.
- The hole entry and exit each need to be contained within a face. You can't create a hole where two faces meet, and you can't create a groove using a hole that only partially overlaps the shell.
- And I'm sure there are more bugs and limitations that haven't been discovered yet.

Again, like the face splitting, this is just a first step. Future iterations will be more powerful, robust, and flexible.

#### And much more!

This is only the tip of the iceberg! There are many more improvements. Some user-visible, but smaller than the ones presented above. Others under the hood, where they support the user-visible features.

Check out the list below, for a more complete overview.

#### What's next?

I've been focused on new features for a while, and this release is the culmination of that. But now it's time to turn inward. To lift some of the limitations of those features, and to add new and better features, we need better infrastructure.

The planning process for this has started. We'll have to see where it takes us, but [#2116](https://github.com/hannobraun/fornjot/issues/2116), [#2117](https://github.com/hannobraun/fornjot/issues/2117), and [#2118](https://github.com/hannobraun/fornjot/issues/2118), are probably what will keep me busy for a while.

However, this is not all there is to do! There are [open issue](https://github.com/hannobraun/fornjot/issues) and a [feature wishlist](https://github.com/hannobraun/fornjot/discussions/146) with many more work items, and **help is always appreciated**. So if you see anything there that appeals to you, or have an idea of your own, please feel free to jump in and help out!


### Sponsors

Fornjot is supported by [@MitchellHansen](https://github.com/MitchellHansen), [@webtrax-oz](https://github.com/webtrax-oz), [@seanjensengrey](https://github.com/seanjensengrey), [@reivilibre](https://github.com/reivilibre), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@martindederer](https://github.com/martindederer), [@bollian](https://github.com/bollian), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

Additional thanks go to [@jonnedelm](https://github.com/jonnedelm), [@refarb](https://github.com/refarb), and [@Retraze](https://github.com/Retraze), who also supported this release with their financial contribution!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### Library improvements

Improvements to Fornjot libraries.

#### `fj`

- Improve error output ([#2012])
- Set up logging in model handler ([#2013])

#### `fj-core`

- Make improvements to validation infrastructure ([#1907], [#1914], [#1942], [#2058], [#2061], [#2064], [#2065], [#2095], [#2096])
- Expand and clean up build and update operations ([#1912], [#1913], [#1934], [#1975], [#2029], [#2031], [#2032], [#2033], [#2119])
- Add `ObjectSet` to manage and access references to other objects ([#1915], [#2026], [#2027], [#2028], [#2036], [#2083], [#2084], [#2109])
- Clean up edge approximation code ([#1936], [#1953], [#1966], [#1996], [#2005], [#2008], [#2009], [#2010], [#2016], [#2017], [#2019], [#2020], [#2021], [#2041], [#2042], [#2043], [#2044], [#2046], [#2051], [#2054], [#2055], [#2057], [#2066], [#2067], [#2068], [#2074])
- Improve user-facing documentation ([#1938], [#2056], [#2081], [#2100], [#2101])
- Clean up representation of curves in object graph ([#1939], [#1950], [#1952], [#1982], [#1983], [#1997], [#1998], [#1999], [#2000], [#2059])
- Make various minor cleanups and additions ([#1940], [#1951], [#1981], [#2107], [#2113], [#2127], [#2135])
- Add `CurveBoundary` to represent boundaries on a curve ([#1941], [#1967], [#1990], [#1991], [#2037])
- Add `queries` API ([#1949], [#1974], [#2063], [#2077])
- Remove out-of-date note ([#1954]; thank you, [@A-Walrus]!)
- Validate shell orientations ([#1968]; thank you, [@A-Walrus]!)
- Expand reverse operation ([#1976], [#1979], [#1980])
- Make `IsInserted` more useful ([#1978])
- Add `operations::split` ([#2076], [#2088], [#2093], [#2094], [#2097], [#2115])
- Add `operations::replace` ([#2086], [#2089], [#2091], [#2092])
- Clean up sweep operations ([#2099], [#2103], [#2106], [#2110], [#2111], [#2114], [#2120], [#2137])
- Add operation to extend a shell by sweeping one of its faces ([#2121])
- Add operations to add holes to a shell ([#2128], [#2134], [#2136], [#2138])


#### `fj-viewer`

- Be more relaxed about what GPU configs to accept ([#2014])
- Fix initialization panic with GL backend ([#2015])
- Add debug logging, improve error messages, shuffle some render stuff around ([#2035])
- Try all adapters, if one fails to return a device ([#2038])


### Other changes

Improvements that are not associated with a specific Fornjot library.

- Update release procedure ([#1902])
- Update README ([#1903], [#1989], [#2050], [#2075])
- Update dependencies ([#1906], [#1911], [#1919], [#1933], [#1948], [#1965], [#1972], [#1987], [#1992], [#1993], [#1994], [#1995], [#2001], [#2002], [#2003], [#2006], [#2007], [#2018], [#2022], [#2030], [#2039], [#2048], [#2052], [#2053], [#2062], [#2073], [#2079], [#2080], [#2082], [#2085], [#2090], [#2102], [#2112], [#2122], [#2131], [#2139])
- Add automation for generating blog posts on sponsor updates ([#1920])
- Update contribution guide ([#1955]; thank you, [@TobiasJacob]!)
- Remove fj.toml file ([#1956]; thank you, [@TobiasJacob]!)
- Upgrade to Rust 1.74.1 ([#1988], [#2004], [#2024], [#2045], [#2104], [#2129])
- Add configuration for developing on NixOS ([#2105], [#2108])


[#1902]: https://github.com/hannobraun/fornjot/pull/1902
[#1903]: https://github.com/hannobraun/fornjot/pull/1903
[#1906]: https://github.com/hannobraun/fornjot/pull/1906
[#1907]: https://github.com/hannobraun/fornjot/pull/1907
[#1911]: https://github.com/hannobraun/fornjot/pull/1911
[#1912]: https://github.com/hannobraun/fornjot/pull/1912
[#1913]: https://github.com/hannobraun/fornjot/pull/1913
[#1914]: https://github.com/hannobraun/fornjot/pull/1914
[#1915]: https://github.com/hannobraun/fornjot/pull/1915
[#1919]: https://github.com/hannobraun/fornjot/pull/1919
[#1920]: https://github.com/hannobraun/fornjot/pull/1920
[#1933]: https://github.com/hannobraun/fornjot/pull/1933
[#1934]: https://github.com/hannobraun/fornjot/pull/1934
[#1936]: https://github.com/hannobraun/fornjot/pull/1936
[#1938]: https://github.com/hannobraun/fornjot/pull/1938
[#1939]: https://github.com/hannobraun/fornjot/pull/1939
[#1940]: https://github.com/hannobraun/fornjot/pull/1940
[#1941]: https://github.com/hannobraun/fornjot/pull/1941
[#1942]: https://github.com/hannobraun/fornjot/pull/1942
[#1948]: https://github.com/hannobraun/fornjot/pull/1948
[#1949]: https://github.com/hannobraun/fornjot/pull/1949
[#1950]: https://github.com/hannobraun/fornjot/pull/1950
[#1951]: https://github.com/hannobraun/fornjot/pull/1951
[#1952]: https://github.com/hannobraun/fornjot/pull/1952
[#1953]: https://github.com/hannobraun/fornjot/pull/1953
[#1954]: https://github.com/hannobraun/fornjot/pull/1954
[#1955]: https://github.com/hannobraun/fornjot/pull/1955
[#1956]: https://github.com/hannobraun/fornjot/pull/1956
[#1965]: https://github.com/hannobraun/fornjot/pull/1965
[#1966]: https://github.com/hannobraun/fornjot/pull/1966
[#1967]: https://github.com/hannobraun/fornjot/pull/1967
[#1968]: https://github.com/hannobraun/fornjot/pull/1968
[#1972]: https://github.com/hannobraun/fornjot/pull/1972
[#1974]: https://github.com/hannobraun/fornjot/pull/1974
[#1975]: https://github.com/hannobraun/fornjot/pull/1975
[#1976]: https://github.com/hannobraun/fornjot/pull/1976
[#1978]: https://github.com/hannobraun/fornjot/pull/1978
[#1979]: https://github.com/hannobraun/fornjot/pull/1979
[#1980]: https://github.com/hannobraun/fornjot/pull/1980
[#1981]: https://github.com/hannobraun/fornjot/pull/1981
[#1982]: https://github.com/hannobraun/fornjot/pull/1982
[#1983]: https://github.com/hannobraun/fornjot/pull/1983
[#1987]: https://github.com/hannobraun/fornjot/pull/1987
[#1988]: https://github.com/hannobraun/fornjot/pull/1988
[#1989]: https://github.com/hannobraun/fornjot/pull/1989
[#1990]: https://github.com/hannobraun/fornjot/pull/1990
[#1991]: https://github.com/hannobraun/fornjot/pull/1991
[#1992]: https://github.com/hannobraun/fornjot/pull/1992
[#1993]: https://github.com/hannobraun/fornjot/pull/1993
[#1994]: https://github.com/hannobraun/fornjot/pull/1994
[#1995]: https://github.com/hannobraun/fornjot/pull/1995
[#1996]: https://github.com/hannobraun/fornjot/pull/1996
[#1997]: https://github.com/hannobraun/fornjot/pull/1997
[#1998]: https://github.com/hannobraun/fornjot/pull/1998
[#1999]: https://github.com/hannobraun/fornjot/pull/1999
[#2000]: https://github.com/hannobraun/fornjot/pull/2000
[#2001]: https://github.com/hannobraun/fornjot/pull/2001
[#2002]: https://github.com/hannobraun/fornjot/pull/2002
[#2003]: https://github.com/hannobraun/fornjot/pull/2003
[#2004]: https://github.com/hannobraun/fornjot/pull/2004
[#2005]: https://github.com/hannobraun/fornjot/pull/2005
[#2006]: https://github.com/hannobraun/fornjot/pull/2006
[#2007]: https://github.com/hannobraun/fornjot/pull/2007
[#2008]: https://github.com/hannobraun/fornjot/pull/2008
[#2009]: https://github.com/hannobraun/fornjot/pull/2009
[#2010]: https://github.com/hannobraun/fornjot/pull/2010
[#2012]: https://github.com/hannobraun/fornjot/pull/2012
[#2013]: https://github.com/hannobraun/fornjot/pull/2013
[#2014]: https://github.com/hannobraun/fornjot/pull/2014
[#2015]: https://github.com/hannobraun/fornjot/pull/2015
[#2016]: https://github.com/hannobraun/fornjot/pull/2016
[#2017]: https://github.com/hannobraun/fornjot/pull/2017
[#2018]: https://github.com/hannobraun/fornjot/pull/2018
[#2019]: https://github.com/hannobraun/fornjot/pull/2019
[#2020]: https://github.com/hannobraun/fornjot/pull/2020
[#2021]: https://github.com/hannobraun/fornjot/pull/2021
[#2022]: https://github.com/hannobraun/fornjot/pull/2022
[#2024]: https://github.com/hannobraun/fornjot/pull/2024
[#2026]: https://github.com/hannobraun/fornjot/pull/2026
[#2027]: https://github.com/hannobraun/fornjot/pull/2027
[#2028]: https://github.com/hannobraun/fornjot/pull/2028
[#2029]: https://github.com/hannobraun/fornjot/pull/2029
[#2030]: https://github.com/hannobraun/fornjot/pull/2030
[#2031]: https://github.com/hannobraun/fornjot/pull/2031
[#2032]: https://github.com/hannobraun/fornjot/pull/2032
[#2033]: https://github.com/hannobraun/fornjot/pull/2033
[#2035]: https://github.com/hannobraun/fornjot/pull/2035
[#2036]: https://github.com/hannobraun/fornjot/pull/2036
[#2037]: https://github.com/hannobraun/fornjot/pull/2037
[#2038]: https://github.com/hannobraun/fornjot/pull/2038
[#2039]: https://github.com/hannobraun/fornjot/pull/2039
[#2041]: https://github.com/hannobraun/fornjot/pull/2041
[#2042]: https://github.com/hannobraun/fornjot/pull/2042
[#2043]: https://github.com/hannobraun/fornjot/pull/2043
[#2044]: https://github.com/hannobraun/fornjot/pull/2044
[#2045]: https://github.com/hannobraun/fornjot/pull/2045
[#2046]: https://github.com/hannobraun/fornjot/pull/2046
[#2048]: https://github.com/hannobraun/fornjot/pull/2048
[#2050]: https://github.com/hannobraun/fornjot/pull/2050
[#2051]: https://github.com/hannobraun/fornjot/pull/2051
[#2052]: https://github.com/hannobraun/fornjot/pull/2052
[#2053]: https://github.com/hannobraun/fornjot/pull/2053
[#2054]: https://github.com/hannobraun/fornjot/pull/2054
[#2055]: https://github.com/hannobraun/fornjot/pull/2055
[#2056]: https://github.com/hannobraun/fornjot/pull/2056
[#2057]: https://github.com/hannobraun/fornjot/pull/2057
[#2058]: https://github.com/hannobraun/fornjot/pull/2058
[#2059]: https://github.com/hannobraun/fornjot/pull/2059
[#2061]: https://github.com/hannobraun/fornjot/pull/2061
[#2062]: https://github.com/hannobraun/fornjot/pull/2062
[#2063]: https://github.com/hannobraun/fornjot/pull/2063
[#2064]: https://github.com/hannobraun/fornjot/pull/2064
[#2065]: https://github.com/hannobraun/fornjot/pull/2065
[#2066]: https://github.com/hannobraun/fornjot/pull/2066
[#2067]: https://github.com/hannobraun/fornjot/pull/2067
[#2068]: https://github.com/hannobraun/fornjot/pull/2068
[#2073]: https://github.com/hannobraun/fornjot/pull/2073
[#2074]: https://github.com/hannobraun/fornjot/pull/2074
[#2075]: https://github.com/hannobraun/fornjot/pull/2075
[#2076]: https://github.com/hannobraun/fornjot/pull/2076
[#2077]: https://github.com/hannobraun/fornjot/pull/2077
[#2079]: https://github.com/hannobraun/fornjot/pull/2079
[#2080]: https://github.com/hannobraun/fornjot/pull/2080
[#2081]: https://github.com/hannobraun/fornjot/pull/2081
[#2082]: https://github.com/hannobraun/fornjot/pull/2082
[#2083]: https://github.com/hannobraun/fornjot/pull/2083
[#2084]: https://github.com/hannobraun/fornjot/pull/2084
[#2085]: https://github.com/hannobraun/fornjot/pull/2085
[#2086]: https://github.com/hannobraun/fornjot/pull/2086
[#2088]: https://github.com/hannobraun/fornjot/pull/2088
[#2089]: https://github.com/hannobraun/fornjot/pull/2089
[#2090]: https://github.com/hannobraun/fornjot/pull/2090
[#2091]: https://github.com/hannobraun/fornjot/pull/2091
[#2092]: https://github.com/hannobraun/fornjot/pull/2092
[#2093]: https://github.com/hannobraun/fornjot/pull/2093
[#2094]: https://github.com/hannobraun/fornjot/pull/2094
[#2095]: https://github.com/hannobraun/fornjot/pull/2095
[#2096]: https://github.com/hannobraun/fornjot/pull/2096
[#2097]: https://github.com/hannobraun/fornjot/pull/2097
[#2099]: https://github.com/hannobraun/fornjot/pull/2099
[#2100]: https://github.com/hannobraun/fornjot/pull/2100
[#2101]: https://github.com/hannobraun/fornjot/pull/2101
[#2102]: https://github.com/hannobraun/fornjot/pull/2102
[#2103]: https://github.com/hannobraun/fornjot/pull/2103
[#2104]: https://github.com/hannobraun/fornjot/pull/2104
[#2105]: https://github.com/hannobraun/fornjot/pull/2105
[#2106]: https://github.com/hannobraun/fornjot/pull/2106
[#2107]: https://github.com/hannobraun/fornjot/pull/2107
[#2108]: https://github.com/hannobraun/fornjot/pull/2108
[#2109]: https://github.com/hannobraun/fornjot/pull/2109
[#2110]: https://github.com/hannobraun/fornjot/pull/2110
[#2111]: https://github.com/hannobraun/fornjot/pull/2111
[#2112]: https://github.com/hannobraun/fornjot/pull/2112
[#2113]: https://github.com/hannobraun/fornjot/pull/2113
[#2114]: https://github.com/hannobraun/fornjot/pull/2114
[#2115]: https://github.com/hannobraun/fornjot/pull/2115
[#2119]: https://github.com/hannobraun/fornjot/pull/2119
[#2120]: https://github.com/hannobraun/fornjot/pull/2120
[#2121]: https://github.com/hannobraun/fornjot/pull/2121
[#2122]: https://github.com/hannobraun/fornjot/pull/2122
[#2127]: https://github.com/hannobraun/fornjot/pull/2127
[#2128]: https://github.com/hannobraun/fornjot/pull/2128
[#2129]: https://github.com/hannobraun/fornjot/pull/2129
[#2131]: https://github.com/hannobraun/fornjot/pull/2131
[#2134]: https://github.com/hannobraun/fornjot/pull/2134
[#2135]: https://github.com/hannobraun/fornjot/pull/2135
[#2136]: https://github.com/hannobraun/fornjot/pull/2136
[#2137]: https://github.com/hannobraun/fornjot/pull/2137
[#2138]: https://github.com/hannobraun/fornjot/pull/2138
[#2139]: https://github.com/hannobraun/fornjot/pull/2139

[@A-Walrus]: https://github.com/A-Walrus
[@TobiasJacob]: https://github.com/TobiasJacob
