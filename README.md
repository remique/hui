# hui - http user interface
---
The purpose of the aplication together with attached scripts is to
- store and serve collections of http requests
- provide nice user interface to compose and run requests
- give requests scripting possibility
  - pre hook - script called before request
  - after hook - script called after request
  - give possibility of storing history of request execution
- be well suited for scripting
- be simple to integrate with other tools

## architecture proposition

hui is client->server aplication, which primarely uses FIFO[0] as comunication
channel.

It's built from:
 - huid   - daemon, the core of the hui
 - huictl - cli interface
 - huitui - tui interface
 - huic   - convert from/to hui file format
 - hui    - script connecting all parts of application and making them easier to use

### huid

huid is built from blocks:

```
FIFO API -> |event layer|
            |---------- | -> hooks
            |loop       | -> web API
            |handlers   | -> store
```

0: https://man7.org/linux/man-pages/man7/fifo.7.html
