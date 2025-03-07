use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128-pf200:64:64:64:32-A200-P200-G200".into(),
        llvm_target: "riscv32".into(),
        metadata: TargetMetadata {
            description: Some("CHERIoT RISC-V (RV32EMC base ISA with CHERIoT extension)".into()),
            tier: None,
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            abi: "ilp32e".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "cheriot".into(),
            max_atomic_width: Some(32),
            atomic_cas: false,
            features: "+e,+m,+c,+forced-atomics".into(),
            llvm_abiname: "ilp32e".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            pointer_extension_width: Some(32),
            ..Default::default()
        },
    }
}
