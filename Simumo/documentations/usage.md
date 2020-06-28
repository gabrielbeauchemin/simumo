# Manuel de l'utilisateur #
Ce manuel explique comment utiliser le simulateur et le visualiseur.

IMPORTANT: Le simulateur fonctionne seulement avec un système exploitation linux.

## Simulateur ##
Pour utiliser le simulateur, vous devez spécifier un fichier de configuration en entré.
Il faut comprendre que les exemples données des configurations ne sont pas génériques. Il existe des millions de configurations possibles.
### Generals ###
1. clocl_dt: Indique le temps entre chaque tick de l'horloge en seconde.
2. end_time.val: Indique le temps total de la simulation en seconde.
3. debugger: Contient les informations à propos du debugger. La grosseur de la fenêtre et si vous voulez l'utiliser.
4. logging: Indique les informations à propos des données de sorties lors de la simulation.
    1. path: Le chemin pour de calcul des données.
    2. filenames: Contient les fichiers des métriques.
4. (Optionel) seed: Il est possible de ne pas spécifier le seed. Celui-ci sera généré automatiquement.
```
  clock_dt: 0.25
  end_time:
    val: 12.5
  debugger:
    use: true
    width: 720
    height: 720

  logging:
    path: "./tmp_logs"
    filenames:
      - car_positions
      - car_speed

  seed: 055c1509-cf2e-499a-9c79-e85ace6e654d
```
### Map ###
Cette configuration indique quelle carte utiliser pour la simulation.
Il y a deux types de map.
1. type: Indique le type de le map. Dans ce cas **OsmGraph**.
2. latitude: Indique la latitude d'origine de la carte.
3. longitude: Indique la longitude d'origine de la carte.
4. zoom:  Indique le zoom selon le point d'origine.
```
 type: OsmGraph
 latitude: 45.40008
 longitude: -71.89908
 zoom: 1200
```
1. type: Indique le type de carte ("map"). Dans le cas ci-dessous, il s'agit d'un **PolarFileMap**. Les données du graphe sont spécifiées à partir d'un fichier json au lieu d'utiliser l'API **OSMGraph**.
2. path: Indique le chemin vers le fichier json.
```
  type: PolarFileMap
  path: etc/sherbrooke_graph.json
```

Le fichier de *sherbrooke_graph.json* doit ressembler à ceci:
```
{
    "nodes": {
        "1870902252": [-71.896566, 45.3983854],
        ...}
    "edges": [
        [1870902252, 2004673453],
        ...]
}
```
### Systems ### 
Actuellement, il existe plusieurs types de systèmes. Voici les plus importants.
Dans ce fichier, on indique quels types de système sont fournis au simulation.
1. clock: Actuellement, il y a seulement un type de *StandardClock*.
2. mobility: Actuellement, il y a seulement un type de *StandardMobility*.
3. physic: Actuellement, le seul système physique de supporter est: *Acceleration*.
4. Il peut y avoir plusieurs *recorder* selon quelles métriques vous avez de besoin.
    1. type: *CarPositionRecorder* ou CarSpeedRecorder.
    2. capture_freq: Indique la fréquence de capture en secondes.
```
  clock:
    type: StandardClock

  mobility:
    type: StandardMobility

  physic:
    type: Acceleration

  recorders:
    - type : CarPositionRecorder
      capture_freq: 6.0
    - type: CarSpeedRecorder
      capture_freq: 6.0
```

### Spawner ###
Actuellement, les *spawners* font partie des systèmes. 
Le *Spawner* va générer des entités à partir d'une liste de positions données et qui vont aller jusqu'aux positions de fin.
1. type: Indique quel type de spawner. Dans ce cas-ci, c'est un *spawner* de fréquence.
2. min: indique le nombre minimum d'entités à générer.
3. max: indique le nombre maximum d'entités à générer.
4. start_locations: Constitue la liste des positions de départ.
5. end_locations: Constitue la liste des positions de fin.
```
type: Frequency
    min: 1
    max: 3
    start_locations:
      - 5872015933
      ...
     end_locations:
      - 115710646
      ...
```
### Entities ###
Cette configuration permet d'instancier les entités souhaités. Présentement, il y a seulement deux types d'entités.

Pour une lumière, vous allez retrouver ces champs:dd
1. id Indique l'identifiant unique de l'entité.
2. type: Indique le type de l'entité en question.
3. light.initial_color: Indique la couleur actuellede la lumière.
4. light.max_green_time: Indique la durée maximale de la lumière verte en secondes.
5. light.max_yellow_time: Indique la durée maximale de la lumière jaune en secondes.
6. light.time: Indique le temps cyclique en secondes.
```
id: trafficlight
  type: trafficlight
  light:
    initial_color: RED
    max_green_time: 3
    max_yellow_time: 1
    time: 0
```

Pour une voiture, vous allez retrouver ces champs:
1. id Indique l'identifiant unique de l'entité.
2. type: Indique l'identifiant unique de l'entité.
3. position: Indique la position initial de la voiture.
4. speed: Indique la vitesse initiale de la voiture.
```
id: car0000000
type: vehicle
position: [[2004673453, 4267279738], 0.50]
speed: 2
```

## Visualiseur ##
Pour lancer le visualiseur web, vous devez spécifier un fichier de configuration en entré.

Le fichier de configuration doit avoir ces champs:
1. city: Indique la ville dans laquelle se produit la simulation.
2. logs: 
    1. directory: Indique le chemin du répertoire où les données calculées par le simulateur. En général, ce répertoire sera le même que celui indiqué dans le fichier de configuration du simulateur.
    2. metrics
        1. name: Indique le nom affiché dans le visualiseur web.
        2. logName: Indique l'identifiant utilisé dans le fichier de log. Il doit correspondre au même identifiant que celui calculés par le simulateur.
        3. unit: Indique l'identifiant des unités de cette métrique. Il doit correspondre aux mêmes unités que celles calculées par le simulateur.
        4. unitLabel: Indique l'identifiant affiché au visualiseur web.
3. legends: Indique le dégradé de la légende. Il doit y avoir au moins deux couleurs.
    1. Première couleur dans le dégradé
    2. Deuxième couleur ...
    3. Vous pouvez mettre autant de couleurs que vous pouvez.
```
city: "sherbrooke"
logs:
    directory: "./tmp_logs"
    metrics:
        - name: "Vitesse moy. Automobiles"
          logName: "car_speed.ndjson"
          unit: "MeterPerSecond"
          unitLabel: "Metres par seconde"
legends:
    - red: 0
      green: 255
      blue: 0
    - red: 255
      green: 255
      blue: 0
    - red: 255
      green: 0
      blue: 0
```

## Production ##
À partir de la racine du dossier **build**, vous devriez utiliser le script simumo.sh.