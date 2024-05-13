CREATE TABLE food_i18n (
  lang VARCHAR NOT NULL,
  food_id INT NOT NULL,
  name_translation VARCHAR NOT NULL,
  CONSTRAINT fk_food_i18n_food_id FOREIGN KEY (food_id)
        REFERENCES public.food(id)
        ON DELETE CASCADE,
  PRIMARY KEY(lang, food_id, name_translation)
)
