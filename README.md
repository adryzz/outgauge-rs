# outgauge-rs
[OutGauge](https://documentation.beamng.com/modding/protocols/) protocol crate (BeamNG)

This crate allows you to interface your rust app with [BeamNG](https://beamng.com/game/) in order to develop a custom user interface.

See the [examples](https://github.com/adryzz/outgauge-rs/tree/master/examples) to learn more.

Enable OutGauge in the BeamNG settings, and then run the example to see the motor RPM in your console.

`cargo run --example async-print`

## no_std support

This crate supports `no_std` environments, and `embassy-net`
In order to take advantage of this, install it with `--no-default-features` and `-F embassy, embassy-net`