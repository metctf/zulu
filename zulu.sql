-- Host: localhost    Database: zulu
-- Server version	10.9.2-MariaDB

DROP DATABASE IF EXISTS zulu;
CREATE DATABASE zulu;

GRANT ALL ON zulu.* TO zulu@localhost;

DROP TABLE IF EXISTS zulu.accounts;
CREATE TABLE zulu.accounts (
    accountid int(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    studentid varchar(50) NOT NULL,
    firstname varchar(50) NOT NULL,
    lastname varchar(50) NOT NULL,
    password varchar(255) NOT NULL,
	origin varchar(50) NOT NULL,
    flagquantity int(11) UNSIGNED NOT NULL DEFAULT 0,
    accesslevel varchar(10) NOT NULL,
	creationtime timestamp NOT NULL,
    PRIMARY KEY (accountid)
); 


DROP TABLE IF EXISTS zulu.flags;
CREATE TABLE zulu.flags (
    flagID int(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    challenge varchar(255) NOT NULL,
    challengeAuthor varchar(255) NOT NULL,
    flagString varchar(255) NOT NULL,
    points int(11) UNSIGNED NOT NULL,
	creationTime timestamp NOT NULL,
    PRIMARY KEY (flagID)
); 


DROP TABLE IF EXISTS zulu.accountFlags;
CREATE TABLE zulu.accountFlags (
    fkAccountID int(11) UNSIGNED NOT NULL,
    fkFlagID int(11) UNSIGNED NOT NULL,
    creationTime timestamp NOT NULL,
    PRIMARY KEY (fkAccountId, fkFlagID),
    CONSTRAINT accountFlags_ibfk_1 FOREIGN KEY (fkAccountID) REFERENCES accounts (accountID),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagID)
);
