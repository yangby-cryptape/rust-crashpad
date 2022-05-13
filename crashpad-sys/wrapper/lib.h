#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

extern bool start_crashpad(char* raw_handler,
                           char* raw_datadir,
                           char* raw_url,
                           char* annotations[],
                           int n_annotations);



#ifdef __cplusplus
}  /* end extern "C" */
#endif
