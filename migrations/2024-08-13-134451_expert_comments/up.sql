ALTER TABLE "food" ADD COLUMN "comment_expert" VARCHAR;
ALTER TABLE "food" ADD COLUMN "greenhouse_gas_comment_expert" VARCHAR;
ALTER TABLE "food" ALTER "comment" DROP NOT NULL;
