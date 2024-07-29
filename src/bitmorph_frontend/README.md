# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080


## TODO

Use [extractors](https://dioxuslabs.com/learn/0.5/reference/fullstack/extractors) to
communicate with backend canister.

## Upstream Dioxus Bugs

**manganis** cli still looking for an input css despite
adjusting the Dioxus.toml config. The workaround is to
remove `~/.cargo/assets`. Source of workaround is in this
[link](https://github.com/DioxusLabs/dioxus/issues/2192#issuecomment-2028548217).
A [fix is merged](https://github.com/DioxusLabs/manganis/issues/14) but not
yet part of upstream's releases.
