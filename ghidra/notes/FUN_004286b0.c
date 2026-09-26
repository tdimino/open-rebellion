
void __thiscall FUN_004286b0(void *this,undefined4 param_1)

{
  int iVar1;
  
  FUN_006037f0(7);
  DAT_006be5ac = param_1;
  if (*(void **)((int)this + 0x134) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x134),0);
    FUN_00602840(*(void **)((int)this + 0x134),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x134) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x134),0x1501);
  }
  if (*(void **)((int)this + 0x138) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x138),0);
    FUN_00602840(*(void **)((int)this + 0x138),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x138) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x138),0x1502);
  }
  if (*(void **)((int)this + 0x13c) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x13c),0);
    FUN_00602840(*(void **)((int)this + 0x13c),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x13c) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x13c),0x1503);
  }
  if (*(void **)((int)this + 0x140) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x140),0);
    FUN_00602840(*(void **)((int)this + 0x140),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x140) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x140),0x1504);
  }
  if (*(void **)((int)this + 0x144) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x144),0);
    FUN_00602840(*(void **)((int)this + 0x144),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x144) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x144),0x1505);
  }
  if (*(void **)((int)this + 0x148) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x148),0);
    FUN_00602840(*(void **)((int)this + 0x148),600);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x148) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x148),0x1506);
  }
  if (*(void **)((int)this + 0x14c) != (void *)0x0) {
    FUN_005ffce0(*(void **)((int)this + 0x14c),0);
    FUN_00602840(*(void **)((int)this + 0x14c),0x260);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x138) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x14c),0x1500);
  }
  iVar1 = *(int *)((int)this + 0x248);
  if (iVar1 != 0) {
    *(uint *)(iVar1 + 0x50) = *(uint *)(iVar1 + 0x50) & 0xefffffff;
    FUN_005ffce0(*(void **)((int)this + 0x248),0);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x248) + 0x18),(HWND)0x1,0,0,0,0,3);
    FUN_006009f0(this,*(void **)((int)this + 0x248),0x173c);
  }
  return;
}

