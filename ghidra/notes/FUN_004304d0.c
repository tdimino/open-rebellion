
undefined4 __fastcall FUN_004304d0(int *param_1)

{
  uint *puVar1;
  void *pvVar2;
  undefined4 *this;
  uint uVar3;
  int iVar4;
  int *piVar5;
  undefined4 uVar6;
  uint local_18;
  void *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cc83;
  local_c = ExceptionList;
  uVar6 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  if (param_1[4] == 0) {
    (**(code **)(*param_1 + 4))();
    piVar5 = FUN_004f2db0(param_1[2]);
    param_1[4] = (int)piVar5;
  }
  else {
    pvVar2 = *(void **)(param_1[4] + 0x1c);
    if (pvVar2 != (void *)0x0) {
      puVar1 = FUN_004025b0(pvVar2,(uint *)&local_14);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_18,puVar1);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    puVar1 = FUN_004025b0((void *)param_1[4],(uint *)&local_14);
    local_4._0_1_ = 2;
    pvVar2 = FUN_004f5940(param_1,puVar1);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar2 == (void *)0x0) {
      local_14 = (void *)FUN_00618b70(0xbc);
      local_4._0_1_ = 3;
      if (local_14 == (void *)0x0) {
        this = (undefined4 *)0x0;
      }
      else {
        this = FUN_00484230(local_14,(void *)param_1[4],param_1[2],param_1[5],(void *)param_1[6],
                            param_1[7]);
      }
      local_4._0_1_ = 0;
      if (this != (undefined4 *)0x0) {
        uVar3 = FUN_00430450(param_1,this);
        if (uVar3 == 0) {
          (**(code **)*this)(1);
        }
        else {
          FUN_00484570((int)this);
          puVar1 = FUN_00403040(this,&local_14);
          local_4._0_1_ = 4;
          FUN_00430120((void *)param_1[5],&local_18,puVar1);
          local_4._0_1_ = 0;
          FUN_00619730();
        }
      }
    }
    local_14 = (void *)0x14;
    local_10 = 0x1c;
    local_4._0_1_ = 5;
    iVar4 = FUN_004f6010((void *)param_1[4],(uint *)&local_14,1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    param_1[4] = iVar4;
    if (iVar4 == 0) {
      uVar6 = 1;
      param_1[3] = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar6;
}

