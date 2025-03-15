use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128-pf200:64:64:64:32-A200-P200-G200".into(),
        llvm_target: "riscv32cheriot".into(),
        metadata: TargetMetadata {
            description: Some("CHERIoT RISC-V (RV32EMC base ISA CHERIoT extension)".into()),
            tier: None,
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        arch: "riscv32".into(),

        options: TargetOptions {
            abi: "cheriot-baremetal".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "cheriot".into(),
            max_atomic_width: Some(0),
            atomic_cas: false,
            features: "+e,+m,+c,+forced-atomics,+xcheri,+cap-mode".into(),
            llvm_abiname: "cheriot-baremetal".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            address_width: Some(32),
            ..Default::default()
        },
    }
}
