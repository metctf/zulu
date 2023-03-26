-- Host: localhost    Database: zulu
-- Server version	10.9.2-MariaDB

DROP DATABASE IF EXISTS zulu;
CREATE DATABASE zulu;

GRANT ALL ON zulu.* TO zulu@localhost;

DROP TABLE IF EXISTS zulu.accounts;
CREATE TABLE zulu.accounts (
    id varchar(255) NOT NULL PRIMARY KEY,
    username varchar(50) NOT NULL,
    firstname varchar(50) NOT NULL,
    lastname varchar(50) NOT NULL,
    password varchar(255),
	origin varchar(50) NOT NULL,
    solves int(11) UNSIGNED DEFAULT 0 NOT NULL,
    accesslevel varchar(10) NOT NULL,
	creationtime timestamp
); 


DROP TABLE IF EXISTS zulu.challenges;
CREATE TABLE zulu.challenges (
    id varchar(255) NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    author varchar(255) NOT NULL,
    flag varchar(255) NOT NULL,
    points int(11) UNSIGNED DEFAULT 0 NOT NULL,
	creationtime timestamp
); 


DROP TABLE IF EXISTS zulu.accountFlags;
CREATE TABLE zulu.accountFlags (
    fkaccountid varchar(255),
    fkflagid varchar(255),
    creationtime timestamp NOT NULL,
    CONSTRAINT accountflags_ibfk_1 FOREIGN KEY (fkaccountid) REFERENCES accounts (id),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkflagid) REFERENCES challenges (id),
    PRIMARY KEY(fkaccountid, fkflagid)
);
