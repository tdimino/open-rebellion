
uint __fastcall FUN_00539b60(int *param_1)

{
  uint uVar1;
  undefined3 extraout_var;
  uint uVar2;
  bool bVar3;
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006461b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar1 = thunk_FUN_004f9700(param_1);
  FUN_004fcd00(local_2c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while ((local_10 != 0 && (bVar3 = uVar1 != 0, uVar1 = 0, bVar3))) {
    FUN_005130d0((int)local_2c);
    uVar1 = 0;
  }
  local_30 = 0;
  uVar2 = 0;
  if (uVar1 != 0) {
    bVar3 = FUN_004f6b50(param_1,&local_30);
    uVar2 = CONCAT31(extraout_var,bVar3);
    if ((uVar2 != 0) && (((param_1[0x14] & 8U) == 0 || ((param_1[0x14] & 0x4000U) == 0)))) {
      uVar2 = ~*(uint *)(local_30 + 0x50) >> 3 & 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar2;
}

