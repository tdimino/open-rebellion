
void __thiscall FUN_00426ee0(void *this,undefined4 param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c0f6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x294) = 0x27a5;
  *(undefined4 *)((int)this + 0x298) = 0x27a4;
  *(undefined4 *)((int)this + 0x29c) = 0x27a3;
  *(undefined4 *)((int)this + 0x2a0) = 0x27a2;
  *(undefined4 *)((int)this + 0x2a4) = 0x27a6;
  *(undefined4 *)((int)this + 0x2a8) = 0x27a7;
  *(undefined4 *)((int)this + 0x2ac) = 0x27a8;
  *(undefined4 *)((int)this + 0x2b0) = 0x27a9;
  *(undefined4 *)((int)this + 0x2b4) = 0x27aa;
  *(undefined4 *)((int)this + 0x2b8) = 0x27ab;
  *(undefined4 *)((int)this + 700) = 0x27ac;
  *(undefined4 *)((int)this + 0x2c0) = 0x27ad;
  if (*(int *)((int)this + 0x2c4) == 0) {
    pvVar1 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005fbd20(pvVar1,param_1,0x27ae,10);
    }
    *(void **)((int)this + 0x2c4) = pvVar1;
  }
  local_4 = 0xffffffff;
  if (*(int *)((int)this + 0x2c8) == 0) {
    pvVar1 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_005fbd20(pvVar1,param_1,0x27b6,10);
    }
    *(void **)((int)this + 0x2c8) = pvVar1;
  }
  ExceptionList = local_c;
  return;
}

