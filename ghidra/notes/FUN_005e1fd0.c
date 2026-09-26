
void __fastcall FUN_005e1fd0(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654ebd;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0x1a,9,0x61,9,1,0x10);
  }
  *(void **)(param_1 + 0xa4) = pvVar1;
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xa4),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 1;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0x14,0x14,0x66,9,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xa8) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xa8),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,0x14,0x1f,0x66,9,0,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xac) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xac),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 3;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,-100,-100,0x66,9,1,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb0) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xb0),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 4;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,-100,-100,0x66,9,1,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb4) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xb4),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 5;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,-100,-100,0x66,9,1,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xb8) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xb8),0x2ffffff,(HWND)0x0);
  }
  pvVar1 = (void *)FUN_00618b70(0x34);
  local_4 = 6;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_00601880(pvVar1,(char *)&lpString_006a7f40,-100,-100,0x66,9,1,0x10);
  }
  local_4 = 0xffffffff;
  *(void **)(param_1 + 0xbc) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_00601c60(pvVar1,8);
    FUN_00601c90(*(void **)(param_1 + 0xbc),0x2ffffff,(HWND)0x0);
  }
  ExceptionList = local_c;
  return;
}

