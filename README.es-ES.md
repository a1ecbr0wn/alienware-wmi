# alienware-wmi

[![Crates.io](https://img.shields.io/crates/l/alienware)](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/alienware)](https://crates.io/crates/alienware) [![Build Status](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml/badge.svg)](https://github.com/a1ecbr0wn/alienware-wmi/actions/workflows/build.yml) [![docs.rs](https://img.shields.io/docsrs/alienware)](https://docs.rs/alienware) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/alienware-wmi/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/alienware-wmi) [![snapcraft.io](https://snapcraft.io/alienware-cli/badge.svg)](https://snapcraft.io/alienware-cli)

Repositorio de herramientas que emulan el script `alienware_wmi_control.sh` que solía venir con la distribución SteamOS de Linux para la máquina de escritorio Alienware Alpha. El crate principal es el crate [`alienware`](https://github.com/a1ecbr0wn/alienware-wmi/tree/main/alienware), que proporciona el acceso a la API del sysfs de alienware para las luces LED y el control de entrada/salida HDMI. El crate [`alienware-cli`](https://github.com/a1ecbr0wn/alienware-wmi/tree/main/alienware_cli) utiliza la API para proporcionar acceso mediante línea de comandos a algunas de las funcionalidades de la API.

También puede interesarte un proyecto de Python para controlar las mismas luces: [`AlienFX`](https://github.com/trackmastersteve/alienfx).

## Descargo de responsabilidad y Licencia

Si utilizas este software, lo haces BAJO TU PROPIO RIESGO.

Este software está licenciado bajo la licencia [Apache-2.0](https://github.com/a1ecbr0wn/alienware-wmi/blob/main/LICENSE).
