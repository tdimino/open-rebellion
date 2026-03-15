
void __fastcall FUN_0047b950(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063378b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0x7fffffff;
  pvVar1 = FUN_004f5940((void *)(param_1 + 0x4c),(uint *)(param_1 + 0x28));
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (pvVar1 == (void *)0x0) {
      pvVar1 = (void *)0x0;
    }
    else {
      pvVar1 = FUN_004ecc70(pvVar1,(uint *)(param_1 + 0x28));
    }
    local_4 = 0xffffffff;
    if (pvVar1 != (void *)0x0) {
      FUN_004f57b0((void *)(param_1 + 0x4c),pvVar1);
    }
  }
  ExceptionList = local_c;
  return;
}

