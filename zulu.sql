-- Host: localhost    Database: zulu
-- Server version	10.9.2-MariaDB

DROP DATABASE IF EXISTS zulu;
CREATE DATABASE zulu;

GRANT ALL ON zulu.* TO zulu@localhost;

DROP TABLE IF EXISTS zulu.accounts;
CREATE TABLE zulu.accounts (
  accountID int(11) NOT NULL AUTO_INCREMENT,
  studentID varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  firstName varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  lastName varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  password varchar(255) COLLATE utf8mb4_unicode_ci,
	origin varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  flagQuantity int(11) NOT NULL,
	timestamp timestamp NOT NULL,
  PRIMARY KEY (accountID)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


DROP TABLE IF EXISTS  zulu.flags;
CREATE TABLE zulu.flags (
  flagID int(11) NOT NULL AUTO_INCREMENT,
  challenge varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  challengeAuthor varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  flagString varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
  points int(11) NOT NULL,
	timestamp timestamp NOT NULL,
  PRIMARY KEY (flagID)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


DROP TABLE IF EXISTS zulu.accountFlags;
CREATE TABLE zulu.accountFlags (
  fkAccountID int(11) NOT NULL,
  fkFlagID int(11) NOT NULL,
	timestamp timestamp NOT NULL,
  PRIMARY KEY (fkAccountId, fkFlagID),
  CONSTRAINT accountFlags_ibfk_1 FOREIGN KEY (fkAccountID) REFERENCES accounts (accountID),
  CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagID)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
