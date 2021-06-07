+++
title = "Working around Google Photos limitation"
[taxonomies]
tags = [ "cloud", "android", "tasker", "termux" ]
[extra]
summary = "Using Tasker and Termux to backup photos"
+++

### Context

As of June 1, 2021, Google Photos [started limiting][gphotosnews] the storage for new photos and videos to 15GB/account.

15GB isn't much nowadays, so I looked for alternatives. 

Signing up for paid storage is an option which I discarded: 
- first, because I don't want to yet another lifetime subscription
- second, because I don't want to be tied to a ecosystem. What if prices double overnight? What if I reach the storage limit and need to upgrade to the next tier?

That said, I love Google Photos and its machine learning wizardry.

A suboptimal experience, yet better than nothing, would be to:
- upload low quality photos to Google Photos
- upload high/original quality photos to alternative storages

This way, Google Photos will still keep me reminding me of past trips or allow me to search for "bridge" or "Rio de Janeiro" against my 1000s of photos. Given an image ID/date, I can download the high/original quality photo elsewhere.

### Alternatives

Whatever the solution would turn out to be, I wanted it to last at least 3.5 years. If we assume 10 photos/day in average, we're talking about ~13k photos here.

I ended up creating/reusing the following accounts:

| Provider | Available storage | Photo size | Trust? |
| :---: | :---: | :---: | :---: | 
| [Google Photos][gphotos] | 6GB | 450KB | Yes |  
| [Box][box] | 7GB | 550KB | Yes |  
| [pCloud][pcloud] | 10GB | 750KB | No |  
| [Degoo][degoo] | 100GB | 7MB | No |  
| [Terabox][terabox] | 1TB | irrelevant | No |  
| [Telegram][telegram] | ∞ | irrelevant | Yes |  

I've never heard about [pCloud][pcloud], [Degoo][degoo] or [Terabox][terabox] before so I'm not uploading my photos in a "human-readable" format to these services.

### Solution

I won't get into too much implementation details, but I basically used [Tasker][tasker] and [Termux][termux]. I also installed the packages for [file][file], [exiftool][exiftool] and [rclone][rclone].

The pseudo-code is as follows:
```
every day at 2am:
    for each photo in /sdcard/DCIM:
        compress photo to /sdcard/Cloud/GooglePhotos/⋯.jpg
        compress and zip photo to /sdcard/Cloud/pCloud/⋯.jpg.7z
        ...
        zip photo to /sdcard/Cloud/Terabox/⋯.jpg.7z
        move photo to /sdcard/Cloud/Telegram/⋯.jpg

every day at 3am, if wifi is connected:
    for each file in /sdcard/Cloud/<provider>:
        move file to <provider>
```

### Compression

A photo is compressed for each storage provider accordingly.

The output image will be limited by `Photo size` and, if `Trust?` is `false`, it will be 7zipped using a password.

The output image size is limited by playing with the properties for image resizing inside Tasker (`max dimension` and `compression quality`). These are estimated based on the `Photo size` input and the original image properties, retrived by `file`.

One caveat is that, for some reason, Tasker doesn't preserve the EXIF attributes when resizing images, so I needed to use `exiftool` to overwrite the new file EXIF attributes with the original ones.

### Uploading files to decent, real storage providers

This was very straightforward and was automated using [rclone][rclone].

### Uploading files to bad, real storage providers

[Degoo][degoo] and [Terabox][terabox] don't have support for WebDAV or similar protocols. In other words, I need to open their app and manually move files.

I plan to do that once a month.

The good news is that all files will already be easily located in `/sdcard/Cloud/Degoo`, for example.

### Uploading files to Telegram

[Telegram][telegram] isn't a storage provider but, to my knowledge, a given chat can have infinite attachments. 🤷

Its API allows file uploads using a simple POST request. The response body contains a `file_id`.

We can download the attachment by making a request to `https://⋯.telegram.org/⋯?file_id=<file_id>`.

We thus need to keep this `file_id` in a database. My Tasker task stores this in `/sdcard/Tasker/db/telegram_uploads.txt` and its contents look like this:
```txt
/Pictures/IMG_00.jpg;123456
/Pictures/IMG_01.jpg;789012
/Pictures/IMG_02.jpg;654321
```

Having this database in hand, we can even design a primitive web-based file manager, for example.

The database is replicated across all other storage providers, using the aforementioned methods.

<figure style="margin-left: 0; margin-right: 0">
    <img src="/posts/gphotos_telegram.jpg">
    <figcaption style="font-size: 0.8em">My chat with my bot will ultimately contain thousands of messages like this</figcaption>
</figure> 

### Conclusion

I'm happy with the solution because Google Photos will still keep doing its magic and photos are replicated. If one storage provider goes down (I'm sure that down the road at least one of them will), I won't lose my photos.

### Code

The Tasker tasks aren't public because they rely on a very specific setup (and because I've never met anyone in person who uses Tasker - or Termux, for that matter).

In the unexpected scenario I get requests to do it, I'll share them in [Taskernet][taskernet].

[gphotosnews]: https://www.theverge.com/2020/11/11/21560810/google-photos-unlimited-cap-free-uploads-15gb-ending
[gphotos]: https://photos.google.com
[box]: https://box.com
[pcloud]: https://pcloud.com
[degoo]: https://degoo.com
[terabox]: https://terabox.com
[telegram]: https://telegram.org
[taskernet]: https://taskernet.com/shares/
[exiftool]: https://exiftool.org
[rclone]: https://rclone.org
[file]: https://linux.die.net/man/1/file
[tasker]: https://play.google.com/store/apps/details?id=net.dinglisch.android.taskerm&hl=en&gl=US
[termux]: https://play.google.com/store/apps/details?id=com.termux&hl=en&gl=US