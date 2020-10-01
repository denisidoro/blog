+++
title = "Using Grafana for personal financial management"
[taxonomies]
tags = [ "grafana", "finance", "clojure" ]
[extra]
summary = "Building dashboards using familiar tools."
+++

{{ medium_image(
    id="1*tfGEp0dvIJ1_L3-D0rJSdw",
    ext="jpeg",
    caption="One of the dashboards provided by the system") }}

This post is intended to be a brain dump with quick highlights of some technologies. If you’re somewhat familiar with the stack I’ve used, hopefully there will be something useful for you to learn or to base on. It will be fast paced.

### Objective

To develop an investment tracker with the following requirements:

* everything must be determined by a plain-text file;
* the only recurring input needed is sparse balance data;
* it must infer data as good as possible;
* it must maintain historic data;
* it must draw beautiful visualizations.

In other words, if I write down today the balance of some investments of mine, and 1 month from now I write down the balance of other investments, the system should be able to interpolate data so that it can plot smooth, realistic curves.

This is different from a ledger system, for which there are plenty of open source solutions.

It could be done with some Excel wizardry, I suppose. But I didn’t want to learn that much of Excel.

### File syntax

I’ve decided to use the [edn format](https://github.com/edn-format/edn), because I was determined to code the system in [Clojure](https://clojure.org/). Here is some simplified, mock data:

{{ gist(url="https://gist.github.com/denisidoro/d9c38c04feb8e69c27d706adf435043a") }}

It has as little information as possible yet it still is human-readable. Nice!

### Internalizing data

The first step was to convert the file contents to internal models with namespaced keys. In particular, I converted date strings to a numeric format so that I could apply traditional math over it. After some maps and reduces, we have a collection of elements such as:

{{ gist(url="https://gist.github.com/denisidoro/8ffe9d8009d712a500ba1fbe79c8e93c") }}

Initially, I had written functions for integrals, derivatives, curve aggregation and so on. But I was reinventing the wheel. Now, I’m leveraging the robustness of time-series ecosystems.

The competitors were [Prometheus](https://prometheus.io/), [InfluxDB](https://www.influxdata.com/) and [Graphite](https://graphiteapp.org/). As of the time of writing, there’s no way to fetch past data into Prometheus. InfluxDB has too much SQL for my taste. Graphite was the chosen one, then.

With a simple pure function, we can export our internal models as Graphite-compatible data:

```
citigroup.rf.cdb_paribas 1554325794 1000.00
citigroup.rf.cdb_paribas 1554378794 1050.33
citigroup.rf.cdb_paribas 1564325312 1097.44
```

Now we can perform queries such as `sumSeries(some.bucket.*)`, instead of hand crafting it directly in our code.

### Visualizing balances

Graphite has a built-in renderer for data:

{{ medium_image(
    id="1*nLHgAwVpSW9veGmMp_-Rbg",
    caption="This is really ugly") }}

But we aren’t in the 90s anymore. I wanted to use something easier on the eyes: [Grafana](https://grafana.com/).

Fortunately, Grafana works out of the box with Graphite, so it was a piece of cake to build beautiful dashboards:

{{ medium_image(
    id="1*FFPJW-tIF3967y3BcEoSoQ",
    caption="The main dashboard") }}

### Tabular and scalar data

Even though piping data to Graphite made my life easier, it limited possibilities. There’s no simple way to display in a table all the info for yielding investments. Or adding a single stat with the next investment maturity date.

It became clear I had to add a second data source. Considering that some steps before I already had all the data in Clojure code, I decided to upgrade the Clojure scripts to an HTTP server.

{{ medium_image(
    id="1*274rN8NGm-TdaKmZXEwquA",
    caption="InfluxDB can’t provide this tabular data") }}

Grafana has support for [JSON APIs as data source](https://grafana.com/plugins/grafana-simple-json-datasource). Its serialization format is different from Graphite’s but I only had to write yet another pure, adapter function to get a result like this:

{{ gist(url="https://gist.github.com/denisidoro/f2cb104d0c47aec5e93b81d84568ef67") }}

### Extras

After finishing all the groundwork, it became easy peasy to add new features, such as consuming external APIs for displaying stock values or currency history, to name a few.

{{ medium_image(
    id="1*7TJ_w-ThefC6aVi4Yy4J9g",
    caption="Currencies and stock information scraped from third-party APIs") }}

[This](https://github.com/denisidoro/rosebud/blob/e68671b/server/resources/example_log.edn) is a complete edn file specifying what currencies and stocks we should keep track of.

### Deploying

The easiest way to install Grafana on OSX is to use [brew](https://brew.sh/). Then you run `brew service start` and have to remember to stop it because the service persists even after a reboot. Then there's Graphite and Clojure.

Maybe there’s a better way to do it but I rage quit it and started using [docker](https://www.docker.com/) for everything instead.

By mounting volumes, we can start Grafana pre-configured with the default data sources and dashboards in a such way that we don’t have to set it up manually everytime.

### Adapting it to your needs

[The code is available on Github](https://github.com/denisidoro/rosebud) for you to fork. It’s a little bit oriented to my needs and the Brazilian financial system. However, smalls adjustments should be enough to adapt it.

### Conclusion

I’m very happy with the result because I was able to develop it quickly, without having to learn new tools. [The way the namespaces and models are organized](https://medium.com/@den.isidoro/microservice-size-and-splitting-dd9fc98a262e) facilitates swapping backends or splitting the code into microservices, if the future demands it.

{{ medium_first(id="using-grafana-for-personal-financial-management-ac0e4d0cb43") }}
