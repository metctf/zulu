CREATE TABLE accounts (
    accountID int(11) NOT NULL AUTO_INCREMENT,
    studentID varchar(50) NOT NULL,
    firstName varchar(50) NOT NULL,
    lastName varchar(50) NOT NULL,
    password varchar(255) NOT NULL,
    origin varchar(50) NOT NULL,
    flagQuantity int(11) DEFAULT 0,
    accessLevel varchar(10) NOT NULL,
    creationTime timestamp NOT NULL,
    PRIMARY KEY (accountID)
);


CREATE TABLE flags (
    flagID int(11) NOT NULL AUTO_INCREMENT,
    challenge varchar(255) NOT NULL,
    challengeAuthor varchar(255) NOT NULL,
    flagString varchar(255) NOT NULL,
    points int(11) NOT NULL,
    creationTime timestamp NOT NULL,
    PRIMARY KEY (flagID)
);


CREATE TABLE accountFlags (
    fkAccountID int(11) NOT NULL,
    fkFlagID int(11) NOT NULL,
    creationTime timestamp NOT NULL,
    PRIMARY KEY (fkAccountId, fkFlagID),
    CONSTRAINT accountFlags_ibfk_1 FOREIGN KEY (fkAccountID) REFERENCES accounts (accountID),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagID)
);

INSERT INTO accounts(accountID, studentID, firstName, lastName, password, origin, accessLevel) 
VALUES ("1940", "121", "Winston", "Churchill", "neversurrender", "internal", "student");
