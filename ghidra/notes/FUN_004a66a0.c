
byte FUN_004a66a0(int *param_1)

{
  uint uVar1;
  int iVar2;
  byte bVar3;
  undefined1 local_6c [28];
  undefined4 uStack_50;
  undefined1 local_4c [28];
  undefined4 uStack_30;
  undefined1 local_2c [28];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637e08;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005039d0(local_2c,param_1,3);
  local_4 = 0;
  FUN_00504c40(local_4c,param_1,3);
  local_4._0_1_ = 1;
  FUN_00536da0(local_6c,param_1,3);
  local_4 = CONCAT31(local_4._1_3_,2);
  iVar2 = FUN_00513120((int)local_2c);
  bVar3 = *(int *)(iVar2 + 0x1c) != 0;
  iVar2 = FUN_00513120((int)local_4c);
  if (*(int *)(iVar2 + 0x1c) != 0) {
    bVar3 = bVar3 | 2;
  }
  iVar2 = FUN_00513120((int)local_6c);
  if (*(int *)(iVar2 + 0x1c) != 0) {
    bVar3 = bVar3 | 4;
  }
  uVar1 = param_1[0x14];
  if ((uVar1 & 0x10) != 0) {
    bVar3 = bVar3 | 8;
  }
  if ((uVar1 & 0x200) != 0) {
    bVar3 = bVar3 | 0x10;
  }
  if ((uVar1 & 4) == 0) {
    bVar3 = bVar3 | 0x20;
  }
  iVar2 = (**(code **)(*param_1 + 0x34))(1);
  if (iVar2 == 0) {
    bVar3 = bVar3 | 0x40;
  }
  puStack_8._0_1_ = 1;
  FUN_00536ea0((undefined4 *)&stack0xffffff90);
  puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
  FUN_00504d40(&uStack_50);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00503ad0(&uStack_30);
  ExceptionList = pvStack_10;
  return bVar3;
}

