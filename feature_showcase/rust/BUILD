load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "kyron_example",
    srcs = glob(["kyron/**/*.rs"]),
    deps = [
        "@score_kyron//src/kyron:libkyron",
        "@score_kyron//src/kyron-foundation:libkyron_foundation",
    ],
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "orch_per_example",
    srcs = glob(["orchestration_persistency/**/*.rs"]),
    deps = [
        "@score_kyron//src/kyron:libkyron",
        "@score_kyron//src/kyron-foundation:libkyron_foundation",
    ],
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "hello_score_example",
    srcs = glob(["hello_score/**/*.rs"]),
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "car_system_example",
    srcs = glob(["car_system/**/*.rs"]),
    visibility = ["//visibility:public"],
)
