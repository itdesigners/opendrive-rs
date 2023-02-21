# opendrive - Rust parser for [ASAM OpenDRIVE](https://www.asam.net/standards/detail/opendrive/)

[![Build Status](https://github.com/itdesigners/opendrive-rs/workflows/Rust/badge.svg)](https://github.com/itdesigners/opendrive-rs/actions?query=workflow%3ARust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/itdesigners/opendrive-rs)
[![Crates.io](https://img.shields.io/crates/v/opendrive.svg)](https://crates.io/crates/opendrive)
[![Documentation](https://docs.rs/opendrive/badge.svg)](https://docs.rs/opendrive)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/itdesigners/opendrive-rs/issues/new)

This crate provides data types for the [ASAM OpenDRIVE](https://www.asam.net/standards/detail/opendrive/) standard.
Currently, version `1.7.0` is supported. 

See the [very basic example](examples/inout.rs) for a brief usage introductoin.

### cargo features

 - `workaround-sumo-issue-10301`: OpenDRIVE files generated by sumo might lack required `paramPoly3.pRange` values, assume `ParamPoly3pRange::Normalized` while parsing, see https://github.com/eclipse/sumo/issues/10301
 - `workaround-sumo-roadmark-missing-color`: OpenDRIVE files generated by sumo might lack required `roadmark.color`, assume `Color::Standard` while parsing
 - `workaround-sumo`: Enable all parser workarounds related to SUMO
 - `fuzzing`: Load dependency `arbitrary` for fuzzing 

This crate might or might not be developed further as the need for more API calls arise.
That said, (small!) pull-requests are welcome. 

# ASAM OpenDRIVE License

The rustdoc documentation contains comments from the XML-Schemata definitions of the [ASAM OpenDRIVE](https://www.asam.net/standards/detail/opendrive/) standard and therefore might introduce further license restrictions.
These XML-Schemata definitions contain the following file header:

```
ASAM OpenDRIVE V1.7.0

© by ASAM e.V., 2021

ASAM OpenDRIVE defines a file format for the precise analytical description of
road networks


Any use is limited to the scope described in the ASAM license terms.
This file is distributable in accordance with the ASAM license terms.
See www.asam.net/license.html for further details.
```

See also the additional [disclaimer](https://www.asam.net/index.php?eID=dumpFile&t=f&f=4422&token=e590561f3c39aa2260e5442e29e93f6693d1cccd):

  * This document is the copyrighted property of ASAM e.V. In alteration to the
    regular [license terms](https://www.asam.net/license), ASAM allows unrestricted distribution of this standard.
    §2 (1) of ASAM’s regular [license terms](https://www.asam.net/license) is therefore substituted by the following clause:
    "The licensor grants everyone a basic, non-exclusive and unlimited license to use the standard ASAM OpenDRIVE".
