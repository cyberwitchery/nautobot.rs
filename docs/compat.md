# compatibility

maps `nautobot.rs` releases to the Nautobot upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| nautobot.rs   | nautobot | notes                                        |
|---------------|----------|----------------------------------------------|
| main          | 3.2.x    | CI pinned to 3.2.2                           |
| 0.5.0         | 3.1.x    | CI pinned to 3.1.6                           |
| 0.4.2-0.4.3   | 3.1.x    | CI pinned to 3.1.4                           |
| 0.4.1         | 3.1.x    | CI pinned to 3.1.2                           |
| 0.4.0         | 3.1.x    | CI pinned to 3.1.0                           |
| 0.3.1         | 2.4.x    | CI pinned to 2.4.27 (before #1)              |
| 0.0.1-0.3.0   | 2.4.x    | bindings from 2.4.20, no integration CI yet  |

integration CI landed in 0.3.1, so everything up to and including 0.3.0 was only
ever checked against the schema its bindings were generated from. those releases
have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`.
when the upstream drift check opens an issue, update the pin, run CI, and add
a row here before closing the issue.
