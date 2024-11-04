# XMas Challenge

## Get Started
- Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install deno `curl -fsSL https://deno.land/x/install/install.sh | sh`

## Run
- Prepare data `make ip-setup`
- To run application `make ip-web`
- To run tests `make ip-benchmark`

## Benchmakrs
- `/refresh` 0.877s
- `/ips` 0.00084s
- `memory` n/a


Introduction
----

The idea is to try to optimize a .NET service. It's a simple HTTP service that has two methods:
- Load the IP data
- Resolve a location by IP.

The goal is to improve the speed of loading the IP data, lower the service's memory usage and increase the IP search speed.

The main target of this challenge is to learn about .NET performance tricks and techniques. It's allowed and encouraged to collaborate in groups, share ideas, and code to find the best solutions.

Loading a GeoIP database CSV file
----

A "load" method, which can specify a file of the CSV file where all the IP info is provided:
http://localhost:8001/refresh?IP2LOCATION-LITE-DB5.CSV
(this GeoIP database is around 270MB)
... and a row will look like the following:

"16804352","16804607","JP","Japan","Tottori","Tottori","35.500000","134.233000"

And the output message could be something like this:
```
GeoIp table loaded 00:00:07.4427546
MemoryAfterLoad: 787 MB
```

And the "IP locations API"
----

With an API like:

http://localhost:8001/ips?value=123.123.132.123,213.123.132.124
Use this for your preliminary benchmark, I will use 10 IPs as well but a bit different ones to avoid biased results:
http://localhost:8001/ips?value=
1.0.107.0
,12.151.160.0
,41.43.189.0
,67.185.139.0
,85.106.24.0
,109.208.221.0
,173.249.140.0
,197.25.154.0
,197.234.221.0
,223.252.177.0

And will build the following response:
```
China/Beijing,United Kingdom of Great Britain and Northern Ireland/London,...
Time to process 00:00:00.1839841
```

So the challenge is simple:

Make the code FAST and SMALL (memory-wise)!

Rules:
---
* .NET 6/7 only
* No ASP.NET Core, use the provided HttpListener to simplify code (as it's not the part for optimizations).
* The HTTP API will always be called with the concurrency of 1 (no need to synchronize two HTTP methods).
* It is recommended not only to optimize the code as is but if possible the algorithms.
* It is recommended to derive from the original CSV and build a separate format if it makes it easier to improve the loading time (the file can be split and loaded in parallel).
* You can make the code multi-threaded inside.
* You use any library or write a bit of ugly performant code. The main target is performance.
* Data CONTENT cannot be changed, all the columns and rows must be loaded even if some are not used to print the results.
* To share the code either create a new branch or send it directly to me via Slack if you don't want to reveal it to others during the challenge.

The score:
---
There are three main aspects to improve:
- The speed of the IP data loading (refresh endpoint) (30% of the score). Baseline: 3.5s
- The service memory usage (working set, 50% of the score). Baseline: 792 MB
- The speed of searching a location by IP (ips endpoint) (20% of the score). Baseline: 0.5s

The score is a weighted log scale. In this way, you can improve more times, but your benefit in scoring will decrease.
log10 = log base 10.
So the score is with this formula:
```
Score = log10 (reload) x3 + log10 (getIp) x 2 + log10 (redurecMemory) x 5
```

So a person who would speedup the refresh by 10 times, ips endpoint by 100 times, and reduces the memory usage by 10 times will have this score (it's just, for example, to showcase the math, does not mean it's possible or impossible to optimize that much):
```
log10(10) x 3 + log10 (100) x 2 + log10 (10) x 5 = 
3 + 4 + 5 = 12
```

Last notes
----
- If you have any issues running the server/setting it up and so on, feel free to contact  @sbaltulionis-kayak.con on how to run it
- if you find a bug, please report it to Simonas and the implementation will be adjusted. Please don't consider security (i.e. parameter validations as bugs, I consider that the code is working well enough for this challenge)
- Fill the preliminary results on https://docs.google.com/spreadsheets/d/1o2oYOBngmU0vY9YGKJoaOn97CYfcnHw0ji2AOCguv_c/edit?usp=sharing. This is just to see how others progress and how your current solution ranks between other engineers.
- For the speed performance when adding preliminary results measure at least 10 times and take the average (you can modify the code to do that when calling each method).
- I will do benchmarks that count on my machine to avoid any bias and will check if the solution fits the requirements. But as it would take a lot of time to do it every time you change something, it will be done at the end only to get the final results. I will share the benchmarking code after the challenge ends for transparency.
- Final results should be similar to your provided ones because we all have pretty similar laptops.
- Don't forget to use the release build mode and launch the service directly using the .exe file.
