+++
title = "Using dictionaries in shell scripts"
[taxonomies]
tags = [ "dev", "shell" ]
[extra]
summary = "Sometimes you need to script in bash. And it’ll probably be a pain in the neck. Dictionaries to the rescue!"
+++

Sometimes you need to script in bash. And it’ll probably be a pain in the neck. Dictionaries to the rescue!

Anyway, chances are that, if you have ever written some scripts, you have already come up with something like this:

{{ gist(url="https://gist.github.com/denisidoro/5d98ee3cdf10c08c400a399798d4e63c") }}

Bash doesn’t allow returning two values so we need to return a string which represents multidimensional data, using `;` as delimiter.

### Can we do better?

I think so!

With a set of scripts such as [this one](https://github.com/denisidoro/navi/blob/7be9353a41d5ae1e56ef60d4761863e73cef3d89/src/dict.sh), we can write:

{{ gist(url="https://gist.github.com/denisidoro/070d7b1ac6cf8676465e4d3f85795b03") }}

We’re using textual data to represent a dictionary/map in this case.

The script isn’t smaller, but that’s not our main objective. We’re trying to achieve legibility.

If you only read `cut -d';' -f1` you have no idea what's going on unless you debug the code. If you read `dict::get foo` at least you can expect it to return something foo-like.

Also, it composes very nicely:

{{ gist(url="https://gist.github.com/denisidoro/6843d26ac9ed1fc38333f772844ceda1") }}

There are more examples [here](https://github.com/denisidoro/navi/blob/7be9353a41d5ae1e56ef60d4761863e73cef3d89/test/dict_test.sh).

### Why not JSON? Or YAML?

If you’re sure that the platform which will run the script has something like [jq](https://stedolan.github.io/jq/) then JSON is a good candidate!

But if you want the script to be extremely portable, then a custom solution is required. Besides, the core of the library has ~30 lines of code, anyway.

{{ medium_first(id="dictionaries-in-shell-scripts-61d34e1c91c6") }}
