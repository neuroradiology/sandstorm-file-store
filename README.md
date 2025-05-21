# sandstorm-file-store

This is a simple (but working) rust server that is intended to be used by developers who are packaging sandstorm apps.

It is used within a grain and accepts PUT requests from the sandstorm parent, and writes them to in-grain storage (which is at /var/data).
