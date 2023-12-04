# DEVOPS B3

## Sommaire
 - [wik-dps-tp01](#wik-dps-tp01)
 - [wik-dps-tp02](#wik-dps-tp02)
 - [wik-dps-tp03](#wik-dps-tp03) 

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

## wik-dps-tp02

*Les fichiers concernant ce tp se trouvent dans le répertoire ``tp-wik-dps-tp01`` pour plus de facilité d'usage.*

### Simple Stage

Le dockerfile utilisant un seul stage pour exécuter l'API se nomme [Dockerfile](tp-wik-dps-tp01/Dockerfile)

Pour créer l'image liée à ce dockerfile, déplacez vous à l'endroit du repo: ``tp-wik-dps-tp01``

Puis:
``docker build -t my-rust-app .``

Pour lancer le container correspondant:
``docker run -it --rm -e PING_LISTEN_PORT=8080 -p 7200:8080 my-rust-app``

*Il y a toujours la possibilité de personnaliser le port d'écoute et également de changer le port correspondant de docker pour pouvoir y accéder sur la machine hôte (-p 7200:8080 ici).*

### Scan trivy

[trivy_result.txt](trivy_result.txt)

On remarque qu'il y a 758 vulnérabilités trouvées dont 3 critiques sur l'image my-rust-app créé juste avant.

### Multiple stage

Le dockerfile utilisant deux stages pour exécuter l'API se nomme [multistage.dockerfile](tp-wik-dps-tp01/multistage.dockerfile)

Pour créer l'image liée à ce dockerfile, déplacez vous à l'endroit du repo: ``tp-wik-dps-tp01``

Puis:
``docker build -t my-rust-app -f .\multistage.dockerfile .``

Pour lancer le container correspondant:
``docker run -it --rm -e PING_LISTEN_PORT=8080 -p 7200:8080 my-rust-app``

*Il y a toujours la possibilité de personnaliser le port d'écoute et également de changer le port correspondant de docker pour pouvoir y accéder sur la machine hôte (-p 7200:8080 ici).*

## wik-dps-tp03

Le fichier docker compose se trouve dans le répertoire *tp-wik-dps-tp03*:  [docker-compose](tp-wik-dps-tp03/docker-compose.yaml)

Pour utiliser le docker-compose:
``docker compose up --build``

L'API est donc maintenant accessible via : ``http://localhost:8080/ping``

Pour afficher à chaque requête *ping* le hostname, une dépendance dans le code Rust a été ajoutée: 
- [gethostname](https://crates.io/crates/gethostname)