-- Your SQL goes here

CREATE TABLE types
(
    address          bytea          NOT NULL,
    u256             numeric(78, 0) NOT NULL,
    block_number     int8           NOT NULL,
    tx_hash          bytea          NOT NULL,
    optional_address bytea,
    optional_u256    numeric(78, 0),
    primary key (address, u256)
);

INSERT INTO types (address, u256, block_number, tx_hash, optional_address, optional_u256)
VALUES ('\x92BE2F02C94D214F8D38ECE700385471D9A66C0A', '9999999999999999999999999999999999999999999999', 1,
        '\xb44c4e99de65f6a5f4a2162a76241cf858c09ff218f3023a3ac03acc17fea885', NULL, NULL);
