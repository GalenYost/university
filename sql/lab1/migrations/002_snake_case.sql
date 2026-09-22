ALTER TABLE "Clients" RENAME TO clients;
ALTER TABLE "Services" RENAME TO services;
ALTER TABLE "Deals" RENAME TO deals;

ALTER TABLE clients RENAME COLUMN "Id" TO id;
ALTER TABLE clients RENAME COLUMN "Name" TO name;
ALTER TABLE clients RENAME COLUMN "ActivityType" TO activity_type;
ALTER TABLE clients RENAME COLUMN "Address" TO address;
ALTER TABLE clients RENAME COLUMN "Phone" TO phone;

ALTER TABLE services RENAME COLUMN "Id" TO id;
ALTER TABLE services RENAME COLUMN "Name" TO name;
ALTER TABLE services RENAME COLUMN "Description" TO description;

ALTER TABLE deals RENAME COLUMN "Id" TO id;
ALTER TABLE deals RENAME COLUMN "ClientId" TO client_id;
ALTER TABLE deals RENAME COLUMN "ServiceId" TO service_id;
ALTER TABLE deals RENAME COLUMN "Amount" TO amount;
ALTER TABLE deals RENAME COLUMN "Commission" TO commission;
ALTER TABLE deals RENAME COLUMN "Description" TO description;

ALTER TABLE clients RENAME CONSTRAINT "Clients_pkey" TO clients_pkey;
ALTER TABLE services RENAME CONSTRAINT "Services_pkey" TO services_pkey;
ALTER TABLE deals RENAME CONSTRAINT "Deals_pkey" TO deals_pkey;
ALTER TABLE deals RENAME CONSTRAINT "Deals_ClientId_fkey" TO deals_client_id_fkey;
ALTER TABLE deals RENAME CONSTRAINT "Deals_ServiceId_fkey" TO deals_service_id_fkey;