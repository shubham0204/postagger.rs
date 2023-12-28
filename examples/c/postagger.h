#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PerceptronTagger PerceptronTagger;

typedef struct CTag {
  const uint8_t *word;
  const uint8_t *tag;
  float conf;
} CTag;

typedef struct TagResults {
  const struct CTag *tags;
  uintptr_t num_tags;
} TagResults;

struct PerceptronTagger *tagger_create(const char *weights_filepath,
                                       const char *classes_filepath,
                                       const char *tags_filepath);

const struct TagResults *tagger_annotate(struct PerceptronTagger *tagger_ptr, const char *sentence);

void tagger_release(struct PerceptronTagger *tagger_ptr);
