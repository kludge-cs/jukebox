# Project Roadmap / Implementation Guidelines

- [ ] Form player using FFmpeg or other utility that streams to sockets
- [ ] Create REST / WS backend and server applet
- [ ] Create plugin interface and abstraction layer for target sources
- [ ] Create Discord interface plugin for main player


## Ideas

[actix-web] will handle REST, [async-tungstenite] / [tokio] will handle
websockets (it is likely that tokio will be used through [agnostik]).

[actix-web]: https://github.com/actix/actix-web
[async-tungstenite]: https://github.com/sdroege/async-tungstenite
[tokio]: https://github.com/tokio-rs/tokio
[agnostik]: https://github.com/bastion-rs/agnostik
