
undefined4 __fastcall FUN_004032c0(int *param_1)

{
  uint *puVar1;
  void *pvVar2;
  undefined4 *this;
  int iVar3;
  int *piVar4;
  uint uVar5;
  uint local_1c;
  undefined4 local_18;
  void *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062948b;
  local_c = ExceptionList;
  uVar5 = 0;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_005f4950(&local_1c,0);
  local_4 = 0;
  if ((void *)param_1[5] == (void *)0x0) {
    (**(code **)(*param_1 + 4))();
    piVar4 = FUN_004f30a0(param_1[2]);
    param_1[5] = (int)piVar4;
  }
  else {
    puVar1 = FUN_004025b0((void *)param_1[5],(uint *)&local_14);
    local_4._0_1_ = 1;
    pvVar2 = FUN_004f5940(param_1,puVar1);
    local_4._0_1_ = 0;
    FUN_00619730();
    if (pvVar2 == (void *)0x0) {
      local_14 = (void *)FUN_00618b70(0x68);
      local_4._0_1_ = 2;
      if (local_14 == (void *)0x0) {
        this = (undefined4 *)0x0;
      }
      else {
        this = FUN_00401d20(local_14,(undefined1 *)param_1[5],param_1[2],param_1[4]);
      }
      local_4._0_1_ = 0;
      if (this != (undefined4 *)0x0) {
        puVar1 = FUN_00403040(this,&local_14);
        local_4._0_1_ = 3;
        pvVar2 = FUN_004f5940(param_1,puVar1);
        local_4._0_1_ = 0;
        FUN_00619730();
        if (pvVar2 == (void *)0x0) {
          uVar5 = FUN_004f57b0(param_1,this);
          FUN_004021b0((int)this);
        }
        if (uVar5 == 0) {
          (**(code **)*this)(1);
        }
        else {
          FUN_004021b0((int)this);
          local_1c = this[0xc];
          if ((((local_1c & 2) != 0) && ((local_1c & 0x20000000) != 0)) && ((local_1c & 8) == 0)) {
            param_1[6] = param_1[6] + -1;
          }
        }
      }
    }
    local_14 = (void *)0x30;
    local_10 = 0x40;
    local_4._0_1_ = 4;
    iVar3 = FUN_004f6010((void *)param_1[5],(uint *)&local_14,1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    param_1[5] = iVar3;
    if (iVar3 == 0) {
      local_18 = 1;
      param_1[3] = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_18;
}

