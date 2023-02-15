CREATE TABLE accounts (
    accountid varchar(255) NOT NULL PRIMARY KEY,
    username varchar(50) NOT NULL,
    firstname varchar(50) NOT NULL,
    lastname varchar(50) NOT NULL,
    password varchar(255),
    origin varchar(50) NOT NULL,
    flagquantity int(11) UNSIGNED DEFAULT 0 NOT NULL,
    accesslevel varchar(10) NOT NULL,
    creationtime timestamp NOT NULL
);


CREATE TABLE flags (
    flagid varchar(255) NOT NULL PRIMARY KEY,
    challenge varchar(255) NOT NULL,
    challengeauthor varchar(255) NOT NULL,
    flagstring varchar(255) NOT NULL,
    points int(11) UNSIGNED DEFAULT 0 NOT NULL,
    creationtime timestamp NOT NULL
);


CREATE TABLE accountFlags (
    fkaccountid varchar(255),
    fkflagid varchar(255),
    creationtime timestamp NOT NULL,
    CONSTRAINT accountflags_ibfk_1 FOREIGN KEY (fkaccountid) REFERENCES accounts (accountid),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (flagid),
    PRIMARY KEY (fkAccountID, fkFlagID)
);

INSERT INTO accounts(accountid, username, firstname, lastname, password, origin, accesslevel) 
VALUES ("cd41294a-afb0-11df-bc9b-00241dd75637", "winston.churchill", "Winston", "Churchill", "neversurrender", "internal", "user");
