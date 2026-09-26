
uint __thiscall FUN_0053c810(void *this,void *param_1,int *param_2,void *param_3)

{
  int *piVar1;
  undefined1 uVar2;
  short sVar3;
  undefined4 *puVar4;
  undefined4 uVar5;
  int local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006464ef;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = param_2;
  *param_2 = 0;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  param_2 = (int *)FUN_0053cbc0(param_1,local_1c,param_3);
  uVar2 = (undefined1)local_4;
  if ((param_2 != (int *)0x0) && (*(int *)((int)param_3 + 4) == -1)) {
    sVar3 = FUN_005f50e0((int)local_1c);
    uVar2 = (undefined1)local_4;
    if (sVar3 != 0) {
      param_2 = (int *)FUN_00618b70(0x60);
      if (param_2 == (int *)0x0) {
        FUN_00619730();
      }
      local_4._0_1_ = 2;
      if (param_2 == (int *)0x0) {
        puVar4 = (undefined4 *)0x0;
      }
      else {
        puVar4 = FUN_004f5440(param_2);
      }
      local_4._0_1_ = 1;
      uVar2 = (undefined1)local_4;
      local_4._0_1_ = 1;
      param_2 = (int *)(uint)(puVar4 != (undefined4 *)0x0);
      *piVar1 = (int)puVar4;
      if (param_2 != (int *)0x0) {
        FUN_005f5600(puVar4,*(undefined4 *)(*(int *)((int)this + 0x44) + 8));
        *(undefined4 *)(*piVar1 + 0x1c) = *(undefined4 *)((int)this + 0x1c);
        *(undefined4 *)(*piVar1 + 0x20) = *(undefined4 *)((int)this + 0x20);
        *(undefined4 *)(*piVar1 + 0x44) = 0;
        *(undefined4 *)(*piVar1 + 0x48) = 0;
        uVar5 = FUN_0053ca30(this,&param_1);
        local_4._0_1_ = 3;
        (**(code **)(*(int *)*piVar1 + 0x2c))(uVar5);
        puStack_8._0_1_ = 1;
        FUN_00619730();
        puStack_8._0_1_ = 4;
        FUN_004f26d0((void *)(*piVar1 + 0x4c),(undefined4 *)&stack0x00000000);
        puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
        FUN_00619730();
        (**(code **)(*(int *)*piVar1 + 0x24))(&stack0xffffffe0);
        puVar4 = FUN_004f4340(&local_14);
        local_4._0_1_ = 5;
        FUN_004f43b0((void *)(*piVar1 + 0x58),puVar4);
        local_4._0_1_ = 1;
        FUN_004f4380(&local_14);
        uVar2 = (undefined1)local_4;
      }
    }
  }
  local_4._0_1_ = uVar2;
  if (param_2 == (int *)0x0) {
    local_14 = 1;
    local_10 = 1;
    local_4._0_1_ = 6;
    FUN_00520580(param_3,&local_14);
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  if ((*(int *)((int)param_3 + 4) != -1) && ((undefined4 *)*piVar1 != (undefined4 *)0x0)) {
    (*(code *)**(undefined4 **)*piVar1)(1);
    *piVar1 = 0;
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_1c);
  ExceptionList = pvStack_c;
  return (uint)param_2;
}

