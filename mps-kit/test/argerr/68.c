/* 
TEST_HEADER
 id = $Id$
 summary = null thr_t to thread_reg
 language = c
 link = testlib.o
OUTPUT_SPEC
 assert = true
 assertfile P= mpsi.c
 assertcond = mps_thr_o != NULL
END_HEADER
*/

#include "testlib.h"
#include "arg.h"

void *stackpointer;

static void test(void)
{
 mps_arena_t arena;

 cdie(mps_arena_create(&arena, mps_arena_class_vm(), mmqaArenaSIZE), "create arena");

 cdie(mps_thread_reg(NULL, arena), "register thread");

}

int main(void)
{
 void *m;
 stackpointer=&m; /* hack to get stack pointer */

 easy_tramp(test);
 return 0;
}
