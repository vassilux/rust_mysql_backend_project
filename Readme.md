# Rust Backend Development

Ce projet contient des exemples de développement backend en Rust utilisant différentes bases de données.

## Structure du projet

- `docker_mysql/`: docker configuration MYSQL.
	- `docker-compose.yml`: Configuration Docker pour les services de base de données.
- `mysql_project/`: Projet Rust utilisant MySQL.
- `mssql_project/`: Projet Rust utilisant MSSQL.
- `oracle_project/`: Projet Rust utilisant Oracle.


## Instructions

### Démarrer les services de base de données

Pour démarrer les services de base de données définis dans `docker-compose.yml`, exécutez la commande suivante à la racine du projet :

```bash
docker-compose up -d
```

### Purger les anciens conteneurs et volumes Docker

Pour purger les anciens conteneurs et leurs volumes associés, suivez ces étapes :

### Arrêter et supprimer les conteneurs existants

Utilisez la commande suivante pour arrêter et supprimer tous les conteneurs en cours d'exécution :

```bash
docker-compose down

docker volume rm $(docker volume ls -q)


docker volume rm mysql_data

docker-compose down
docker volume rm mysql_data
docker-compose up -d
```
