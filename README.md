# Simple Rust Chat Server

## TODOs

Must todos to have success

- [x] Receive requests
- [x] Respond to requests
- [ ] Mount a Request object
  - [x] Mount headers
  - [x] Mount host
  - [x] Mount path
  - [x] Mount query-params
  - [ ] Mount data
    - [ ] Build from raw text (zzz)
    - [ ] Build from JSON (zzz)
    - [x] Build from form-data
      - [x] Field -> data
      - [ ] Files (Probably no) (zzz)

## Future

Should accept `HTTP/1.1` requests or start a web `WebSocket` request/response event.
