This element is a clone of the element-rtpgccbwe that implements the
[Google Congestion Control algorithm](https://datatracker.ietf.org/doc/html/draft-ietf-rmcat-gcc-02).

But instead of trying to adjust the bitrate of the video source it adjust
the pacing of packets to smooth out bursts generated by I-frames.
On packet loss the pacing is adjusted down => packets are spread out over longer time.

The issue with the original rtpgccbwe (for our use case) is that it doesn't really consider
the fact that an I-frame comes in a big chunk and is a major part of the total number of
bits in the stream. The algorithm only allows buffering 30ms of data based on the
estimated bitrate" which, in a best loss-less scenario, would be the streams average bitrate.
When more than 30ms is buffered all excessive data is bursted out immediately. That burst will likely
cause packet loss, which in turn would decrease "estimated bitrate", which would mean an even bigger
burst next I-frame, since the algorithm decrease the bitrate when packet loss occurs.

With this adjusted algorithm the burst are spread out over a regulated period of time. When packet
loss occurs, the spread-out period increase. This will of course introduce delay, so to avoid
situations where packets are delayed too much, a max delay value can be specificed.


## RTP packets as they are received by payload element (for example rtph264pay)

```
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =                             =
   =     =     =     =     =     =     =     =     =
   =     =     =     =     =     =     =     =     =
 __=_____=_____=_____=_____=_____=_____=_____=_____=___
```


## RTP packets as they are passed on with the original rtpgccbwe element BEFORE any loss (≠ => lost packet)

```
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  =                             =
  =                             =
  =======     =     =     =     =======     =     =
__========____==____==____==____========____==____==__
```

## RTP packets as they are passed on with the original rtpgccbwe element AFTER loss (≠ => lost packet)

```
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  ≠                             ≠
  =                             =
  =                             =
  =                             =
__=========___===___===___===___=========___===___===_
```

##RTP packets as they are generated with rtppace

```
  ======                        ======
  =======     =     =     =     =======     =     =
__========____==____==____==____========____==____==__
```