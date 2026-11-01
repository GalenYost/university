CREATE TABLE "Clients" (
    "Id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "Name" TEXT NOT NULL,
    "ActivityType" TEXT NOT NULL,
    "Address" TEXT NOT NULL,
    "Phone" TEXT NOT NULL
);

CREATE TABLE "Services" (
    "Id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "Name" TEXT NOT NULL,
    "Description" TEXT NOT NULL
);

CREATE TABLE "Deals" (
    "Id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "Amount" NUMERIC(18,2) NOT NULL,
    "Commission" NUMERIC(18,2) NOT NULL,
    "Description" TEXT NOT NULL,

    "ClientId" UUID NOT NULL REFERENCES "Clients"("Id") ON DELETE CASCADE,
    "ServiceId" UUID NOT NULL REFERENCES "Services"("Id") ON DELETE CASCADE
);
