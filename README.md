# return-ip

UDP service that responds to messages received with the source IP address, in ASCII.

## example session

```
$ nc -6 -u -w1 HOST 12345 < <(perl -E 'say "hi"x20')
AN:IPV6:ADDRESS:...
$ nc -4 -u -w1 HOST 12345 < <(perl -E 'say "hi"x20')
AN.IPV4.ADDRESS.FOO
```

## the future

might like to speak dns some day.
