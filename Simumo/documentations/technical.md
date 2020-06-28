# Manuel technique #
Ce document explique l'architecture et la structure des fichiers.

## Géneration automatique de la documentation du code Rust
Au niveau de la racine du dossier simulator, lancer la commande:
```
cargo doc --open --no-deps --document-private-items
```

## build ##
Ce dossier contient tous les fichiers permettant d'exécuter le simulateur en version de production.
Le script **simumo.sh** permet d'exécuter le simulateur.

## documentations ##
Ce dossier contient toutes la documentation à propos de l'installation, utilisation et la conception du projet.

## scripts ##
Ce dossier contient les scripts du projet. Le plus important est : **simumo.sh**

## simulator ##
Ce dossier contient le simulateur en Rust, qui calcule des données de la simulation.
### etc
Ce dossier contient les différentes configurations qui ont été développées. On retrouve aussi les graphes dans les fichiers json. Ces graphes sont utiles si vous souhaitez ne pas utiliser l'API **osmGraph**.
### extern
Ce dossier contient des librairies externes qui ne sont pas téléchargées par le web.
### src
Ce dossier contient le code source du simulateur. Les sous-dossiers ont été créés selon l'architecture **ECS**. On y retrouve les *composantes* ("components"), les *entités*, les *ressources*, et les *systèmes*.
1. commons
    Ce dossier contient tous les types de données qui sont communs dans le simulateur. C'est l'équivalent d'un module utilitaire.
2. components
    Ce dossier contient les composantes définies pour l'architecture **ECS**.
3. configurations
    Ce dossier contient les différents types de configurations possibles. Regardez les fichiers se retrouvant dans **./simulator/etc** pour plus de détails.
4. entities
        Ce dossier contient les entités définies pour l'architecture **ECS**.
5. ressources
        Ce dossier contient les ressources définies pour l'architecture **ECS**.
6. simulation
    Ce dossier contient tout ce qu'il faut pour construire le simulateur, le démarrer.
7. systems
    Ce dossier contient les systèmes définies pour l'architecture **ECS**.
### target
Ce dossier contient les exécutables de débug et de production.

## simumap ##
Ce dossier contient le "package" afin d'utiliser l'API **osmGraph**. Il a été écrit en Python et est utilisé par le simulateur pour importer des cartes routières.

## visualizer ##
Ce dossier contient le visualiseur web.
### layoutVisualization
Ce dossier contient tout le contenu autour de la carte. Le header, footer etc.
### logs
Ce dossier est seulement un exemple pour le chemin des *logs* (données calculées par le simulatleur). 
### ol-geocoder
Ce dossier contient l'outil permettant de générer la carte et de zoomer au bon emplacement. C'est la carte qu'on peut voir dans le visualiseur. Il contient l'information à propos des différents types de carte.
### visualizationConfigExample.yaml ###
Ce fichier est essentiellement un exemple de configuration pour le visualiseur.
Il est important de spécifier le même chemin de *log* que celui utilisé pour le visualiseur.

## venv ##
Ce dossier contient tout ce qu'il faut à propos de l'environnement virtuel. Il a été créé à partir du makefile.
## makefile ##
Ce fichier permet de créer l'environnement virtuel de développement et de créer le dossier build.