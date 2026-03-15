
int __fastcall FUN_00566690(int *param_1)

{
  int iVar1;
  int unaff_EBX;
  int iVar2;
  undefined4 local_20;
  int local_1c [3];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064b000;
  pvStack_c = ExceptionList;
  local_1c[0] = 1;
  local_1c[1] = 0xffffffff;
  iVar2 = 0;
  local_4 = 0;
  local_20 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*param_1 + 0x1c))(local_1c);
  if (iVar1 != 0) {
    if (local_1c[0] == -1) {
      local_1c[1] = 1;
      local_1c[2] = 0x12;
      puStack_8._0_1_ = 1;
      iVar1 = FUN_00506d00((uint *)(param_1 + 0xf),(undefined4 *)&stack0xffffffdc,
                           (uint *)(local_1c + 1),&local_20);
      puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
      FUN_00619730();
    }
    if ((iVar1 != 0) && (local_1c[0] == -1)) {
      iVar2 = unaff_EBX + 0x58;
    }
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return iVar2;
}

