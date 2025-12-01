thread 'main' (39258) panicked at package/project/mahjong/src/plugin/scene/in_game/mod.rs:590:57:                                                                                                                                                                                                                                                                                                                                                                              22:36:46 [8/1745]
called `Result::unwrap()` on an `Err` value: QueryDoesNotMatch(106v0, ArchetypeId(36))
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/ed61e7d7e242494fb7057f2657300d9e77bb4fcb/library/std/src/panicking.rs:698:5
   1: core::panicking::panic_fmt
             at /rustc/ed61e7d7e242494fb7057f2657300d9e77bb4fcb/library/core/src/panicking.rs:75:14
   2: core::result::unwrap_failed
             at /rustc/ed61e7d7e242494fb7057f2657300d9e77bb4fcb/library/core/src/result.rs:1855:5
   3: core::result::Result<T,E>::unwrap
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1226:23
   4: mahjong::plugin::scene::in_game::on_click
   5: core::ops::function::FnMut::call_mut
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:166:5
   6: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:21
   7: <Func as bevy_ecs::system::function_system::SystemParamFunction<(bevy_ecs::system::function_system::HasSystemInput,fn(In,F0,F1,F2,F3,F4,F5) .> Out)>>::run::call_inner
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:961:21
   8: <Func as bevy_ecs::system::function_system::SystemParamFunction<(bevy_ecs::system::function_system::HasSystemInput,fn(In,F0,F1,F2,F3,F4,F5) .> Out)>>::run
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:964:17
   9: <bevy_ecs::system::function_system::FunctionSystem<Marker,Out,F> as bevy_ecs::system::system::System>::run_unsafe
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/function_system.rs:711:29
  10: bevy_ecs::observer::runner::observer_system_runner::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/observer/runner.rs:103:38
  11: core::result::Result<T,E>::and_then
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1486:22
  12: bevy_ecs::observer::runner::observer_system_runner
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/observer/runner.rs:103:14
  13: bevy_ecs::event::trigger::trigger_entity_internal
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/event/trigger.rs:211:17
  14: <bevy_ecs::event::trigger::PropagateEntityTrigger<_,E,T> as bevy_ecs::event::trigger::Trigger<E>>::trigger
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/event/trigger.rs:289:13
  15: bevy_ecs::world::deferred_world::DeferredWorld::trigger_raw
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/deferred_world.rs:816:21
  16: bevy_ecs::observer::<impl bevy_ecs::world::World>::trigger_ref_with_caller
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/observer/mod.rs:122:39
  17: bevy_ecs::system::commands::command::trigger::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/command.rs:218:15
  18: core::ops::function::FnOnce::call_once
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  19: <F as bevy_ecs::system::commands::command::Command<Out>>::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/command.rs:62:9
  20: bevy_ecs::world::command_queue::RawCommandQueue::push::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs:194:33
  21: core::ops::function::FnOnce::call_once
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  22: bevy_ecs::world::command_queue::RawCommandQueue::apply_or_drop_queued::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs:284:26
  23: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic/unwind_safe.rs:274:9
  24: std::panicking::catch_unwind::do_call
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:590:40
  25: std::panicking::catch_unwind
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:553:19
  26: std::panic::catch_unwind
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:359:14
  27: bevy_ecs::world::command_queue::RawCommandQueue::apply_or_drop_queued
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs:289:30
  28: bevy_ecs::world::command_queue::CommandQueue::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs:115:28
  29: <bevy_ecs::world::command_queue::CommandQueue as bevy_ecs::system::system_param::SystemBuffer>::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/command_queue.rs:350:14
  30: <bevy_ecs::system::system_param::Deferred<T> as bevy_ecs::system::system_param::SystemParam>::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:1300:21
  31: <(P0,P1) as bevy_ecs::system::system_param::SystemParam>::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs:2177:1
  32: bevy_ecs::system::commands::_::<impl bevy_ecs::system::system_param::SystemParam for bevy_ecs::system::commands::Commands>::apply
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/commands/mod.rs:152:13
  33: bevy_ecs::schedule::executor::multi_threaded::apply_deferred::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/executor/multi_threaded.rs:809:20
  34: core::ops::function::FnOnce::call_once
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  35: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panic/unwind_safe.rs:274:9
  36: std::panicking::catch_unwind::do_call
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:590:40
  37: std::panicking::catch_unwind
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:553:19
  38: std::panic::catch_unwind
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:359:14
  39: bevy_ecs::schedule::executor::multi_threaded::apply_deferred
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/executor/multi_threaded.rs:808:19
  40: <bevy_ecs::schedule::executor::multi_threaded::MultiThreadedExecutor as bevy_ecs::schedule::executor::SystemExecutor>::run
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/executor/multi_threaded.rs:302:23
  41: bevy_ecs::schedule::schedule::Schedule::run
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/schedule/schedule.rs:493:14
  42: bevy_ecs::world::World::try_run_schedule::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:3600:61
  43: bevy_ecs::world::World::try_schedule_scope
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:3533:21
  44: bevy_ecs::world::World::try_run_schedule
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:3600:14
  45: bevy_app::main_schedule::Main::run_main::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/main_schedule.rs:296:31
  46: bevy_ecs::world::World::try_resource_scope
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:2626:22
  47: bevy_ecs::world::World::resource_scope
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:2589:14
  48: bevy_app::main_schedule::Main::run_main
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.17.3/src/main_schedule.rs:294:15
  49: core::ops::function::FnMut::call_mut
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:166:5
  50: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
             at /home/marcus/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:298:21
  51: <Func as bevy_ecs::system::exclusive_function_system::ExclusiveSystemParamFunction<fn(F0) .> Out>>::run::call_inner
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/exclusive_function_system.rs:270:21
  52: <Func as bevy_ecs::system::exclusive_function_system::ExclusiveSystemParamFunction<fn(F0) .> Out>>::run
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/exclusive_function_system.rs:273:17
  53: <bevy_ecs::system::exclusive_function_system::ExclusiveFunctionSystem<Marker,Out,F> as bevy_ecs::system::system::System>::run_unsafe::{{closure}}
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/exclusive_function_system.rs:135:33
  54: bevy_ecs::world::World::last_change_tick_scope
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/world/mod.rs:3009:9
  55: <bevy_ecs::system::exclusive_function_system::ExclusiveFunctionSystem<Marker,Out,F> as bevy_ecs::system::system::System>::run_unsafe
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/exclusive_function_system.rs:113:15
  56: bevy_ecs::system::system::System::run_without_applying_deferred
             at /home/marcus/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system.rs:139:23
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Encountered a panic when applying buffers for system `bevy_picking::events::pointer_events`!
Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
