# libpbr

[![](https://img.shields.io/github/v/tag/thechampagne/libpbr?label=version)](https://github.com/thechampagne/libpbr/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libpbr)](https://github.com/thechampagne/libpbr/blob/main/LICENSE)

Console progress bar for **C**.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libpbr.git
```
#### 2. Navigate to the root
```
cd libpbr
```
#### 3.1 Build the project
```
cargo build
```
#### 3.2 Run tests
```
cargo test
```

### API
```c
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

pbr_multi_bar_t pbr_multi_bar_new();

pbr_multi_bar_t pbr_multi_bar_on(pbr_handle_t handle);

void pbr_multi_bar_println(const pbr_multi_bar_t* multi_bar, const char* s);

pbr_progress_bar_t pbr_multi_bar_create_bar(const pbr_multi_bar_t* multi_bar, uint64_t total);

void pbr_multi_bar_listen(const pbr_multi_bar_t* multi_bar);

pbr_progress_bar_t pbr_progress_bar_new(uint64_t total);

pbr_progress_bar_t pbr_progress_bar_on(pbr_handle_t handle, uint64_t total);

void pbr_progress_bar_set_units(pbr_progress_bar_t* progress_bar, pbr_units_t units);

void pbr_progress_bar_format(pbr_progress_bar_t* progress_bar, const char* fmt);

void pbr_progress_bar_message(pbr_progress_bar_t* progress_bar, const char* message);

void pbr_progress_bar_tick_format(pbr_progress_bar_t* progress_bar, const char* tick_fmt);

void pbr_progress_bar_set_width(pbr_progress_bar_t* progress_bar, size_t w);

void pbr_progress_bar_set_max_refresh_rate(pbr_progress_bar_t* progress_bar, pbr_duration_t duration, uint64_t w);

void pbr_progress_bar_tick(pbr_progress_bar_t* progress_bar);

uint64_t pbr_progress_bar_add(pbr_progress_bar_t* progress_bar, uint64_t i);

void pbr_progress_bar_set(pbr_progress_bar_t* progress_bar, uint64_t i);

uint64_t pbr_progress_bar_inc(pbr_progress_bar_t* progress_bar);

void pbr_progress_bar_reset_start_time(pbr_progress_bar_t* progress_bar);

void pbr_progress_bar_finish(pbr_progress_bar_t* progress_bar);

void pbr_progress_bar_finish_print(pbr_progress_bar_t* progress_bar, const char* s);

void pbr_progress_bar_finish_println(pbr_progress_bar_t* progress_bar, const char* s);
```

### References
 - [pbr](https://github.com/a8m/pb)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libpbr/blob/main/LICENSE).
