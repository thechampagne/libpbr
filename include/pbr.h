/*
 * Copyright (c) 2023 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#ifndef __PBR_H__
#define __PBR_H__

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    PBR_HANDLE_STDERR,
    PBR_HANDLE_STDOUT
} pbr_handle_t;

typedef struct {
  void* multi_bar;
  pbr_handle_t handle;
} pbr_multi_bar_t;

typedef struct {
  void* progress_bar;
  pbr_handle_t handle;
} pbr_progress_bar_t;

typedef enum {
    PBR_DURATION_MICROS,
    PBR_DURATION_MILLIS,
    PBR_DURATION_NANOS,
    PBR_DURATION_SECS,
} pbr_duration_t;

typedef enum {
    PBR_UNITS_DEFAULT,
    PBR_UNITS_BYTES,
} pbr_units_t;

extern pbr_multi_bar_t pbr_multi_bar_new();

extern pbr_multi_bar_t pbr_multi_bar_on(pbr_handle_t handle);

extern void pbr_multi_bar_println(const pbr_multi_bar_t* multi_bar, const char* s);

extern pbr_progress_bar_t pbr_multi_bar_create_bar(const pbr_multi_bar_t* multi_bar, uint64_t total);

extern void pbr_multi_bar_listen(const pbr_multi_bar_t* multi_bar);

extern pbr_progress_bar_t pbr_progress_bar_new(uint64_t total);

extern pbr_progress_bar_t pbr_progress_bar_on(pbr_handle_t handle, uint64_t total);

extern void pbr_progress_bar_set_units(pbr_progress_bar_t* progress_bar, pbr_units_t units);

extern void pbr_progress_bar_format(pbr_progress_bar_t* progress_bar, const char* fmt);

extern void pbr_progress_bar_message(pbr_progress_bar_t* progress_bar, const char* message);

extern void pbr_progress_bar_tick_format(pbr_progress_bar_t* progress_bar, const char* tick_fmt);

extern void pbr_progress_bar_set_width(pbr_progress_bar_t* progress_bar, size_t w);

extern void pbr_progress_bar_set_max_refresh_rate(pbr_progress_bar_t* progress_bar, pbr_duration_t duration, uint64_t w);

extern void pbr_progress_bar_tick(pbr_progress_bar_t* progress_bar);

extern uint64_t pbr_progress_bar_add(pbr_progress_bar_t* progress_bar, uint64_t i);

extern void pbr_progress_bar_set(pbr_progress_bar_t* progress_bar, uint64_t i);

extern uint64_t pbr_progress_bar_inc(pbr_progress_bar_t* progress_bar);

extern void pbr_progress_bar_reset_start_time(pbr_progress_bar_t* progress_bar);

extern void pbr_progress_bar_finish(pbr_progress_bar_t* progress_bar);

extern void pbr_progress_bar_finish_print(pbr_progress_bar_t* progress_bar, const char* s);

extern void pbr_progress_bar_finish_println(pbr_progress_bar_t* progress_bar, const char* s);
  
#ifdef __cplusplus
}
#endif

#endif // __PBR_H__
