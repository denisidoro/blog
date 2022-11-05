+++
title = "A memory-efficient data structure for geolocation history"
[taxonomies]
tags = [ "dev", "rust", "geo" ]
[extra]
summary = "Do we really need to store all absolute data points?"
+++

In this blog post I'll cover how I built a memory-efficient data structure for storing location history data.  
  
Some Rust details will also be explained.  
  
### Motivation  
  
My DSLR camera has no GPS built-in. Evidently, photos from it don't include geolocation-based EXIF tags.  
  
If only my location history were stored somewhere, so that I could update photos with the correct data...  
  
Unsurprisingly, Google knows where I am, where I've been ~~and where I'm going to be~~. They even allow me to export this data.  
  
The problem is that the exported JSON weighs ~1GB. My computer has 16GB of RAM so it can all fit in memory, but we certainly can store it more efficiently.  
  
### JSON parsing  
  
For reference, here's what the JSON looks like:  
```json
{
  "locations": [{
    "latitudeE7": 435631892,
    "longitudeE7": 26848689,
    "accuracy": 56,
    "activity": [{
      "activity": [{
        "type": "STILL",
        "confidence": 100
      }],
      "timestamp": "2014-01-09T00:48:24.424Z"
    }],
    "source": "WIFI",
    "deviceTag": 1348159918,
    "timestamp": "2014-01-09T00:48:24.751Z"
  }, {
    "latitudeE7": 435631881,
    // ...
```  
  
Deserializing a JSON file into a struct results in something that is certainly lighter than the JSON string.  
  
The reason for this is that a string with the value `"2022-10-22"` needs more bytes than, say, its equivalent `Date`  object.  
  
The latter can simply throw away the hyphens, for example. Additionally, the number 10 can be represented with 4 bits instead of using two chars, which occupy at least 8 bits each.  
  
So moving away from a textual representation is the first step in our journey.  
  
Instead of using a JSON deserializer per se, I decided to iterate over each line of the file and ignore the lines I wasn't interested in.  
  
### Do we really need to store everything?  
  
A naive structure for our purposes is a sequence of `DataPoint`s, where `DataPoint` has `datetime`, `latitude` and `longitude`:  
  
```json 
[
 (DateTime(2022,10,22,10,45,32), 27.5226335, 43.552225),  
 (DateTime(2022,10,22,10,46,21), 27.5226382, 43.552237),  
 // ...
]  
```  
  
But isn't there some redundancy? The data is clearly not random.  
  
There are some assumptions we can make to help us design a better data structure:  
1. Data points are sorted by time  
2. Neighbor data points are very similar
   - unless I'm on a plane, my speed is at most 100 km/h, so in a minute I'll travel less 2 km
   - on foot I'll travel just a few meters 
4. Most of the days I stay inside a circle of 3km of radius  
5. One data point per minute is more than enough  
6. Errors in position up to 15m is something I'm OK with  
  
### How many bits do we need to represent a position?  
  
First let's try to understand how much each decimal place contributes to precision.  
  
A quick search on Google gives us [the following](https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude):  
```bash 
- The first decimal place  is worth up to 11.1 km: it can distinguish the position of one large city from a neighboring large city.
- The second decimal place is worth up to 1.1 km: it can separate one village from the next.
- The third decimal place is worth up to 110 m: it can identify a large agricultural field or institutional campus.
- The fourth decimal place is worth up to 11 m: it can identify a parcel of land. It is comparable to the typical accuracy of an uncorrected GPS unit with no interference.
```  
  
If we want to have errors less than 15 m, then we need to be able to correctly represent the latitude and longitude up to the 4th decimal place.  
  
If both coordinates are off by 0.0001° then the error will be close to `sqrt(10^2+10^2) = 15 m`.
  
Given that neighbor data points are close to each other, we can represent deltas in position instead of absolute positions.  
  
Let's first try using a single byte to represent a delta in latitude.  
  
Given that we must represent the 4th decimal place precisely, we can have a range that goes from 0 to `2^7-1` = 127, where 0 translates to 0°; 1 to 0.0001°; and, as a consequence, 127 to 0.0127°. The remaining bit can be used to indicate if the difference is positive or negative.  
  
0.0127° of difference in both coordinates results in a distance of ~2.1km. This is good because, from assumption number 3, we can conclude that, for most days, most data points can be represented with deltas of only one byte for each coordinate.  
  
This conversion simply consists of a rule of three, so I'll skip further details.  
  
### How many bits do we need to represent time?  
  
From assumption number 4, we can discard seconds from timestamps.  
  
Let's say that we're looking at an interval of 20 years. 20 years is roughly equal to 10.5 million minutes.  
  
To represent this number we need `ceiling(log2(10.5 million))` = 24 bits.  
  
Rust doesn't have a `u24` type. Even though we could implement a `u24` type with a tuple of `(u8, u16)`, in the end our  `u24` would end up using 4 bytes instead of 3:
  
```rust
type struct u24 {
  msb: u8,
  lsb: u8
}

fn main() {
   let x = u24 { msb: 1, lsb: 1 };
   println!("{} bytes", x.deep_size_of()); // 4 bytes
}
```

This happens because of [memory alignment](https://doc.rust-lang.org/reference/type-layout.html).

In Rust, we can circumvent this "limitation" by using the `repr` directive:
```rust
#[repr(packed(1))]
type struct u24 {
  msb: u8,
  lsb: u8
}

fn main() {
   let x = u24 { msb: 1, lsb: 1 };
   println!("{} bytes", x.deep_size_of()); // 3 bytes
}
```

But even [clippy](https://github.com/rust-lang/rust-clippy) complains about this, with the `unaligned_references` warning.
  
So let's stick with a `u32` for timestamps instead. Hey, at least we can now represent 8000+ years worth of data.
  
### Getting our hands dirty  
  
At its core, our database will do the following:  
  
```rust  
fn add(&mut self, time: DateTime, lat: f32, lng: f32) {  
   let point = self.low_precision_point(lat, lng);  
   if error(point, lat, lng) > threshold {  
      self.store_high_precision_point(time, lat, lng)  
   } else {  
      self.store_low_precision_point(point)  
   }  
}  
```  
  
Simply using the 0.0001°-based rule in the `error()` function isn't enough because the error varies with your distance to the Equator. To be more precise, this function must calculate the [haversine distance](https://en.wikipedia.org/wiki/Haversine_formula) instead.  
  
This is more CPU-intensive but it's a cost we need to pay at the time of writing only.  
  
### Defining our structs  
  
Our data structure for low-precision datapoints can be a sequence of 2 1-byte positions, as mentioned above. We also need to have a reference to a high-precision point so that the final position is reference + delta.  
  
Our data structure for high-precision can be a sequence of minutes plus both coordinates.  
  
In the end we'll have something like this:  
```rust  
type Minutes = u32
type LatLng = (f32, f32)
type LatLngDelta = (u8, u8)
  
struct Db {
   high_precision_points: Vec<Minutes, LatLng>,  
   low_precision_points: HashMap<Minutes, Vec<LatLngDelta>>  
}
```  
  
I'll skip the implementation details but you can check the full code [here](https://gist.github.com/denisidoro/c79282fa44aab10f5a33e838b8b1811f).  
  
### Benchmarking  
  
After finishing everything, we can measure how many megabytes our data structure needs.  
  
Let's insert data for the last couple of years into it and measure memory consumption:  
```bash 
input points: 345 k
high-precision data points: 42 k
low-precision data points: 280 k
high-precision structure size: 768 KB
low-precision structure size: 2583 KB
sum size: 3351 KB
```  
  
In the end, less than 4 MB were occupied. 
  
### Oversized Vecs  
  
There's something odd with the numbers above: the `Vec`s should be smaller. 

If we have 42k elements of `32*3 bits` = 12 bytes each, the total sequence should have ~492KB, not 768KB.
  
After some experiments, I realized that the `Vec`s were bigger than expected by 5 to 100%. In average, they were ~55% bigger.  
  
I then remembered that such data structures usually double their inner buffers when there's no room for a new element.  
  
Fortunately, Rust offers a [Vec::shrink_to_fit()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit) operation, which we can call after finishing mutating our database.  
  
### Benchmarking  
  
Finally, here are our results:  
```bash
input points: 345 k
high-precision data points: 42 k
low-precision data points: 280 k
high-precision structure size: 499 KB
low-precision structure size: 2339 KB
sum size: 2838 KB
```  
  
Let's compare this to a naive `Vec<Timestamp, f64, f64>` implementation:  
```bash
input_points * (size_of_timestamp + 64 + 64) * average vec overhead
354k * (64+64+64) * 1.5 bits
11.8 MB
```
  
Overall we saved around ~80% in memory!  

After fiddling with some thresholds, I noticed that, to achieve a 5m precision instead of 15m, the memory overhead is negligible:
```bash
input points: 345 k
high-precision data points: 46 k
low-precision data points: 273 k
high-precision structure size: 546 KB
low-precision structure size: 2326 KB
sum size: 2872 KB
```

So I decided to set the max error to 5m instead.

### Future improvements

A high-precision data point occupies 32 bits for a timestamp and 32 bits for each coordinate. 

Timestamps could be represented with 24 bits, as mentioned above. 32 bits for each coordinate is unnecessary.

We could use a `u64` for everything instead: with some bit shifting, 24 bits could be reserved for the timestamp and each coordinate could use 20 bits.

This would decrease the size of these data points by `1-64/(32*3)` = 33%.

In addition, I wonder if we could use [QuadTrees](https://en.wikipedia.org/wiki/Quadtree#:~:text=A%20quadtree%20is%20a%20tree,into%20four%20quadrants%20or%20regions.) somehow...
  
### Conclusion  
  
This journey was clearly over-engineered.  
  
But I learned a lot as part of the process and I ended up with results very efficient memory-wise.  
  
Instead of asking "why?", I asked "why not?" 😂  