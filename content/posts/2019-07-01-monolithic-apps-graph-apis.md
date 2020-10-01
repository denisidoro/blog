+++
title = "Writing multi-module, monolithic apps with graph APIs"
[taxonomies]
tags = [ "dev", "clojure", "architecture" ]
[extra]
summary = "Multi-module, loosely-coupled, graph-oriented structure that is easy to split into microservices when needed."
+++

{{ medium_cover(
    id="0*sbUufEe2m6a0hwbN",
    caption="Honeycombs have many cells but they act together to fulfill the same purpose",
    unsplash="matthew_t_rader") }}

Last year, I talked about monolithic architecture that enables easy microservice splitting later.

After applying it for a large codebase, I’ve started using another approach: [graph APIs](https://zapier.com/engineering/graph-apis/).

### Recap

In a nutshell, I believe it’s fundamental to isolate different parts of an application into different modules, even if they are deployed as a single monolith at the end of the day.

Limiting the required cognitive load to navigate the code base is a must.

Writing scalable and readable code in the first shot is tough. If you have time and effort constraints (as I have for personal projects: I don’t want to spend hundreds of hours developing them), it’s even harder.

Unreadable, difficult to maintain code is a red flag. But if it works, is isolated and no one needs to look at it anymore, it’s less bad. Sure, one day it will stop working or will need to be updated, but until then people will have better context and know what works and what doesn’t. If it’s contained, it may even be plausible to rewrite it from scratch without huge costs.

### Past proposal

[In my past article](https://medium.com/@den.isidoro/microservice-size-and-splitting-dd9fc98a262e) I proposed this isolation via interfaces/objects/providers. I’ve been using it in [my personal financial platform](https://medium.com/@den.isidoro/using-grafana-for-personal-financial-management-ac0e4d0cb43) and it has been great!

I needed to change the provider for stock historical value and I only had to look at a single package, because there was no implementation details leakage to other parts of the code. Even if the codebase had 1MM LOC for all sorts of features, I knew that I (or anyone else) would only need to look at, and try to understand, ~50 LOC.

However, using these providers has some downsides:

#### It’s relatively too verbose

One module of mine only had pure functions, summing up ~20 LOC. However, in order to isolate it, I had to create a `[defprotocol](https://clojuredocs.org/clojure.core/defprotocol)`, a `[defrecord](https://clojuredocs.org/clojure.core/defrecord)`, implement some lifecycle and so on, even though it was stateless.

#### The interfaces don’t scale well

Having a `defprotocol` with `get-stocks` and `get-stock-history` is fine. But when you start pushing it towards `get-new-stocks-with-high-volatility`, things can get out of control.

#### Explicit dependencies

I wanted to get the historical value of all the investments available at my bank. One module was able to fetch my bank and get the name of the investments. Another one, which queries another API, was able to convert the name to an ID. Finally, a third module was able to get the history given an ID.

Where to put this composition? I wanted to make things transparent (the fact that my bank doesn’t expose the IDs shouldn’t leak to outer modules), so I made the bank module depend on the second one.

That worked. But in another flow the second module relied on the bank one. If we’re not careful enough, circular dependency may happen. This interdependency is difficult to reason about.

Another solution is to have a higher-level module that knows everyone else. A sort of [BFF/façade](https://alexandreesl.com/2016/03/18/backend-for-frontends-a-microservices-pattern/). That’s better, but it still needs to know that, e.g., the bank module doesn’t expose the IDs so and additional query is needed.

### New approach

What if we could have looser dependencies? What if no one needed to know how to surgically compose different modules to deliver a single response? With Graph APIs we can.

For [Clojure](https://clojure.org/), you can use the awesome [Pathom library](https://github.com/wilkerlucio/pathom), which will do all the heavy work for you.

To implement my example, the bank module could register a resolver like the one below (I’m stripping away some boilerplate for readability):

{{ gist(url="https://gist.github.com/denisidoro/e259a93747d73caeb584feb07820f925") }}

For the second module:

{{ gist(url="https://gist.github.com/denisidoro/c6803b999adae0fb703268b7bc5cd2be") }}

Finally, for the last module:

{{ gist(url="https://gist.github.com/denisidoro/0860d8729e08705a6785f296032494c9") }}

I now have a decentralized system. The graph parser knows by itself that, for each element in `:bank/investments`, it needs to go from `:investment/name` to `:investment/id` then `:investment/history` and how to resolve it.

As for the namespace organization, each module has the following structure:

{{ gist(url="https://gist.github.com/denisidoro/64a9111749ced25ac06a6ebba0e6c0a1") }}

* `logic/` contains pure functions
* `http/` and `db/` are one of many ports in the [hexagonal architecture](https://dzone.com/articles/hexagonal-architecture-what-is-it-and-how-does-it)
* `graph/` has the resolvers above
* when the resolvers get complicated, helper functions are extracted to `controllers/`, which orchestrate function calls
* `definition.clj` is a file I already had to bootstrap the server, but has now been extended to each module

### Module definition

An example of `definition.clj`:

{{ gist(url="https://gist.github.com/denisidoro/f44af804265590d3aa5e51689146076a/raw/5b16fe38fbbdfbf7d0d77ad27c5dcd67ac6a07cb/definition.clj") }}

* `:http` and `:bank` have immutable data that can be propagated to other components
* `:components` has some dependency-injection declarations
* `:entry-point` is the function to be called in case this module is to be used standalone or as the routing hub

Finally, I have a `[defmethod](https://clojuredocs.org/clojure.core/defmethod)` that’s able to reduce a config vector into a final, single config. For `:http`, for example, it merges all the bookmarks; for `:resolvers` it concatenates all vectors; for `:entry-point`, it keeps the last one.

In the end, to start the system, I call:

{{ gist(url="https://gist.github.com/denisidoro/5a1a4b9b7db59a54e1d9afc2c6f34001/raw/75b2f2e804988c0c49cb3560ef82eabcd39db059/boot.clj") }}

`(:entry-point rest-server.definition/config)`, for instance, is what spawns the server listening on port 80. If I want to use the system as a CLI, there's no need to spawn the server and curl it. I could simply swap the last config with `cli.definition/config`.

### Splitting into microservices/libraries

If for any reason the need arises, deploying a module as a microservice is trivial from a code perspective.

For the new microservice:

* clone the monolith into a different repository
* remove all undesired modules
* edit the `bootstrap-app!` call accordingly
* expose the resolvers it has (Pathom call this `index`)

For the original monolith:

* remove the module folder except `definition.clj`
* change `definition.clj` in such a way that the monolith is able to merge its `index` with the one in the new microservice (Pathom allows merging `index`es as well)

Steps for extracting a module to a library would be very similar (in case the module only has pure logic, for example).

### Downsides of the new approach

* for my particular case, having the graph find out the edge traversal is perfectly fine, but if I wanted maximum performance, calling the functions directly could be faster and less resource intensive (pending benchmark, though)
* since edges are loose, it’s difficult to `find usages` given a function or field. The IDE won't be able to know in what flows a resolver may be used

### Conclusion

This has given my code huge scalability at low costs and I’m fine with the downsides I could think of.

At the moment I have no open-source code to show, but if you’re really interested, contact me and we can work something out.

If you’ve liked the ideas highlighted here regarding graph APIs, please check [this talk](https://www.youtube.com/watch?v=r3zywlNflJI) from Pathom’s author.

{{ medium_first(id="writing-multi-module-monolithic-apps-with-graph-apis-1c095cdaccdf") }}
