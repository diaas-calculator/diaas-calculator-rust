CREATE TABLE mix (
  id INTEGER PRIMARY KEY,
  visibility SMALLINT NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR
);

CREATE TABLE mix_food (
  mix_id INTEGER,
  food_id INTEGER,
  food_weight REAL NOT NULL,
  CONSTRAINT fk_mix_food_mix FOREIGN KEY (mix_id)
        REFERENCES public.mix(id)
        ON DELETE CASCADE,
  CONSTRAINT fk_mix_food_food FOREIGN KEY (food_id)
        REFERENCES public.food(id)
        ON DELETE CASCADE,
  PRIMARY KEY(mix_id, food_id)
);
