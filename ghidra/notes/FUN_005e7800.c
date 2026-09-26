
void __fastcall FUN_005e7800(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006551bc;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0x1a,7,0x61,9,0,0x10);
  }
  *(void **)(param_1 + 0xa8) = pvVar1;
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xa8),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0x1a,0x34,0x61,9,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xac) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xac),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0xf,0xb6,0x79,9,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb0) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xb0),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 3;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0xf,0xc1,0x79,9,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb4) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xb4),0x2ffffff,(HWND)0x0);
  }
  ExceptionList = local_c;
  return;
}

