# Manuel d'installation #
Ce manuel explique comment installer le simulateur ainsi que le visualiseur de **Simumo**.

IMPORTANT: Le simulateur fonctionne seulement avec un système exploitation Linux.

## Prérequis ##
* Python 3.6.7 https://www.python.org/downloads/release/python-367/
* make
* Rustup
* linux

## Installation ##

### développement ##
Afin de créer l'environnement virtuel de développement, lancer la commande suivante à partir de la racine du dossier dans le terminal:
```
make dev_venv
```
ensuite pour entrer dans l'environnement virtuel, exécuter:
```
source venv/Scripts/activate
```

Pour sortir de l'environnement virtuel, exécuter:
```
deactivate
```
#### Simulateur ####
Pour exécuter le simulateur, lancer la commande suivante à partir de la racine du dossier **simulator** dans le terminal:
```
cargo run -- -c <pathOfSimulatorConfigFile>
```
Explorer le dossier *./simulator/etc* pour avoir quelques exemples.
#### Visualiseur ####
Pour éxecuter le simulateur,  exécuter la commande suivante à partir de la racine du dossier **visualizer** dans le terminal:
```
python Server.py <pathOfVisualizerConfigFile>
```
Explorer le fichier *visualizer/visualizationConfigExample.yaml* pour avoir un exemple.

### Release ###
Afin de créer l'exécutable, lancer la commande à partir de la racine du dossier dans le terminal:
```
make build
```

Pour avoir plus d'informations relatives à l'utilisation du simulateur, référez-vous au document: **usage**

