CREATE TABLE accounts (
    accountID uuid DEFAULT uuid() PRIMARY KEY,
    username varchar(50) NOT NULL,
    firstName varchar(50) NOT NULL,
    lastName varchar(50) NOT NULL,
    password varchar(255),
    origin varchar(50) NOT NULL,
    flagQuantity int(11) UNSIGNED DEFAULT 0 NOT NULL,
    accessLevel varchar(10) NOT NULL,
    creationTime timestamp NOT NULL
);


CREATE TABLE flags (
    flagID uuid DEFAULT uuid() PRIMARY KEY,
    challenge varchar(255) NOT NULL,
    challengeAuthor varchar(255) NOT NULL,
    flagString varchar(255) NOT NULL,
    points int(11) UNSIGNED DEFAULT 0 NOT NULL,
    creationTime timestamp NOT NULL
);


CREATE TABLE accountFlags (
    fkAccountID uuid DEFAULT uuid(),
    fkFlagID uuid DEFAULT uuid(),
    creationTime timestamp NOT NULL,
    CONSTRAINT accountFlags_ibfk_1 FOREIGN KEY (fkAccountID) REFERENCES accounts (accountID),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagID),
    PRIMARY KEY (fkAccountID, fkFlagID)
);

INSERT INTO accounts(accountID, username, firstName, lastName, password, origin, accessLevel) 
VALUES ("cd41294a-afb0-11df-bc9b-00241dd75637", "winston.churchill", "Winston", "Churchill", "neversurrender", "internal", "user");
