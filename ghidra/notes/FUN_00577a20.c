
undefined4 __fastcall FUN_00577a20(int param_1)

{
  undefined4 uVar1;
  int iVar2;
  void *local_40;
  undefined4 local_3c;
  undefined4 local_38;
  int local_34;
  uint local_30 [9];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064cdb8;
  local_c = ExceptionList;
  local_3c = 0;
  ExceptionList = &local_c;
  FUN_004fd450(local_30 + 2,param_1 + 0x20);
  local_40 = (void *)0x0;
  local_38 = 1;
  local_34 = -1;
  local_30[0] = 1;
  local_30[1] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar2 = FUN_00506bf0((uint *)(param_1 + 0x3c),&local_40,local_30,&local_38);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  if ((iVar2 != 0) && (local_34 == -1)) {
    FUN_0052fbf0(local_40,&local_3c,local_30 + 2);
  }
  uVar1 = local_3c;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_30 + 2);
  ExceptionList = local_c;
  return uVar1;
}

