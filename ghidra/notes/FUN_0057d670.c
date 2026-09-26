
undefined4 __fastcall FUN_0057d670(int *param_1)

{
  uint uVar1;
  int iVar2;
  int *unaff_EBX;
  undefined4 local_3c;
  int local_38 [10];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064e088;
  pvStack_c = ExceptionList;
  local_38[0] = 1;
  local_38[1] = 0xffffffff;
  local_4 = 0;
  local_3c = 0;
  ExceptionList = &pvStack_c;
  uVar1 = (**(code **)(*param_1 + 0x1c))(local_38);
  if ((uVar1 != 0) && (local_38[0] == -1)) {
    local_38[1] = 1;
    local_38[2] = 0x12;
    puStack_8._0_1_ = 1;
    uVar1 = FUN_00505230((uint *)(param_1 + 0xf),(undefined4 *)&stack0xffffffc0,
                         (uint *)(local_38 + 1),&local_3c);
    puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
    FUN_00619730();
  }
  if ((uVar1 != 0) && (local_38[0] == -1)) {
    iVar2 = thunk_FUN_00506e80();
    uVar1 = (uint)(iVar2 != 0);
  }
  if ((uVar1 != 0) && (local_38[0] == -1)) {
    FUN_004fd450(local_38 + 3,(int)(param_1 + 8));
    puStack_8._0_1_ = 2;
    FUN_00561990(unaff_EBX,param_1[0x14],local_38 + 3);
    puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
    FUN_004fd4d0(local_38 + 3);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return 0;
}

