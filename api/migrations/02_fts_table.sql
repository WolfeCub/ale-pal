CREATE VIRTUAL TABLE beverage_search USING FTS5(
    beverage_id,
    name,
    kind,
    producer,
    rating,
    description
);

CREATE TRIGGER beverage_search_before_update
BEFORE UPDATE ON beverage BEGIN
    DELETE FROM beverage_search WHERE beverage_id = old.beverage_id;
END;

CREATE TRIGGER beverage_search_before_delete
BEFORE DELETE ON beverage BEGIN
    DELETE FROM beverage_search WHERE beverage_id = old.beverage_id;
END;

CREATE TRIGGER beverage_search_after_update
AFTER UPDATE ON beverage BEGIN
    INSERT INTO beverage_search(
        beverage_id,
        name,
        kind,
        producer,
        rating,
        description
    )
    SELECT beverage_id,
           beverage.name as name,
           kind.name as kind,
           producer.name as producer,
           rating,
           description
    FROM beverage
    INNER JOIN kind ON kind.kind_id = beverage.kind_id
    INNER JOIN producer ON producer.producer_id = beverage.producer_id
    WHERE beverage_id = new.beverage_id;
END;

CREATE TRIGGER beverage_search_after_insert
AFTER INSERT ON beverage BEGIN
    INSERT INTO beverage_search(
        beverage_id,
        name,
        kind,
        producer,
        rating,
        description
    )
    SELECT beverage_id,
           beverage.name as name,
           kind.name as kind,
           producer.name as producer,
           rating,
           description
    FROM beverage
    INNER JOIN kind ON kind.kind_id = beverage.kind_id
    INNER JOIN producer ON producer.producer_id = beverage.producer_id
    WHERE beverage_id = new.beverage_id;
END;
