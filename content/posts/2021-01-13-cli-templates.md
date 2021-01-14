+++
title = "Creating templates for CLIs"
[taxonomies]
tags = [ "dev", "terminal" ]
[extra]
summary = "Terminal commands can get quite convoluted. How can we make CLI calls easy to use?"
+++

### Inspiration

I've been using [yab](https://github.com/yarpc/yab) —a curl-like CLI— recently and I was surprised by how elegantly it solves a common problem.

yab calls can get quite verbose. As an example, to get a `Customer` by its `id` from a `customer` microservice:

```sh
yab customer Customer::get \
    -t /path/to/idl/some.company/customer/customer.thrift \
    -P 131.144.23.5/customer:tchannel \
    -r '{"request": {"id": 11}}'
```

The call is pretty convoluted, but it can be simplified by storing params in a yaml file:

```yaml
# customer_by_id.yaml
service: customer
procedure: "Customer::get"
thrift: /path/to/idl/some.company/customer/customer.thrift
peer-list: 131.144.23.5/customer:tchannel
request:
  request:
    id: ${number:11}
```

The file takes a set of CLI flags in their long form, e.g. `--thrift` instead of `-t`, and their associated values.

Then, the following call is equivalent to the beforementioned one:

```sh
yab -y customer_by_id.yaml
```

Additional flags can be passed to yab to override the default values specified in the yaml file:

```sh
yab -y customer_by_id.yaml --number 42
```

I think this is pretty cool!

It can save us lots of time. It also helps sharing knowledge between team members: we could have a git repo with a bunch of these `.yaml` files and a simple `git pull` would allow everyone to be on the same page.

My immediate thought was "could we easily replicate this for other commands?"

# Solution

For the sake of simplicity, let's try to write a complex [httpie](https://github.com/httpie/httpie) command. It could be an involved `awk`, `grep` or `jq` command, though.

The final call should look like this:
```
http -v POST https://jsonplaceholder.typicode.com/posts title=foo body=bar userId=11
```

The tool I'm going to use is [navi](https://github.com/denisidoro/navi). It allows you to browse through cheatsheets —that you may write yourself or download from maintainers— and execute commands. 

navi encourages you to write `.cheat` files which break commands down into smaller, reusable pieces:
```sh
% httpie

# make a request to a typicode microservice
http -v <method> "https://<service>.typicode.com/<endpoint>" <http-body>

$ method: echo -e 'GET\nPOST\nPUT'
$ service: echo -e 'jsonplaceholder\nanotherservice'
$ endpoint: case "${service}:${method}" in; "jsonplaceholder:post") echo -e 'e1\ne2\ne3';; esac
```

The `endpoint` values here are incomplete for briefness and, in a real-world scenario, would probably be fetched dynamically, instead of being harcoded.

This `.cheat` enables us to very quickly make a request to any endpoint in our company, given that we somehow mapped all possible values to the corresponding variables. 

But an engineer who recently joined the team still wouldn't know what service/endpoint to request for creating a new `post`. We could then add another cheatsheet entry for more granular commands:

```sh
% httpie, endpoints

# create a post
http -v <method> "https://<service>.typicode.com/<endpoint>" title=<title> body=<body> userId=<userId>

$ method: echo 'POST'
$ service: echo 'jsonplaceholder'
$ endpoint: echo 'posts'
$ title: echo 'foo'
$ body: echo 'bar'
$ userId: echo '11'
```

There are a multitude of ways to invoke this cheatsheet. One of them is like this:

{{ asciinema( id="Su5eUYFHn7M5A6Yccvcv7WH7k") }}

### Preventing user interaction

If no interaction is wanted we could override values using environment variables and autoselect the desired command by using the `--best-match` flag:

```sh
userId=12 navi --query 'http endpoints create post' --best-match
```

Let's say that we want to skip the `--query` and `--best-match` boilerplate and we know that we're always gonna use this tool for `http endpoints`. A simple bash script  come to the rescue:

```sh
export_var() {
   local -r var="${1//-/_}"
   export "$var"="$2"
}

endpoint() {
   local var
   for v in $@; do
    case $v in
        --*) var="${v:2}" ;;
        *) export_var "$var" "$v" ;;
    esac
   done

   navi --query "http endpoints ${query}" --best-match
}
```

And we could call it as follows:
```sh
endpoint 'create post'
endpoint 'create post' --userId 12
```

### Conclusion

I hope that, with these tips, using the terminal becomes easier for you. 

Creating templates may speed up day-to-day tasks and improve knowledge sharing —either with other team members or with your future self.

By the way, if you have any feature requests for navi, feel free to leave an issue [here](https://github.com/denisidoro/navi/issues)!
