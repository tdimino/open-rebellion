
void __fastcall FUN_005f1050(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655516;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601700(pvVar1,&DAT_006bdac0,0x14,0x1c,0,0,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0x9c) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c90(pvVar1,0x2ffffff,(HWND)0x0);
    FUN_00601c60(*(void **)(param_1 + 0x9c),10);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601700(pvVar1,&DAT_006bdb38,0xe,0x5c,0,0,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xa0) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c90(pvVar1,0x2ffffff,(HWND)0x0);
    FUN_00601c60(*(void **)(param_1 + 0xa0),10);
  }
  ExceptionList = local_c;
  return;
}

