ALTER TABLE "food" DROP COLUMN "comment_expert";
ALTER TABLE "food" DROP COLUMN "greenhouse_gas_comment_expert";
ALTER TABLE "food" ALTER "comment" SET NOT NULL;
