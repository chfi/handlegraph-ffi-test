#ifndef _TESTLIB_H
#define _TESTLIB_H


  struct CGraph;

  struct HandlesIter_t;


#ifdef __cplusplus
extern "C"{
#endif

  struct CGraph* load_hashgraph();

  struct HandlesIter_t* graph_handles(struct CGraph* g);

  uint64_t handles_next(struct HandlesIter_t* h);


#ifdef __cplusplus
}
#endif
#endif
