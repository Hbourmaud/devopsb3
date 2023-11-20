# DEVOPS B3

## Sommaire
 - [wik-dps-tp01](#wik-dps-tp01)

## wik-dps-tp01

### Prérequis et installation

Il est nécessaire d'avoir rust (rustup ...) et cargo d'installés pour pouvoir faire fonctionner ce projet.

https://www.rust-lang.org/tools/install

Cloner le projet:

```
git clone https://github.com/Hbourmaud/devopsb3.git
```

Puis pour compiler le projet, les dépendances:

```
cargo build
```

Finalement pour lancer le serveur web:

```
cargo run
```

Par défaut la route utilisable est:
http://localhost:7878/ping

### Variable d'environnement
Pour changer le port utilisé (par défaut 7878), il faut modifier la variable d'environnement **PING_LISTEN_PORT**

*Par exemple via Powershell : `$Env:PING_LISTEN_PORT=7800;`*

Ainsi le port utilisé sera 7800 donc la route joignable sera http://localhost:7800/ping

### Dépendance utilisée

**serde_json** https://crates.io/crates/serde_json/1.0.108