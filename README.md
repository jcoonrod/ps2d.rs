Redox won't boot on most modern x86 hardware due to panic in some drivers. In my case, I need a more relaxed ps2d driver to boot on my ThinkPad X1
After editing the ps2d controller with the accompanying patch, I followed willnode advice and did:
make lc.base (to prevent base repo being updated)
make some changes in recipes/core/base
make r.base
make image

(actually I did make server since I wanted to boot the live.iso - after that worked I did make desktop which also booted - the mouse movements worked but mouse click did not.
