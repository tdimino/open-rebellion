
undefined4 __fastcall FUN_00540f40(int param_1)

{
  int iVar1;
  int *local_3c;
  undefined4 local_38;
  int local_34;
  uint local_30 [9];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00646c58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(local_30 + 2,param_1 + 0x20);
  local_3c = (int *)0x0;
  local_38 = 1;
  local_34 = -1;
  local_30[0] = 1;
  local_30[1] = 0x12;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  iVar1 = FUN_00505de0((uint *)(param_1 + 0x3c),&local_3c,local_30,&local_38);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  if ((iVar1 != 0) && (local_34 == -1)) {
    (**(code **)(*local_3c + 0x200))(param_1 + 0x40,local_30 + 2);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_30 + 2);
  ExceptionList = local_c;
  return 0;
}

