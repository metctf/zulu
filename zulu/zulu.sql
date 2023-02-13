-- Host: localhost    Database: zulu
-- Server version	10.9.2-MariaDB

DROP DATABASE IF EXISTS zulu;
CREATE DATABASE zulu;

GRANT ALL ON zulu.* TO zulu@localhost;

DROP TABLE IF EXISTS zulu.accounts;
CREATE TABLE zulu.accounts (
    accountid uuid DEFAULT UUID() PRIMARY KEY,
    username varchar(50) NOT NULL,
    firstname varchar(50) NOT NULL,
    lastname varchar(50) NOT NULL,
    password varchar(255),
	origin varchar(50) NOT NULL,
    flagquantity int(11) UNSIGNED DEFAULT 0 NOT NULL,
    accesslevel varchar(10) NOT NULL,
	creationtime timestamp NOT NULL
); 


DROP TABLE IF EXISTS zulu.flags;
CREATE TABLE zulu.flags (
    flagid uuid DEFAULT uuid() PRIMARY KEY,
    challenge varchar(255) NOT NULL,
    challengeauthor varchar(255) NOT NULL,
    flagstring varchar(255) NOT NULL,
    points int(11) UNSIGNED DEFAULT 0 NOT NULL,
	creationtime timestamp NOT NULL
); 


DROP TABLE IF EXISTS zulu.accountFlags;
CREATE TABLE zulu.accountFlags (
    fkaccountid uuid DEFAULT uuid(),
    fkflagid uuid DEFAULT uuid(),
    creationtime timestamp NOT NULL,
    CONSTRAINT accountflags_ibfk_1 FOREIGN KEY (fkaccountid) REFERENCES accounts (accountid),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagid),
    PRIMARY KEY(fkaccountid, fkflagid)
);
