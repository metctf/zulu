CREATE TABLE accounts (
    id varchar(255) NOT NULL PRIMARY KEY,
    username varchar(50) NOT NULL,
    firstname varchar(50) NOT NULL,
    lastname varchar(50) NOT NULL,
    password varchar(255),
    origin varchar(50) NOT NULL,
    solves int(11) UNSIGNED DEFAULT 0 NOT NULL,
    accesslevel varchar(10) NOT NULL,
    creationtime timestamp NOT NULL
);


CREATE TABLE challenges (
    id varchar(255) NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    author varchar(255) NOT NULL,
    flag varchar(255) NOT NULL,
    points int(11) UNSIGNED DEFAULT 0 NOT NULL,
    creationtime timestamp NOT NULL
);


CREATE TABLE accountFlags (
    fkaccountid varchar(255),
    fkflagid varchar(255),
    creationtime timestamp NOT NULL,
    CONSTRAINT accountflags_ibfk_1 FOREIGN KEY (fkaccountid) REFERENCES accounts (id),
    CONSTRAINT accountFlags_ibfk_2 FOREIGN KEY (fkFlagID) REFERENCES flags (id),
    PRIMARY KEY (fkAccountID, fkFlagID)
);

INSERT INTO accounts(accountid, username, firstname, lastname, password, origin, accesslevel) 
VALUES ("cd41294a-afb0-11df-bc9b-00241dd75637", "winston.churchill", "Winston", "Churchill", "neversurrender", "CMET", "user");
