
void __thiscall FUN_00486b60(void *this,uint *param_1,uint *param_2,uint *param_3)

{
  uint *puVar1;
  int *piVar2;
  uint *puVar3;
  uint *puVar4;
  void *this_00;
  uint local_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006349b0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_004ece30(&local_14);
  puVar4 = param_2;
  local_4._0_1_ = 1;
  puVar1 = FUN_004ece40(param_2);
  if (puVar1 != (uint *)0x0) {
    piVar2 = FUN_004f3000(*(int *)((int)this + 0x24),puVar4);
    if (piVar2 != (int *)0x0) {
      puVar1 = FUN_004025b0(piVar2,(uint *)&param_2);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_14,puVar1);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
  }
  FUN_004ece30(&local_10);
  puVar1 = param_3;
  local_4._0_1_ = 3;
  puVar3 = FUN_004ece40(param_3);
  if (puVar3 != (uint *)0x0) {
    piVar2 = FUN_004f3000(*(int *)((int)this + 0x24),puVar1);
    if (piVar2 != (int *)0x0) {
      puVar1 = FUN_004025b0(piVar2,(uint *)&param_2);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_10,puVar1);
      local_4._0_1_ = 3;
      FUN_00619730();
    }
  }
  if (local_14 != local_10) {
    puVar1 = FUN_004ece40(&local_14);
    if (puVar1 != (uint *)0x0) {
      FUN_004f26d0(&local_18,puVar4);
      FUN_0041d4d0(param_1,(int *)0x0,&local_18,(int *)0x0);
    }
    piVar2 = FUN_004f3110(*(int *)((int)this + 0x24),param_1);
    this_00 = (void *)0x0;
    if (piVar2 != (int *)0x0) {
      this_00 = (void *)piVar2[7];
    }
    if (this_00 != (void *)0x0) {
      puVar4 = FUN_004025b0(this_00,(uint *)&param_2);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_18,puVar4);
      local_4._0_1_ = 3;
      FUN_00619730();
    }
    FUN_0041d4d0(param_1,(int *)0x0,&local_18,(int *)0x0);
    FUN_0041d4b0(param_1,(int *)0x0);
  }
  FUN_0049cdb0(param_1);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

