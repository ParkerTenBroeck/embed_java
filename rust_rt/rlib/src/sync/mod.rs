use spin::RelaxStrategy;


pub type Mutex<T> = spin::mutex::Mutex<T, VmRelax>;
pub type MutexGuard<'a, T> = spin::mutex::MutexGuard<'a, T>;

pub type SpinMutex<T> = spin::mutex::SpinMutex<T, VmRelax>;
pub type SpinMutexGuard<'a, T> = spin::mutex::SpinMutexGuard<'a, T>;

pub type RwLock<T> = spin::rwlock::RwLock<T, VmRelax>;
pub type RwLockReadGuard<'a, T> = spin::rwlock::RwLockReadGuard<'a, T>;
pub type RwLockUpgradableGuard<'a, T> = spin::rwlock::RwLockUpgradableGuard<'a, T>;
pub type RwLockWriteGuard<'a, T> = spin::rwlock::RwLockWriteGuard<'a, T>;

pub type Lazy<T> = spin::Lazy<T, VmRelax>;
pub type Once<T> = spin::Once<T>;
pub type Barrier = spin::barrier::Barrier<VmRelax>;
pub type BarrierWaitResult = spin::barrier::BarrierWaitResult;

pub struct VmRelax;

impl RelaxStrategy for VmRelax{
    #[inline(always)]
    fn relax() {
        crate::arch::sleep_ms(1);
    }
}

