# Energee

A simple TUI to view smart meter readings from the Octopus API: https://developer.octopus.energy/docs/api/#consumption


## Getting started

First, get an API key and your meter information from your Octopus account:

https://octopus.energy/dashboard/new/accounts/personal-details/api-access

Then, simply run the command in your terminal, e.g. with cargo:

```
API_KEY={YOUR_API_KEY} cargo run -- -e {ELECTRIC_MPAN}:{ELECTRIC_SERIAL} -g {GAS_MPRN}:{GAS_SERIAL}
```

First time writing rust so code likely horrific.
