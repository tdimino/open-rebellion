
uint __fastcall FUN_0042f350(void *param_1)

{
  void *pvVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ca8b;
  local_c = ExceptionList;
  uVar2 = 0;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0xe8);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_004762b0(pvVar1,*(undefined4 *)((int)param_1 + 0xc),
                          *(undefined4 *)((int)param_1 + 0x10));
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    uVar2 = FUN_005f5440(param_1,pvVar1);
  }
  ExceptionList = local_c;
  return uVar2;
}

