UI library prototype

Aims
- Support complex layouts
- Extensible with more layouts, widgets and backends

Current features
- Terminal backend
- opengl backend
- Text widget
- containers, providing common layouts

What's next?
- more widgets
    - checkbox
    - slider
    - etc.
- more containers
- better naming
- documentation
- tests

# Architecture
The core of the library is traits.rs, which provides traits to define UI elements
position.rs defines a position struct

Container and widget are not types - they are just names used in documentation.
UI element is also not a type, but just a name for any type that implements the appropriate traits (see traits.rs)

The pure_containers module provides containers that don't do any rendering.
They layout children in some way.
Pure containers are backend independent.
Layout being done by containers instead of a core layout engine manages complexity - adding a new layout does not increase the complexity of the core library.
Note that you can implement pure containers in your own library - but consider contributing them here.

The spacers module provides spacer widgets. Unlike most other widgets, these do no rendering and therefore are backend independent.

widgets and visual_containers modules provide partial implementations of various UI elements that require backend specific implementations.
Backends should implement the backend specific parts of as many of these as possible.
This design means that users can swap backends without needing to change which widgets they use.

a backend primarily provides a BackendContext, and implementations for widgets and visual_containers
Backends can also provide backend specific UI elements (both widgets and containers)

# License

Licensed under either of

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
    ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
