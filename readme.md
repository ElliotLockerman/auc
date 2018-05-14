# AUC: Arbitrary Unit Calculator

AUC is a calculator that lets you work with whatever units you need, unlike other unit-aware calculators which only work with metric/imperial units or derivatives thereof. 

For example, suppose you run a data center with 10,000 hard drives. The manufacture guarantees an average one million hours to a drive failure. Since you have 10,000 drives, however, these hours are accrued *in parallel*. The million hours are *drive-hours*, akin to man-hours. Once this is established, figuring out how often to expect failures becomes a simple dimensional analysis problem. However, common software for doing such calculations wouldn't support such esoteric units as drive-hours. This is where AUC comes it - it supports *any unit you can imagine*.

For example, to solve the above problem, you would write

```
> (1 failure / 1e6 hdd hr) * 10000 hdd
```
and receive the solution

```
1 failure / 100 hr 
```

