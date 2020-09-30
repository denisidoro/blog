+++
title = "On microservice splitting and code refactoring"
[taxonomies]
tags = [ "dev", "architecture" ]
[extra]
summary = "What is the ideal microservice size? What if we need to split it later? This article proposes tips for making this process easier."
+++

{{ medium_cover(
    id="0*YzywaYKyaA6w-HcY",
    caption="Photo"
    unsplash="leliejens") }}

Let’s say that you work for a company that offers a platform for personal finance management. You were requested to add a feature whose input from the customer is a list of company × amount of stocks × investment date and the output is the customer’s total balance for a given point in time.

How big or small should the microservice(s) be?

At one end of the spectrum, you have a single [monolith](https://www.thoughtworks.com/insights/blog/monoliths-are-bad-design-and-you-know-it), with scalability issues.

At the other end, you have one service for:

* stock value scraping (`stockScraper`)
* company info store (`companyStore`)
* stock value history store (`stockHistory`)
* customer input stock store (`stockStore`)

This approach gives us a nice separation of concerns and makes it easy to develop and scale each part independently. However, it takes more time to have an [MVP](https://en.wikipedia.org/wiki/Minimum_viable_product) this way, and [time to market](https://en.wikipedia.org/wiki/Time_to_market) may be crucial. And it costs more, as well, because of overheads (such as multiple JVMs, databases, message streaming, etc).

Given your resource constraints, maybe it makes sense to start as a single service (`newFeature`). But what if you want or need to split it later?

Here are my tips for designing a service easy to refactor, at a low cost during development.

### 0\. Divide your application into layers

You can choose from [onion](https://dzone.com/articles/onion-architecture-is-interesting), [layered](https://dzone.com/articles/layered-architecture-is-good), and [hexagonal](https://dzone.com/articles/hexagonal-architecture-is-powerful) architectures, among others. As always, prioritize immutability, state management, pure logic extraction and all the other best practices.

### 1\. Don’t be afraid of using many namespaces

Don’t keep all namespaces (packages, files) centralized in a root one. Instead of `newFeature.logic.stockScraper`, use `stockScraper.logic`. Have you created a pure function that may be useful for both `stockScraper` and `stockStore`? Put it inside `stock.logic`. Have you written a function for linear interpolation? This math concept isn't restricted to a specific feature so it doesn't have to be inside `newFeature.logic.math`, for example. Put it inside `common.math`. This way you aren't tempted to put `linearInterpolate` and `stockSum` in the same file.

When the time comes for service splitting, you can deploy the `common` and `stock` folders as libraries and start the `stockScraper` service by cut and pasting the respective folder. Much easier than traversing all the codebase later to extract everything you need!

### 2\. Import non-common namespaces with care

Does `stockHistory` have to import `stockScraper` a lot? Can't it be minimized? Remember that when splitting this will translate into HTTP calls or message streaming. Try to at least isolate this dependency into a single namespace, which will be the precursor of the API between the two. This step is the most subjective, because it can quickly introduce overheads in the codebase for an MVP. You must consider the trade-offs.

### 3\. Isolate implementation details into higher level components

Let’s now implement the handler for the endpoint that returns information about all the companies a customer has stocks from. One common approach is the following (we’re exposing components via dependency injection and middlewares):

{{ gist(url="https://gist.github.com/denisidoro/d77a550d25bd6a77d35b5b57d40cf4b8" )}}

After splitting our service, not all data will be available via the `db` argument, of course. We'll need to make some HTTP calls. `customerCompaniesHandler` will have to get an HTTP component and pass it to `getCustomerCompanies`, which will propagate it to the domain-specific helper functions and so on...

But why does the handler have to know this in the first place? We have divided our application into layers and, more importantly, we have already separated our code into `companyStore`, `stockStore` and other namespaces. Yet, we clearly have an implementation detail leak such that all our higher level, integration API code has to know about low level dependency management. These layers should be agnostic to where we store the `company` entity...

Ideally, our handler should only know that it has to fetch data that depends on the `stock` and `company` entities. What if we created a component that abstracts this away, per entity? Let's call one of them as `CompanyRepository` (you can come up with the name you like):

{{ gist(url="https://gist.github.com/denisidoro/335efcab949ddaa3f40c78a9fec4666e" )}}

You can skip the interface/protocol definition if you’re not into that. The important thing is to avoid leaking lower level components.

After adding this component to our dependency graph, our code would become:

{{ gist(url="https://gist.github.com/denisidoro/36dd706464e5cbb223fd6debdfba24a9") }}

For the service split, we could create:

{{ gist(url="https://gist.github.com/denisidoro/36dd706464e5cbb223fd6debdfba24a9") }}

The only thing we need to do is update the dependency graph! All the rest of the code remains the same.

As a plus, we can move `CompanyDbRepository` to `companyStore`. Code reuse!

### Wrap up

Suppose you’re absolutely certain that you won’t have to split your recently created microservice or that the process will be easy enough such that you won’t curse your past self. Then, you may not see much value in my article.

However, this isn’t just about spatial organization or isolation. It’s about making domains arguably easier to reason about. Establishing higher level boundaries reduces the cognitive requirement to fiddle with a codebase. You only need to traverse the code as deep as required. I find it easier to handle new abstractions than reaching a mental stack overflow when browsing lines of code.

And even if you decide to keep a single service, it’s much easier for a newcomer to improve `stockParser`, for example, if he or she only has to read a root namespace and not the whole microservice code.

Anyway, no one is able to perfectly measure the ideal microservice size, because that varies in time, the company size, the feature success and many other factors. So it’s nice to be flexible.

### Clojure sidenotes

* [Duct](https://github.com/duct-framework/duct) follows this pattern and calls it boundary.
* You can skip the component creation altogether and define boundaries with resolvers using a library such as [Pathom](https://github.com/wilkerlucio/pathom): you define a graph edge that enables you to go from a `customerId` to a `company`, for example.

### Front-end sidenote

Even though I focused on the back-end, this applies to front-end as well. Why make the `CompanyDetailsPage` receive an HTTP component and perform a request? It can simply receive a `CompanyRepository` and you can call `repository.getDetails()`.

{{ medium_first(id="microservice-size-and-splitting-dd9fc98a262e") }}
