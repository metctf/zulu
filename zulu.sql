-- Host: localhost    Database: zulu
-- Server version	10.9.2-MariaDB

DROP DATABASE IF EXISTS zulu;
CREATE DATABASE zulu;

GRANT ALL ON zulu.* TO zulu@localhost;

DROP TABLE IF EXISTS zulu.accounts;
CREATE TABLE zulu.accounts (
    accountID int(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    studentID varchar(50) NOT NULL,
    firstName varchar(50) NOT NULL,
    lastName varchar(50) NOT NULL,
    password varchar(255) NOT NULL,
	origin varchar(50) NOT NULL,
    flagQuantity int(11) UNSIGNED DEFAULT 0,
    accessLevel varchar(10) NOT NULL,
	creationTime timestamp NOT NULL,
    PRIMARY KEY (accountID)
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
