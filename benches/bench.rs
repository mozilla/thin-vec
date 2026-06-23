use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use thin_vec::{thin_vec, ThinVec};

// ============================================================================
// Construction Benchmarks
// ============================================================================

fn bench_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction");

    group.bench_function("new_empty", |b| {
        b.iter(|| {
            let v: ThinVec<u64> = ThinVec::new();
            black_box(v)
        })
    });

    group.bench_function("vec_new_empty", |b| {
        b.iter(|| {
            let v: Vec<u64> = Vec::new();
            black_box(v)
        })
    });

    for size in [0, 1, 10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("with_capacity", size), size, |b, &size| {
            b.iter(|| {
                let v: ThinVec<u64> = ThinVec::with_capacity(size);
                black_box(v)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("vec_with_capacity", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let v: Vec<u64> = Vec::with_capacity(size);
                    black_box(v)
                })
            },
        );
    }

    group.bench_function("macro_small", |b| {
        b.iter(|| {
            let v = thin_vec![1u64, 2, 3, 4, 5];
            black_box(v)
        })
    });

    group.bench_function("vec_macro_small", |b| {
        b.iter(|| {
            let v = vec![1u64, 2, 3, 4, 5];
            black_box(v)
        })
    });

    group.bench_function("macro_repeat", |b| {
        b.iter(|| {
            let v = thin_vec![0u64; 100];
            black_box(v)
        })
    });

    group.bench_function("vec_macro_repeat", |b| {
        b.iter(|| {
            let v = vec![0u64; 100];
            black_box(v)
        })
    });

    group.finish();
}

// ============================================================================
// Push Benchmarks
// ============================================================================

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("push");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("push_no_reserve", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: ThinVec<u64> = ThinVec::new();
                    for i in 0..size {
                        v.push(i as u64);
                    }
                    black_box(v)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_push_no_reserve", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<u64> = Vec::new();
                    for i in 0..size {
                        v.push(i as u64);
                    }
                    black_box(v)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("push_with_reserve", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: ThinVec<u64> = ThinVec::with_capacity(size);
                    for i in 0..size {
                        v.push(i as u64);
                    }
                    black_box(v)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_push_with_reserve", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<u64> = Vec::with_capacity(size);
                    for i in 0..size {
                        v.push(i as u64);
                    }
                    black_box(v)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Pop Benchmarks
// ============================================================================

fn bench_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("pop", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    while v.pop().is_some() {}
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_pop", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                |mut v| {
                    while v.pop().is_some() {}
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

// ============================================================================
// Insert Benchmarks
// ============================================================================

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("insert_front", size), size, |b, &size| {
            b.iter(|| {
                let mut v: ThinVec<u64> = ThinVec::new();
                for i in 0..size {
                    v.insert(0, i as u64);
                }
                black_box(v)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("vec_insert_front", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<u64> = Vec::new();
                    for i in 0..size {
                        v.insert(0, i as u64);
                    }
                    black_box(v)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("insert_middle", size), size, |b, &size| {
            b.iter(|| {
                let mut v: ThinVec<u64> = ThinVec::new();
                for i in 0..size {
                    let mid = v.len() / 2;
                    v.insert(mid, i as u64);
                }
                black_box(v)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("vec_insert_middle", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut v: Vec<u64> = Vec::new();
                    for i in 0..size {
                        let mid = v.len() / 2;
                        v.insert(mid, i as u64);
                    }
                    black_box(v)
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Remove Benchmarks
// ============================================================================

fn bench_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("remove_front", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    while !v.is_empty() {
                        v.remove(0);
                    }
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_remove_front", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        while !v.is_empty() {
                            v.remove(0);
                        }
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("swap_remove", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    while !v.is_empty() {
                        v.swap_remove(0);
                    }
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_swap_remove", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        while !v.is_empty() {
                            v.swap_remove(0);
                        }
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Iteration Benchmarks
// ============================================================================

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration");

    for size in [100, 1000, 10000, 100000].iter() {
        let thin_vec: ThinVec<u64> = (0..*size).collect();
        let std_vec: Vec<u64> = (0..*size).collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("iter", size), &thin_vec, |b, v| {
            b.iter(|| {
                let sum: u64 = v.iter().sum();
                black_box(sum)
            })
        });

        group.bench_with_input(BenchmarkId::new("vec_iter", size), &std_vec, |b, v| {
            b.iter(|| {
                let sum: u64 = v.iter().sum();
                black_box(sum)
            })
        });

        group.bench_with_input(BenchmarkId::new("iter_mut", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).collect::<ThinVec<u64>>(),
                |mut v| {
                    for x in v.iter_mut() {
                        *x += 1;
                    }
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_iter_mut", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).collect::<Vec<u64>>(),
                |mut v| {
                    for x in v.iter_mut() {
                        *x += 1;
                    }
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("into_iter", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).collect::<ThinVec<u64>>(),
                |v| {
                    let sum: u64 = v.into_iter().sum();
                    black_box(sum)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_into_iter", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).collect::<Vec<u64>>(),
                |v| {
                    let sum: u64 = v.into_iter().sum();
                    black_box(sum)
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

// ============================================================================
// Clone Benchmarks
// ============================================================================

fn bench_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");

    for size in [0, 10, 100, 1000, 10000].iter() {
        let thin_vec: ThinVec<u64> = (0..*size).collect();
        let std_vec: Vec<u64> = (0..*size).collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("clone", size), &thin_vec, |b, v| {
            b.iter(|| black_box(v.clone()))
        });

        group.bench_with_input(BenchmarkId::new("vec_clone", size), &std_vec, |b, v| {
            b.iter(|| black_box(v.clone()))
        });
    }

    group.finish();
}

// ============================================================================
// Extend Benchmarks
// ============================================================================

fn bench_extend(c: &mut Criterion) {
    let mut group = c.benchmark_group("extend");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("extend_iter", size), size, |b, &size| {
            b.iter_batched(
                || ThinVec::<u64>::new(),
                |mut v| {
                    v.extend(0..size as u64);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_extend_iter", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || Vec::<u64>::new(),
                    |mut v| {
                        v.extend(0..size as u64);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("extend_from_slice", size),
            size,
            |b, &size| {
                let data: Vec<u64> = (0..size as u64).collect();
                b.iter_batched(
                    || ThinVec::<u64>::new(),
                    |mut v| {
                        v.extend_from_slice(&data);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_extend_from_slice", size),
            size,
            |b, &size| {
                let data: Vec<u64> = (0..size as u64).collect();
                b.iter_batched(
                    || Vec::<u64>::new(),
                    |mut v| {
                        v.extend_from_slice(&data);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Drain Benchmarks
// ============================================================================

fn bench_drain(c: &mut Criterion) {
    let mut group = c.benchmark_group("drain");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("drain_all", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    let drained: ThinVec<_> = v.drain(..).collect();
                    black_box((v, drained))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_drain_all", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                |mut v| {
                    let drained: Vec<_> = v.drain(..).collect();
                    black_box((v, drained))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("drain_half", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    let half = v.len() / 2;
                    let drained: ThinVec<_> = v.drain(..half).collect();
                    black_box((v, drained))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_drain_half", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        let half = v.len() / 2;
                        let drained: Vec<_> = v.drain(..half).collect();
                        black_box((v, drained))
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Splice Benchmarks
// ============================================================================

fn bench_splice(c: &mut Criterion) {
    let mut group = c.benchmark_group("splice");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("splice_replace_same", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                    |mut v| {
                        let mid = v.len() / 2;
                        let replacement: Vec<u64> = (100..100 + (size / 4) as u64).collect();
                        let _: ThinVec<_> = v.splice(mid..mid + size / 4, replacement).collect();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_splice_replace_same", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        let mid = v.len() / 2;
                        let replacement: Vec<u64> = (100..100 + (size / 4) as u64).collect();
                        let _: Vec<_> = v.splice(mid..mid + size / 4, replacement).collect();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Truncate and Clear Benchmarks
// ============================================================================

fn bench_truncate_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("truncate_clear");

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("clear", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.clear();
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_clear", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                |mut v| {
                    v.clear();
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("truncate_half", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.truncate(size / 2);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_truncate_half", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        v.truncate(size / 2);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Reserve and Shrink Benchmarks
// ============================================================================

fn bench_reserve_shrink(c: &mut Criterion) {
    let mut group = c.benchmark_group("reserve_shrink");

    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("reserve", size), size, |b, &size| {
            b.iter_batched(
                || ThinVec::<u64>::new(),
                |mut v| {
                    v.reserve(size);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_reserve", size), size, |b, &size| {
            b.iter_batched(
                || Vec::<u64>::new(),
                |mut v| {
                    v.reserve(size);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("shrink_to_fit", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let mut v: ThinVec<u64> = ThinVec::with_capacity(size * 2);
                    v.extend(0..size as u64);
                    v
                },
                |mut v| {
                    v.shrink_to_fit();
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_shrink_to_fit", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let mut v: Vec<u64> = Vec::with_capacity(size * 2);
                        v.extend(0..size as u64);
                        v
                    },
                    |mut v| {
                        v.shrink_to_fit();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Retain Benchmarks
// ============================================================================

fn bench_retain(c: &mut Criterion) {
    let mut group = c.benchmark_group("retain");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("retain_half", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.retain(|x| x % 2 == 0);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_retain_half", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        v.retain(|x| x % 2 == 0);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("retain_none", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.retain(|_| false);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_retain_none", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        v.retain(|_| false);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("retain_all", size), size, |b, &size| {
            b.iter_batched(
                || (0..size).map(|i| i as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.retain(|_| true);
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_retain_all", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size).map(|i| i as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        v.retain(|_| true);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Dedup Benchmarks
// ============================================================================

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // All duplicates
        group.bench_with_input(
            BenchmarkId::new("dedup_all_same", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || thin_vec![42u64; size],
                    |mut v| {
                        v.dedup();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_dedup_all_same", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || vec![42u64; size],
                    |mut v| {
                        v.dedup();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        // No duplicates
        group.bench_with_input(BenchmarkId::new("dedup_no_dups", size), size, |b, &size| {
            b.iter_batched(
                || (0..size as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    v.dedup();
                    black_box(v)
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("vec_dedup_no_dups", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size as u64).collect::<Vec<u64>>(),
                    |mut v| {
                        v.dedup();
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// ============================================================================
// Split Off and Append Benchmarks
// ============================================================================

fn bench_split_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("split_append");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("split_off", size), size, |b, &size| {
            b.iter_batched(
                || (0..size as u64).collect::<ThinVec<u64>>(),
                |mut v| {
                    let split = v.split_off(size / 2);
                    black_box((v, split))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_split_off", size), size, |b, &size| {
            b.iter_batched(
                || (0..size as u64).collect::<Vec<u64>>(),
                |mut v| {
                    let split = v.split_off(size / 2);
                    black_box((v, split))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("append", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let v1: ThinVec<u64> = (0..size as u64 / 2).collect();
                    let v2: ThinVec<u64> = (size as u64 / 2..size as u64).collect();
                    (v1, v2)
                },
                |(mut v1, mut v2)| {
                    v1.append(&mut v2);
                    black_box((v1, v2))
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("vec_append", size), size, |b, &size| {
            b.iter_batched(
                || {
                    let v1: Vec<u64> = (0..size as u64 / 2).collect();
                    let v2: Vec<u64> = (size as u64 / 2..size as u64).collect();
                    (v1, v2)
                },
                |(mut v1, mut v2)| {
                    v1.append(&mut v2);
                    black_box((v1, v2))
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

// ============================================================================
// Memory Footprint Comparison
// ============================================================================

fn bench_memory_footprint(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_footprint");

    // Benchmark empty vec storage in a larger container
    group.bench_function("option_thinvec_size", |b| {
        b.iter(|| {
            let v: Option<ThinVec<u64>> = None;
            black_box(std::mem::size_of_val(&v))
        })
    });

    group.bench_function("option_vec_size", |b| {
        b.iter(|| {
            let v: Option<Vec<u64>> = None;
            black_box(std::mem::size_of_val(&v))
        })
    });

    // Benchmark vec of empty vecs
    group.bench_function("vec_of_empty_thinvecs", |b| {
        b.iter(|| {
            let v: Vec<ThinVec<u64>> = (0..1000).map(|_| ThinVec::new()).collect();
            black_box(v)
        })
    });

    group.bench_function("vec_of_empty_vecs", |b| {
        b.iter(|| {
            let v: Vec<Vec<u64>> = (0..1000).map(|_| Vec::new()).collect();
            black_box(v)
        })
    });

    group.finish();
}

// ============================================================================
// Zero-Sized Type Benchmarks
// ============================================================================

fn bench_zst(c: &mut Criterion) {
    let mut group = c.benchmark_group("zst");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("push_zst", size), size, |b, &size| {
            b.iter(|| {
                let mut v: ThinVec<()> = ThinVec::new();
                for _ in 0..size {
                    v.push(());
                }
                black_box(v)
            })
        });

        group.bench_with_input(BenchmarkId::new("vec_push_zst", size), size, |b, &size| {
            b.iter(|| {
                let mut v: Vec<()> = Vec::new();
                for _ in 0..size {
                    v.push(());
                }
                black_box(v)
            })
        });

        group.bench_with_input(BenchmarkId::new("iter_zst", size), size, |b, &size| {
            let v: ThinVec<()> = (0..size).map(|_| ()).collect();
            b.iter(|| {
                let count = v.iter().count();
                black_box(count)
            })
        });

        group.bench_with_input(BenchmarkId::new("vec_iter_zst", size), size, |b, &size| {
            let v: Vec<()> = (0..size).map(|_| ()).collect();
            b.iter(|| {
                let count = v.iter().count();
                black_box(count)
            })
        });
    }

    group.finish();
}

// ============================================================================
// Conversion Benchmarks
// ============================================================================

fn bench_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");

    for size in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("from_vec_to_thinvec", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size as u64).collect::<Vec<u64>>(),
                    |v| {
                        let tv: ThinVec<u64> = ThinVec::from(v);
                        black_box(tv)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("from_thinvec_to_vec", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || (0..size as u64).collect::<ThinVec<u64>>(),
                    |tv| {
                        let v: Vec<u64> = Vec::from(tv);
                        black_box(v)
                    },
                    criterion::BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(BenchmarkId::new("from_slice", size), size, |b, &size| {
            let data: Vec<u64> = (0..size as u64).collect();
            b.iter(|| {
                let tv: ThinVec<u64> = ThinVec::from(&data[..]);
                black_box(tv)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("vec_from_slice", size),
            size,
            |b, &size| {
                let data: Vec<u64> = (0..size as u64).collect();
                b.iter(|| {
                    let v: Vec<u64> = Vec::from(&data[..]);
                    black_box(v)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_construction,
    bench_push,
    bench_pop,
    bench_insert,
    bench_remove,
    bench_iteration,
    bench_clone,
    bench_extend,
    bench_drain,
    bench_splice,
    bench_truncate_clear,
    bench_reserve_shrink,
    bench_retain,
    bench_dedup,
    bench_split_append,
    bench_memory_footprint,
    bench_zst,
    bench_conversions,
);

criterion_main!(benches);
