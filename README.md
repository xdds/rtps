<!-- http://photofunia.com/results/57f5772f468679c3198b4568 -->
<img src="https://github.com/xdds/rtps/raw/master/assets/logo-medium.jpg">

Real Time Publish Subscribe
====

<a href="https://travis-ci.org/xdds/rtps">
    <img src="https://travis-ci.org/xdds/rtps.svg?branch=master">
</a>

A mature standard for building publish subscribe systems, with a unique emphasis on quality of service parameters and fast cross platform implementations.

There are over 20 implementations of RTPS (well, of DDS, but RTPS underpins all those implementations). They range from FPGAs to C/C++/Java. This is the first Rust implementation.

Find the spec here: [http://www.omg.org/spec/DDSI-RTPS/2.2/PDF/](http://www.omg.org/spec/DDSI-RTPS/2.2/PDF/). It lacks in easy to read binary breakdown, but in general it does a good job of splitting format, purpose, and behavior. Unfortunately it does not read from beginning to end.

CDR:
 - [x] Enough serialization/deserialization to get by
 - [ ] Full serialization/deserialization for all CDR types
 - [ ] Set submessage endianness on the serializer
 - [ ] Manage alignment
 - [ ] Investigate how to make zero-allocation
 
RTPS:
 - [x] Entity thread abstraction
 - [ ] Basic entity thread stats
 - [ ] Message serialization
 - [ ] Submessage header serialization
 - [ ] Submessage content serialization
 - [ ] UDP test client
 - [ ] Built in entities for DDS
 - [ ] Interop test with [eProsima's modern, open-source C++ impl](https://github.com/eProsima/Fast-RTPS)

Code of Conduct
===

Anyone who interacts with XDDS RTPS in any space including but not limited to this GitHub repository is expected to
follow [our code of conduct](https://github.com/xdds/rtps/blob/master/CODE_OF_CONDUCT.md)

