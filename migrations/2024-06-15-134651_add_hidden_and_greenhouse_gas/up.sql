-- Your SQL goes here
ALTER TABLE "food" ADD COLUMN "hidden" BOOL NOT NULL DEFAULT false;
ALTER TABLE "food" ADD COLUMN "greenhouse_gas" FLOAT4 NOT NULL DEFAULT 0;



