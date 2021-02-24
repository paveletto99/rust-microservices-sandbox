// "shippings" COLLECTION SCHEMA VALIDATION
db.createCollection("shippings", {
    validator: {
        $jsonSchema: {
            bsonType: "object",
            required: [ "_id", "code", "created_on", "timezone_offset", "schema_version" ],
            properties: {
                _id: {
                    bsonType: "binData",
                    description: "must be a UUID data type and is required"
                },
                code: {
                    bsonType : "string",
                    pattern : "^[a-z-A-Z]+[a-z-A-Z ]*$",
                    description: "must be a string and match the regular expression pattern"
                },
                created_on: {
                    bsonType : "date",
                    description: "must be an ISODate"
                },
                timezone_offset: {
                    bsonType : "number",
                    description: "must be an INT value for time zone offset in minutes"
                },
                schema_version: {
                    bsonType : "string",
                    pattern : "^[0-9]{1,2}\.[0-9]{1,2}\.[0-9]{1,2}$",
                    description: "must be a string and match the regular expression pattern"
                },
                children: { // VALIDATION ON MAPPED ITEMS: ARRAY OF UUID DATA TYPE - AT LEAST ONE - NO DUPLICATES
                    bsonType: "array",
                    description: "Children items for document",
                    minItems: 1,
                    items: { bsonType: "binData" },
                    uniqueItems: true
                }
            }
        }
    }
});
db.shippings.createIndex({ "code": 1 }, { unique: true });

// DUPLICATE INSERT ERROR ON UNIQUE "_id" FIELD THAT ACTS AS A "PRIMARY KEY" - UUID STORED AS BinData IS QUITE EFFICIENT FOR INDEX PERFORMANCE (https://docs.mongodb.com/manual/core/document/#the-id-field)
db.shippings.insert({ _id: UUID("6fbe947b-db18-4b02-ad8a-2f2284c6db37"), code: "daasdsadasdsa", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0" });
db.shippings.insert({ _id: UUID("6fbe947b-db18-4b02-ad8a-2f2284c6db37"), code: "daasdsadasdsa", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0" });

// DUPLICATE INSERT ERROR ON UNIQUE INDEX "code" FIELD
db.shippings.insert({ _id: UUID("6fbe947b-db18-4b02-ad8a-2f2284c6db38"), code: "daasdsadasdsa", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0" });

// VALIDATION ERROR
db.shippings.insert({ _id: UUID(), code: "" })
db.shippings.insert({ _id: UUID(), code: "   " })

// VALIDATION ON EMPTY "chidlren" ARRAY
db.shippings.insert({ _id: UUID("6fbe947b-db18-4b02-ad8a-2f2284c6db38"), code: "daasdsadasdsa", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [] });

// VALIDATION ON DUPLICATE ITEMS IN "chidlren" ARRAY
db.shippings.insert({ _id: UUID("6fbe947b-db18-4b02-ad8a-2f2284c6db38"), code: "daasdsadasdsa", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID("2c086408-7b0b-4465-a012-a33eef033621"), UUID("2c086408-7b0b-4465-a012-a33eef033621") ] });

// EMPTY COLLECTION
db.shippings.deleteMany({});

// CHECK UUID ON INSERT MANY
db.shippings.insertMany(
    [
        { _id: UUID(), code: "QWEIIWPUIWQUSLKERTIPLD", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID(), UUID() ] },
        { _id: UUID(), code: "ADGGASKLXWQUEWQEWQCSLD", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID(), UUID() ] },
        { _id: UUID(), code: "WQUTRORPEPSLDDFKLKHJTI", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID(), UUID() ] },
        { _id: UUID(), code: "LDDFKLPOINBVBVOPPOAADF", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID(), UUID() ] },
        { _id: UUID(), code: "HJERIAPFSJKLJEWPDCNVLL", created_on: ISODate(), timezone_offset: ISODate().getTimezoneOffset(), schema_version: "0.0.0", children: [ UUID(), UUID() ] }
    ]
);

// PRETTY PRINT RECORDS
db.shippings.find().pretty()
db.shippings.find({ _id: UUID("7f074ee8-0641-43e5-8ae0-f66758f2806a")}).pretty()
db.shippings.find({ children: [ UUID("2b09f615-2fd9-4c1f-b0eb-0249c5df5748"), UUID("18600fe9-ad4d-4f43-967e-d87fc97f986e") ]}).pretty()
db.shippings.find({ children: { $in : [ UUID("2b09f615-2fd9-4c1f-b0eb-0249c5df5748"), UUID("3698afc4-f0bf-4a6e-bbf0-52dd5159b1a6") ] }}).pretty()

db.createCollection("shippings02", {
    validator: {
        $jsonSchema: {
            bsonType: "object",
            properties: {
                amount: {
                    bsonType : "number"
                }
            }
        }
    }
});

db.shippings02.insertMany(
    [
        { amount: NumberDecimal(2.000) }
    ]
);
