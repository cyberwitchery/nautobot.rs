# compatibility

maps `nautobot.rs` releases to the Nautobot upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| nautobot.rs | nautobot | notes                                |
|-------------|----------|--------------------------------------|
| 0.3.x       | 3.1.x    | CI pinned to 3.1.1                   |
| 0.3.x       | 2.4.x    | CI pinned to 2.4.27 (before #1)     |

older client releases have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`.
when the upstream drift check opens an issue, update the pin, run CI, and add
a row here before closing the issue.
