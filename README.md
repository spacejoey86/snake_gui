UI library prototype

Aims
- Support complex layouts
- Extensible with more layouts, widgets and backends

Current features
- Terminal backend
- Text widget
- common layout containers

What's next?
- opengl backend
- state and mouse interactions
- more widgets
    - button
    - checkbox
    - etc.
- more containers
- better naming
- documentation
- tests

# Architecture
The core of the library is traits.rs, which provides traits to define UI elements
position.rs defines a position struct

the layout_containers module provides containers that don't do any rendering.
these are independent of backends.
Note that you can implement layout containers in your own library - but consider contributing them here.

widgets and (NAME NEEDED)_containers modules provide partial implementations of various UI elements
Backends should implement the backend specific parts of as many of these as possible.
This means that users can swap backends without needing to change which widgets they use

Backends can also provide backend specific UI elements (both widgets and containers)
