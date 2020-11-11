#ifndef _TESTLIB_H
#define _TESTLIB_H


  struct CGraph;

  struct HandlesIter_t {
    bool finished;
    /* void* _iter; */
  };


#ifdef __cplusplus
extern "C"{
#endif

  struct CGraph* load_hashgraph(uint8_t const* path, size_t path_len);

  void free_hashgraph(struct CGraph* g);

  struct HandlesIter_t* graph_handles(struct CGraph* g);

  void free_handles_iter(struct HandlesIter_t* g);

  uint64_t handles_next(struct HandlesIter_t* h);


#ifdef __cplusplus
}
#endif
#endif
